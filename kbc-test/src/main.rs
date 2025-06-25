mod kbc_math;
mod math;

use egg::{EGraph, RecExpr, Runner};
use kbc_math::{rules, ConstantFold, Math};

fn main() {
    // Parse an expression to test
    let expr: RecExpr<Math> = "(d x (+ 1 (* 2 x)))".parse().unwrap();

    // Run the rewrite rules on the expression
    let runner = Runner::default().with_expr(&expr).run(&rules());

    // Extract the best expression from the final e-graph
    let root = runner.roots[0];
    let extractor = egg::Extractor::new(&runner.egraph, egg::AstSize);
    let (_cost, best) = extractor.find_best(root);

    println!("Original: {}", expr);
    println!("Simplified: {}", best);
}
