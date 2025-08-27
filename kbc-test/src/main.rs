mod kbc_math;
use std::io::{BufRead, BufReader};
mod math;
use std::fs::File;
use std::io::Write;

use egg::{EGraph, RecExpr, Runner, SimpleScheduler};
use kbc_math::{ConstantFold, Math, rules};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("kbc_math2.txt")?;
    println!("Starting simplification tests...");
    let exprs = load_exprs_from_file("/home/michi/Documents/Thesis/KBC/term_gen/terms_max_len4.txt")?;
    // let mut exprs: std::vec::Vec<RecExpr<Math>> = std::vec![];
    // // Parse an expression to test
    // exprs.push("(* b (+ 0 (* x (+ (+ (+ (* x 1) (- (pow y 0) (/ (* 0 z) (pow a 1)))) (/ (pow (pow b 1) 1) (* 1 1))) (/ (* d 0) (pow e 1))))))"
    //     .parse()
    //     .unwrap());
    // exprs.push("(+ 0 x)".parse().unwrap());
    // exprs.push("(* 1 y)".parse().unwrap());
    // exprs.push("(/ z 1)".parse().unwrap());
    // exprs.push("(pow a 0)".parse().unwrap());
    // exprs.push("(pow b 1)".parse().unwrap());

    // exprs.push("(* x (/ 1 x))".parse().unwrap());
    // exprs.push("(+ x (- 0 x))".parse().unwrap());
    // exprs.push("(/ (pow x 2) x)".parse().unwrap());
    // exprs.push("(- (+ a b) b)".parse().unwrap());

    // exprs.push("(* x (+ y (pow z 0)))".parse().unwrap());
    // exprs.push("(/ (+ x (* y z)) (pow a 1))".parse().unwrap());
    // exprs.push("(- (pow x 2) (/ (* y 0) z))".parse().unwrap());
    // exprs.push("(+ (pow x (+ 1 1)) (- 0 (* y 1)))".parse().unwrap());
    // exprs.push("(* (+ a (* b 0)) (/ 1 (pow c 1)))".parse().unwrap());

    // exprs.push(
    //     "(+ (* x (/ y (pow z 1))) (- (pow a 0) (/ 0 b)))"
    //         .parse()
    //         .unwrap(),
    // );
    // exprs.push(
    //     "(/ (* (pow x 2) (+ y 0)) (pow (pow z 1) 1))"
    //         .parse()
    //         .unwrap(),
    // );
    // exprs.push(
    //     "(* (- (pow x 2) (pow x 1)) (+ (/ x x) (/ 0 x)))"
    //         .parse()
    //         .unwrap(),
    // );
    // exprs.push("(- (pow (+ x 1) 1) (pow x 1))".parse().unwrap());

    // exprs.push("(pow (pow x 1) 1)".parse().unwrap());
    // exprs.push("(/ (* x y) (/ x y))".parse().unwrap());
    // exprs.push("(* x (- x -1))".parse().unwrap());
    // exprs.push("(+ (* x y) (* x z))".parse().unwrap());

    // exprs.push(
    //     "(+ x (+ 0 (+ 0 (+ 0 (+ 0 (+ 0 (+ 0 (+ 0 0))))))) )"
    //         .parse()
    //         .unwrap(),
    // );
    // exprs.push("(* y (* 1 (* 1 (* 1 (* 1 1)))) )".parse().unwrap());
    // exprs.push("(- (- (- (- x 0) 0) 0) 0)".parse().unwrap());
    // exprs.push(
    //     "(+ (+ (+ (* x 1) (* x 1)) (* x 1)) (* x 1))"
    //         .parse()
    //         .unwrap(),
    // );
    // exprs.push("(* (* (+ x 0) (+ y 0)) (+ z 0))".parse().unwrap());
    // exprs.push("(/ (/ (/ a 1) 1) 1)".parse().unwrap());
    // exprs.push("(pow (pow (pow x 1) 1) 1)".parse().unwrap());
    // exprs.push(
    //     "(* x (+ y (+ z (+ a (+ b (+ c (+ d e)))))))"
    //         .parse()
    //         .unwrap(),
    // );
    // exprs.push(
    //     "(+ (+ (+ (+ (+ (+ (+ x y) z) a) b) c) d) e)"
    //         .parse()
    //         .unwrap(),
    // );
    // exprs.push(
    //     "(- (- (- (- (- (- (- x y) z) a) b) c) d) e)"
    //         .parse()
    //         .unwrap(),
    // );
    // exprs.push("(* (* (* (* (* (* x y) z) a) b) c) d)".parse().unwrap());
    // exprs.push(
    //     "(+ x (* y (+ z (* a (+ b (* c (+ d (* e (+ f (* g h))))))))) )"
    //         .parse()
    //         .unwrap(),
    // );
    // exprs.push("(/ (* x (* x (* x x))) (* x x))".parse().unwrap());
    // exprs.push(
    //     "(+ (+ (+ (+ x (* x y)) (* x z)) (* x a)) (* x b))"
    //         .parse()
    //         .unwrap(),
    // );
    // exprs.push(
    //     "(+ 0 (+ 0 (+ 0 (+ 0 (+ 0 (+ 0 (+ 0 (+ 0 (+ 0 x)))))))))"
    //         .parse()
    //         .unwrap(),
    // );
    // exprs.push("--a".parse().unwrap());

    for expr in &exprs {
        println!("Simplifying expression: {}", expr);
        let mut total: f64 = 0.0;
        for i in 0..1 {
            // Run the rewrite rules on the expression
            let runner = Runner::default()
                .with_explanations_enabled()
                .with_expr(expr)
                .with_iter_limit(100)
                .with_scheduler(SimpleScheduler())
                .run(&rules());

            // Extract the best expression from the final e-graph
            let root = runner.roots[0];
            let extractor = egg::Extractor::new(&runner.egraph, egg::AstSize);
            let (_cost, best) = extractor.find_best(root);
            total += runner.report().total_time;
            if i == 0 {
                writeln!(file, "\nAverage time: {}", (total / 1.0))?;
                writeln!(file, "\nApply time: {}", runner.report().apply_time)?;
                writeln!(file, "\nSearch time: {}", runner.report().search_time)?;
                writeln!(file, "\nRebuild time: {}", runner.report().rebuild_time)?;
                writeln!(file, "Stop reason: {:?}", runner.report().stop_reason)?;
                writeln!(file, "Iterations: {}", runner.iterations.len())?;
                writeln!(file, "Original: {}", expr)?;
                writeln!(file, "Simplified: {}\n", best)?;
            }
        }
    }
    Ok(())
}
