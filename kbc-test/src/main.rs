/* https://github.com/egraphs-good/egg/blob/main/tests/math.rs is the basis of all files in kbc-test/src */

mod kbc_base_rules;
mod kbc_extending;
mod kbc_extending_no_div_no_pow;
mod kbc_extending_no_div_no_pow_no_unorderable;
mod kbc_extending_no_unorderable;
mod kbc_extending_sep_div;
mod kbc_extending_sep_div_no_unorderable;
mod kbc_extending_sep_div_plus;
mod kbc_extending_sep_div_plus_no_unorderable;
mod kbc_replacing;
mod kbc_replacing_no_div_no_pow;
mod kbc_replacing_no_div_no_pow_no_unorderable;
mod kbc_replacing_no_unorderable;
mod kbc_replacing_sep_div;
mod kbc_replacing_sep_div_no_unorderable;
mod kbc_replacing_sep_div_plus;
mod kbc_replacing_sep_div_plus_no_unorderable;
use std::io::{BufRead, BufReader};
use std::time::Duration;
mod math;
use serde_derive::Serialize;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use egg::{EGraph, RecExpr, Runner, SimpleScheduler};
use kbc_base_rules::rules as kbc_base_rules;
use kbc_extending::rules as kbc_extending;
use kbc_extending_no_div_no_pow::rules as kbc_extending_no_div_no_pow;
use kbc_extending_no_div_no_pow_no_unorderable::rules as kbc_extending_no_div_no_pow_no_unorderable;
use kbc_extending_no_unorderable::rules as kbc_extending_no_unorderable;
use kbc_extending_sep_div::rules as kbc_extending_sep_div;
use kbc_extending_sep_div_no_unorderable::rules as kbc_extending_sep_div_no_unorderable;
use kbc_extending_sep_div_plus::rules as kbc_extending_sep_div_plus;
use kbc_extending_sep_div_plus_no_unorderable::rules as kbc_extending_sep_div_plus_no_unorderable;
use kbc_replacing::rules as kbc_replacing;
use kbc_replacing_no_div_no_pow::rules as kbc_replacing_no_div_no_pow;
use kbc_replacing_no_div_no_pow_no_unorderable::rules as kbc_replacing_no_div_no_pow_no_unorderable;
use kbc_replacing_no_unorderable::rules as kbc_replacing_no_unorderable;
use kbc_replacing_sep_div::rules as kbc_replacing_sep_div;
use kbc_replacing_sep_div_no_unorderable::rules as kbc_replacing_sep_div_no_unorderable;
use kbc_replacing_sep_div_plus::rules as kbc_replacing_sep_div_plus;
use kbc_replacing_sep_div_plus_no_unorderable::rules as kbc_replacing_sep_div_plus_no_unorderable;

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
        // ("base_rules", kbc_base_rules()),
        // ("extending", kbc_extending()),
        // ("extending_no_div_no_pow", kbc_extending_no_div_no_pow()),
        // (
        //     "extending_no_div_no_pow_no_unorderable",
        //     kbc_extending_no_div_no_pow_no_unorderable(),
        // ),
        // ("extending_no_unorderable", kbc_extending_no_unorderable()),
        // ("extending_sep_div", kbc_extending_sep_div()),
        // (
        //     "extending_sep_div_no_unorderable",
        //     kbc_extending_sep_div_no_unorderable(),
        // ),
        // ("replacing", kbc_replacing()),
        // ("replacing_no_div_no_pow", kbc_replacing_no_div_no_pow()),
        // (
        //     "replacing_no_div_no_pow_no_unorderable",
        //     kbc_replacing_no_div_no_pow_no_unorderable(),
        // ),
        // ("replacing_no_unorderable", kbc_replacing_no_unorderable()),
        // ("replacing_sep_div", kbc_replacing_sep_div()),
        // (
        //     "replacing_sep_div_no_unorderable",
        //     kbc_replacing_sep_div_no_unorderable(),
        // ),
        ("extending_sep_div_plus", kbc_extending_sep_div_plus()),
        (
            "extending_sep_div_plus_no_unorderable",
            kbc_extending_sep_div_plus_no_unorderable(),
        ),
        ("replacing_sep_div_plus", kbc_replacing_sep_div_plus()),
        (
            "replacing_sep_div_plus_no_unorderable",
            kbc_replacing_sep_div_plus_no_unorderable(),
        ),
    ];
    for time_limit in vec![
        0.0001, 0.0005, 0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1., /*, 2., 3., 4., 5.*/
    ] {
        // let parent_dir =
        //     std::path::Path::new("with_timelimits").join(format!("timelimit{}", time_limit));
        let parent_dir = std::path::Path::new("all_results");
        for (dir_name, rules) in &rule_files {
            // let out_dir = parent_dir.join(dir_name).join(
            //     input_file
            //         .split('/')
            //         .last()
            //         .unwrap()
            //         .split('.')
            //         .next()
            //         .unwrap(),
            // );
            let out_dir = parent_dir;
            std::fs::create_dir_all(&out_dir)?;
            // println!("Writing results to {:?}", out_dir);
            for (name, rules) in rules {
                let file_path = out_dir.join(format!(
                    "EqSat-{}-{}-{}.jsonl",
                    name,
                    input_file
                        .split('/')
                        .last()
                        .unwrap()
                        .split('.')
                        .next()
                        .unwrap(),
                    time_limit
                ));
                if Path::new(&file_path).exists() {
                    println!("File {:?} already exists, skipping...", file_path);
                    continue;
                }
                // println!("Writing results to {:?}", file_path);
                let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(file_path)?;
                println!(
                    "Starting tests for rule set {} with time limit {}",
                    name, time_limit
                );
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
