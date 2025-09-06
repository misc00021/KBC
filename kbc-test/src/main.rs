mod kbc_by_num_replacing;
use core::time;
use std::io::{BufRead, BufReader};
use std::time::Duration;
mod kbc_by_num_extending;
mod kbc_by_num_replacing_no_div;
mod kbc_by_num_replacing_sep_div;
mod kbc_by_num_replacing_true_sep_div;
mod kbc_by_term_size_replacing;
mod math;
use serde_derive::Serialize;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

use egg::{EGraph, RecExpr, Runner, SimpleScheduler};
use kbc_by_num_extending::rules as by_num_extending;
use kbc_by_num_replacing::rules as by_num_replacing;
use kbc_by_num_replacing_no_div::rules as by_num_replacing_no_div;
use kbc_by_num_replacing_sep_div::rules as by_num_replacing_sep_div;
use kbc_by_num_replacing_true_sep_div::rules as by_num_replacing_true_sep_div;
use kbc_by_term_size_replacing::rules as by_term_size_replacing;
use math::{ConstantFold, Math};

fn load_exprs_from_file(path: &str) -> std::io::Result<Vec<RecExpr<Math>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut exprs = Vec::new();

    for line in reader.lines() {
        let line = line?; // unwrap IO result
        if line.trim().is_empty() {
            continue; // skip blank lines
        }
        match line.parse::<RecExpr<Math>>() {
            Ok(expr) => exprs.push(expr),
            Err(e) => eprintln!("Failed to parse '{}': {}", line, e),
        }
    }
    Ok(exprs)
}

#[derive(Serialize)]
struct TestResult<'a> {
    original_term: &'a str,
    simplified_term: &'a str,
    input_weight_simple: i32,
    output_weight_simple: i32,
    input_weight_complex: i32,
    output_weight_complex: i32,
    input_depth: usize,
    output_depth: usize,
    simplification_time: f64,
    apply_time: f64,
    search_time: f64,
    rebuild_time: f64,
    stop_reason: String,
    iterations: usize,
}

fn get_cost(expr: &RecExpr<Math>) -> i32 {
    expr.as_ref()
        .iter()
        .map(|node| match node {
            Math::Constant(_) => 1,
            Math::Symbol(_) => 1,
            Math::Add(_) => 1,
            Math::Sub(_) => 1,
            Math::Mul(_) => 1,
            Math::Div(_) => 1,
            Math::Pow(_) => 1,
            _ => 1,
        })
        .sum()
}

fn get_cost_complex(expr: &RecExpr<Math>) -> i32 {
    expr.as_ref()
        .iter()
        .map(|node| match node {
            Math::Constant(_) => 1,
            Math::Symbol(_) => 2,
            Math::Add(_) => 1,
            Math::Sub(_) => 1,
            Math::Mul(_) => 2,
            Math::Div(_) => 5,
            Math::Pow(_) => 4,
            _ => 1,
        })
        .sum()
}

fn get_depth(expr: &RecExpr<Math>) -> usize {
    let str = expr.to_string();
    let mut max_depth = 1;
    let mut current_depth = 1;
    for c in str.chars() {
        if c == '(' {
            current_depth += 1;
            if current_depth > max_depth {
                max_depth = current_depth;
            }
        } else if c == ')' {
            current_depth -= 1;
        }
    }
    max_depth
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let input_file;
    match args.len() {
        2 => {
            input_file = &args[1];
        }
        _ => {
            eprintln!("Usage: {} <input_file>", args[0]);
            std::process::exit(1);
        }
    }
    // println!("Loading expressions from {}", input_file);
    let exprs = load_exprs_from_file(&input_file)?;
    // println!("Loaded {} expressions from {}", exprs.len(), input_file);
    let rule_files = vec![
        ("by_num_replacing_sep_div", by_num_replacing_sep_div()),
        (
            "by_num_replacing_true_sep_div",
            by_num_replacing_true_sep_div(),
        ),
    ];
    for time_limit in vec![
        0.0001, 0.0005, 0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1., 2., /* 3., 4., 5.*/
    ] {
        let parent_dir =
            std::path::Path::new("with_timelimits").join(format!("timelimit{}", time_limit));
        for (dir_name, rules) in &rule_files {
            let out_dir = parent_dir.join(dir_name).join(
                input_file
                    .split('/')
                    .last()
                    .unwrap()
                    .split('.')
                    .next()
                    .unwrap(),
            );
            std::fs::create_dir_all(&out_dir)?;
            // println!("Writing results to {:?}", out_dir);
            for (name, rules) in rules {
                let file_path = out_dir.join(format!(
                    "EqSat_{}_{}.jsonl",
                    name,
                    input_file
                        .split('/')
                        .last()
                        .unwrap()
                        .split('.')
                        .next()
                        .unwrap()
                ));
                // println!("Writing results to {:?}", file_path);
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(file_path)?;
                println!("Starting tests for rule set: {}", name);
                for (i, expr) in exprs.iter().enumerate() {
                    // println!("Simplifying expression {}/{}", i + 1, exprs.len());
                    let runner = Runner::default()
                        .with_expr(expr)
                        .with_node_limit(1000000)
                        .with_iter_limit(1000000)
                        .with_scheduler(SimpleScheduler)
                        .with_time_limit(Duration::from_secs_f32(time_limit))
                        .run(rules);
                    let root = runner.roots[0];
                    let extractor = egg::Extractor::new(&runner.egraph, egg::AstSize);
                    let (_cost, best) = extractor.find_best(root);
                    // println!(
                    //     "expr={} simple={} complex={}",
                    //     expr.to_string(),
                    //     get_cost(&expr),
                    //     get_cost_complex(&expr)
                    // );
                    // println!("expr nodes: {:?}", expr.as_ref());
                    // println!("best nodes: {:?}", best.as_ref());
                    let result = TestResult {
                        original_term: &expr.to_string(),
                        simplified_term: &best.to_string(),
                        input_weight_simple: get_cost(&expr),
                        output_weight_simple: get_cost(&best),
                        input_weight_complex: get_cost_complex(&expr),
                        output_weight_complex: get_cost_complex(&best),
                        input_depth: get_depth(&expr),
                        output_depth: get_depth(&best),
                        simplification_time: runner.report().total_time,
                        apply_time: runner.report().apply_time,
                        search_time: runner.report().search_time,
                        rebuild_time: runner.report().rebuild_time,
                        stop_reason: format!("{:?}", runner.report().stop_reason),
                        iterations: runner.iterations.len(),
                    };
                    let line = serde_json::to_string(&result)?;
                    writeln!(file, "{}", line)?;
                }
            }
        }
    }
    Ok(())
}

//     let mut file = File::create("kbc_math2.txt")?;
//     println!("Starting simplification tests...");
//     let exprs =
//         load_exprs_from_file("/home/michi/Documents/thesis/KBC/term_gen/terms_max_len4.txt")?;
//     for expr in &exprs {
//         println!("Simplifying expression: {}", expr);
//         let mut total: f64 = 0.0;
//         for i in 0..1 {
//             // Run the rewrite rules on the expression
//             let runner = Runner::default()
//                 .with_explanations_enabled()
//                 .with_expr(expr)
//                 .with_iter_limit(100)
//                 .with_scheduler(SimpleScheduler)
//                 .run(&rules());

//             // Extract the best expression from the final e-graph
//             let root = runner.roots[0];
//             let extractor = egg::Extractor::new(&runner.egraph, egg::AstSize);
//             let (_cost, best) = extractor.find_best(root);
//             total += runner.report().total_time;
//             if i == 0 {
//                 writeln!(file, "\nAverage time: {}", (total / 1.0))?;
//                 writeln!(file, "\nApply time: {}", runner.report().apply_time)?;
//                 writeln!(file, "\nSearch time: {}", runner.report().search_time)?;
//                 writeln!(file, "\nRebuild time: {}", runner.report().rebuild_time)?;
//                 writeln!(file, "Stop reason: {:?}", runner.report().stop_reason)?;
//                 writeln!(file, "Iterations: {}", runner.iterations.len())?;
//                 writeln!(file, "Original: {}", expr)?;
//                 writeln!(file, "Simplified: {}\n", best)?;
//             }
//         }
//     }
//     Ok(())
// }
