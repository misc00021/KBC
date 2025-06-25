use egg::{rewrite as rw, *};
use ordered_float::NotNan;

pub type EGraph = egg::EGraph<Math, ConstantFold>;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;

pub type Constant = NotNan<f64>;

define_language! {
    pub enum Math {
        "d" = Diff([Id; 2]),
        "i" = Integral([Id; 2]),

        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
        "*" = Mul([Id; 2]),
        "/" = Div([Id; 2]),
        "pow" = Pow([Id; 2]),
        "ln" = Ln(Id),
        "sqrt" = Sqrt(Id),

        "sin" = Sin(Id),
        "cos" = Cos(Id),

        Constant(Constant),
        Symbol(Symbol),
    }
}

// You could use egg::AstSize, but this is useful for debugging, since
// it will really try to get rid of the Diff operator
pub struct MathCostFn;
impl egg::CostFunction<Math> for MathCostFn {
    type Cost = usize;
    fn cost<C>(&mut self, enode: &Math, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        let op_cost = match enode {
            Math::Diff(..) => 100,
            Math::Integral(..) => 100,
            _ => 1,
        };
        enode.fold(op_cost, |sum, i| sum + costs(i))
    }
}

#[derive(Default)]
pub struct ConstantFold;
impl Analysis<Math> for ConstantFold {
    type Data = Option<(Constant, PatternAst<Math>)>;

    fn make(egraph: &mut EGraph, enode: &Math) -> Self::Data {
        let x = |i: &Id| egraph[*i].data.as_ref().map(|d| d.0);
        Some(match enode {
            Math::Constant(c) => (*c, format!("{}", c).parse().unwrap()),
            Math::Add([a, b]) => (
                x(a)? + x(b)?,
                format!("(+ {} {})", x(a)?, x(b)?).parse().unwrap(),
            ),
            Math::Sub([a, b]) => (
                x(a)? - x(b)?,
                format!("(- {} {})", x(a)?, x(b)?).parse().unwrap(),
            ),
            Math::Mul([a, b]) => (
                x(a)? * x(b)?,
                format!("(* {} {})", x(a)?, x(b)?).parse().unwrap(),
            ),
            Math::Div([a, b]) if x(b) != Some(NotNan::new(0.0).unwrap()) => (
                x(a)? / x(b)?,
                format!("(/ {} {})", x(a)?, x(b)?).parse().unwrap(),
            ),
            _ => return None,
        })
    }

    fn merge(&mut self, to: &mut Self::Data, from: Self::Data) -> DidMerge {
        merge_option(to, from, |a, b| {
            assert_eq!(a.0, b.0, "Merged non-equal constants");
            DidMerge(false, false)
        })
    }

    fn modify(egraph: &mut EGraph, id: Id) {
        let data = egraph[id].data.clone();
        if let Some((c, pat)) = data {
            if egraph.are_explanations_enabled() {
                egraph.union_instantiations(
                    &pat,
                    &format!("{}", c).parse().unwrap(),
                    &Default::default(),
                    "constant_fold".to_string(),
                );
            } else {
                let added = egraph.add(Math::Constant(c));
                egraph.union(id, added);
            }
            // to not prune, comment this out
            egraph[id].nodes.retain(|n| n.is_leaf());

            #[cfg(debug_assertions)]
            egraph[id].assert_unique_leaves();
        }
    }
}

fn is_const_or_distinct_var(v: &str, w: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let v = v.parse().unwrap();
    let w = w.parse().unwrap();
    move |egraph, _, subst| {
        egraph.find(subst[v]) != egraph.find(subst[w])
            && (egraph[subst[v]].data.is_some()
                || egraph[subst[v]]
                    .nodes
                    .iter()
                    .any(|n| matches!(n, Math::Symbol(..))))
    }
}

fn is_const(var: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    move |egraph, _, subst| egraph[subst[var]].data.is_some()
}

fn is_sym(var: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    move |egraph, _, subst| {
        egraph[subst[var]]
            .nodes
            .iter()
            .any(|n| matches!(n, Math::Symbol(..)))
    }
}

fn is_not_zero(var: &str) -> impl Fn(&mut EGraph, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    move |egraph, _, subst| {
        if let Some(n) = &egraph[subst[var]].data {
            *(n.0) != 0.0
        } else {
            true
        }
    }
}

#[rustfmt::skip]
pub fn rules() -> Vec<Rewrite> { vec![
    rw!("minus-x-x"; "(- ?x ?x)" => "0"),
rw!("pow-x-1"; "(pow ?x 1)" => "?x"),
rw!("plus-comm"; "(+ ?x ?y)" => "(+ ?y ?x)"),
rw!("plus-x-0"; "(+ ?x 0)" => "?x"),
rw!("times-x-x"; "(* ?x ?x)" => "(pow ?x 2)"),
rw!("times-comm"; "(* ?x ?y)" => "(* ?y ?x)"),
rw!("times-x-0"; "(* ?x 0)" => "0"),
rw!("times-x-1"; "(* ?x 1)" => "?x"),
rw!("plus-minus"; "(+ ?x (* -1 ?y))" => "(- ?x ?y)"),
rw!("plus-assoc"; "(+ (+ ?x ?y) ?z)" => "(+ ?x (+ ?y ?z))"),
rw!("times-assoc"; "(* (* ?x ?y) ?z)" => "(* ?x (* ?y ?z))"),
rw!("plus-factor"; "(+ (* ?x ?y) (* ?x ?z))" => "(* ?x (+ ?y ?z))"),
rw!("pow-mult-merge"; "(* (pow ?x ?y) (pow ?x ?z))" => "(pow ?x (+ ?y ?z))"),
rw!("div-cancel"; "(/ ?x ?x)" => "1" if is_not_zero("?x")),
rw!("pow-zero"; "(pow ?x 0)" => "1" if is_not_zero("?x")),
rw!("pow-neg-one"; "(pow ?x -1)" => "(/ 1 ?x)" if is_not_zero("?x")),
rw!("times-recip"; "(* ?x (/ 1 ?x))" => "1" if is_not_zero("?x")),
rw!("times-pow-neg-one"; "(* ?y (pow ?x -1))" => "(/ ?y ?x)" if is_not_zero("?x")),
rw!("plus-zero-x"; "(+ 0 ?x)" => "?x"),
rw!("times-one-x"; "(* 1 ?x)" => "?x"),
rw!("minus-x-0"; "(- ?x 0)" => "?x"),
rw!("times-zero-x"; "(* 0 ?x)" => "0"),
rw!("pow-zero-two"; "(pow 0 2)" => "0"),
rw!("pow-one-two"; "(pow 1 2)" => "1"),
rw!("div-d-d"; "(/ d d)" => "1"),
rw!("pow-d-zero"; "(pow d 0)" => "1"),
rw!("plus-x-neg-one"; "(+ ?x -1)" => "(- ?x 1)"),
rw!("pow-d-neg-one"; "(pow d -1)" => "(/ 1 d)"),
rw!("minus-zero-one"; "(- 0 1)" => "-1"),
rw!("times-x-neg-one"; "(* ?x -1)" => "(- 0 ?x)"),
rw!("plus-neg-one-x"; "(+ -1 ?x)" => "(- ?x 1)"),
rw!("times-neg-one-x"; "(* -1 ?x)" => "(- 0 ?x)"),
rw!("pow-neg-one-two"; "(pow -1 2)" => "(- 0 -1)"),
rw!("times-d-recip-d"; "(* d (/ 1 d))" => "1"),
rw!("plus-x-minus0y"; "(+ ?x (- 0 ?y))" => "(- ?x ?y)"),
rw!("times-x-recip-d"; "(* ?x (/ 1 d))" => "(/ ?x d)"),
rw!("div-zero-d"; "(/ 0 d)" => "0"),
rw!("pow-zero-plus-x2"; "(pow 0 (+ ?x 2))" => "0"),
rw!("plus-deep-comm"; "(+ ?x (+ ?y ?z))" => "(+ ?y (+ ?x ?z))"),
rw!("plus-minus-expand"; "(+ ?x (- ?y ?z))" => "(- (+ ?x ?y) ?z)"),
rw!("minus-plus-x-y-x"; "(- (+ ?x ?y) ?x)" => "?y"),
rw!("times-deep-comm"; "(* ?x (* ?y ?z))" => "(* ?y (* ?x ?z))"),
rw!("times-d-divxd"; "(* d (/ ?x d))" => "?x"),
rw!("times-x-timesxy"; "(* ?x (* ?x ?y))" => "(* (pow ?x 2) ?y)"),
rw!("div-pow2-d"; "(/ (pow d 2) d)" => "d"),
rw!("distribute-times-plus1"; "(* ?x (+ ?y 1))" => "(+ ?x (* ?x ?y))"),
rw!("times-x-powxy"; "(* ?x (pow ?x ?y))" => "(pow ?x (+ ?y 1))"),
rw!("pow-zero-plus-x1"; "(pow 0 (+ ?x 1))" => "0"),
rw!("pow-zero-zero"; "(pow 0 0)" => "0"),
rw!("pow-zero-x"; "(pow 0 ?x)" => "0"),
rw!("pow-one-plus-2-1"; "(pow 1 (+ 2 1))" => "1"),
rw!("pow-x-doubley"; "(pow ?x (+ ?y ?y))" => "(pow (pow ?x ?y) 2)"),
rw!("pow-one-plus-2-2"; "(pow 1 (+ 2 2))" => "1"),
rw!("minus0x-y"; "(- (- 0 ?x) ?y)" => "(- 0 (+ ?x ?y))"),
rw!("minus-plus-x-y-y"; "(- (+ ?x ?y) ?y)" => "?x"),
rw!("plus-minus0x-y"; "(+ (- 0 ?x) ?y)" => "(- ?y ?x)"),
rw!("minus-minusx1-x"; "(- (- ?x 1) ?x)" => "-1"),
rw!("times-x-plus1-1"; "(* ?x (+ 1 1))" => "(+ ?x ?x)"),
rw!("pow-one-plus-x2"; "(pow 1 (+ ?x 2))" => "(pow 1 ?x)"),
rw!("pow-one-zero"; "(pow 1 0)" => "1"),
rw!("times-recip-d-x"; "(* (/ 1 d) ?x)" => "(/ ?x d)"),
rw!("minus-0-plus-x-1"; "(- 0 (+ ?x 1))" => "(- -1 ?x)"),
rw!("pow-x-plus-1-1"; "(pow ?x (+ 1 1))" => "(pow ?x 2)"),
rw!("pow-one-plus-x1"; "(pow 1 (+ ?x 1))" => "(pow 1 ?x)"),
rw!("pow-one-neg-one"; "(pow 1 -1)" => "1"),
rw!("minus-minus-x1-neg1"; "(- (- ?x 1) -1)" => "?x"),
rw!("minus-zero-neg1"; "(- 0 -1)" => "1"),
rw!("plus-x-1"; "(+ ?x 1)" => "(- ?x -1)"),
rw!("plus-1-x"; "(+ 1 ?x)" => "(- ?x -1)"),
rw!("plus-minusxy-z"; "(+ (- ?x ?y) ?z)" => "(- (+ ?x ?z) ?y)"),
rw!("minus-minusx-y-z"; "(- (- ?x ?y) ?z)" => "(- ?x (+ ?y ?z))"),
rw!("pow-d-minus-2-1"; "(pow d (- 2 1))" => "d"),
rw!("minus-x-plusxy"; "(- ?x (+ ?x ?y))" => "(- 0 ?y)"),
rw!("times-x-timesyx"; "(* ?x (* ?y ?x))" => "(* ?y (pow ?x 2))"),
rw!("minus-0-minus0x"; "(- 0 (- 0 ?x))" => "?x"),
rw!("times-x-powy2"; "(* ?x (pow ?y 2))" => "(* ?y (* ?y ?x))"),
rw!("plus-x-timesxy"; "(+ ?x (* ?x ?y))" => "(* ?x (- ?y -1))"),
rw!("times-x-minusy-1"; "(* ?x (- ?y -1))" => "(+ ?x (* ?y ?x))"),
rw!("times-x-divyd"; "(* ?x (/ ?y d))" => "(/ (* ?x ?y) d)"),
rw!("div-timesxd-d"; "(/ (* ?x d) d)" => "?x"),
rw!("times-x-minusx-1"; "(* ?x (- ?x -1))" => "(+ ?x (pow ?x 2))"),
rw!("minus-x-minus0y"; "(- ?x (- 0 ?y))" => "(+ ?x ?y)"),
rw!("plus-x-div-neg1-d"; "(+ ?x (/ -1 d))" => "(- ?x (/ 1 d))"),
rw!("minus-x-minusx-1"; "(- ?x (- ?x -1))" => "-1"),
rw!("plus-x-x"; "(+ ?x ?x)" => "(* ?x (- 1 -1))"),
rw!("minus-x-minusx1"; "(- ?x (- ?x 1))" => "1"),
rw!("minus-0-minusx-1"; "(- 0 (- ?x -1))" => "(- -1 ?x)"),
rw!("pow-one-plus2x"; "(pow 1 (+ 2 ?x))" => "(pow 1 ?x)"),
rw!("pow-one-minusx-1"; "(pow 1 (- ?x -1))" => "(pow 1 ?x)"),
rw!("div-dx-d"; "(/ (* d ?x) d)" => "?x"),
rw!("pow-recipd2"; "(pow (/ 1 d) 2)" => "(/ (/ 1 d) d)"),
rw!("pow-d-minusneg1-1"; "(pow d (- -1 1))" => "(/ (/ 1 d) d)"),
rw!("pow-one-minus2-1"; "(pow 1 (- 2 1))" => "1"),
rw!("minus-zero-div1d"; "(- 0 (/ 1 d))" => "(/ -1 d)"),
rw!("pow-one-minusneg1-1"; "(pow 1 (- -1 1))" => "1"),
rw!("div-minus0d-d"; "(/ (- 0 d) d)" => "-1"),
rw!("minus-x-plusyx"; "(- ?x (+ ?y ?x))" => "(- 0 ?y)"),
rw!("times-minusx-1-y"; "(* (- ?x -1) ?y)" => "(+ ?y (* ?y ?x))")
]}

egg::test_fn! {
    kbc_math_associate_adds, [
        rw!("comm-add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
        rw!("assoc-add"; "(+ ?a (+ ?b ?c))" => "(+ (+ ?a ?b) ?c)"),
    ],
    runner = Runner::default()
        .with_iter_limit(7)
        .with_scheduler(SimpleScheduler),
    "(+ 1 (+ 2 (+ 3 (+ 4 (+ 5 (+ 6 7))))))"
    =>
    "(+ 7 (+ 6 (+ 5 (+ 4 (+ 3 (+ 2 1))))))"
    @check |r: Runner<Math, ()>| assert_eq!(r.egraph.number_of_classes(), 127)
}

egg::test_fn! {
    #[should_panic(expected = "Could not prove goal 0")]
    kbc_math_fail, rules(),
    "(+ x y)" => "(/ x y)"
}

egg::test_fn! {kbc_math_simplify_add, rules(), "(+ x (+ x (+ x x)))" => "(* 4 x)" }
egg::test_fn! {kbc_math_powers, rules(), "(* (pow 2 x) (pow 2 y))" => "(pow 2 (+ x y))"}

egg::test_fn! {
    kbc_math_simplify_const, rules(),
    "(+ 1 (- a (* (- 2 1) a)))" => "1"
}

egg::test_fn! {
    kbc_math_simplify_root, rules(),
    runner = Runner::default().with_node_limit(75_000),
    r#"
    (/ 1
       (- (/ (+ 1 (sqrt five))
             2)
          (/ (- 1 (sqrt five))
             2)))"#
    =>
    "(/ 1 (sqrt five))"
}

egg::test_fn! {
    kbc_math_simplify_factor, rules(),
    "(* (+ x 3) (+ x 1))"
    =>
    "(+ (+ (* x x) (* 4 x)) 3)"
}

// egg::test_fn! {kbc_math_diff_same,      rules(), "(d x x)" => "1"}
// egg::test_fn! {kbc_math_diff_different, rules(), "(d x y)" => "0"}
// egg::test_fn! {kbc_math_diff_simple1,   rules(), "(d x (+ 1 (* 2 x)))" => "2"}
// egg::test_fn! {kbc_math_diff_simple2,   rules(), "(d x (+ 1 (* y x)))" => "y"}
// egg::test_fn! {kbc_math_diff_ln,        rules(), "(d x (ln x))" => "(/ 1 x)"}

// egg::test_fn! {
//     diff_power_simple, rules(),
//     "(d x (pow x 3))" => "(* 3 (pow x 2))"
// }

// egg::test_fn! {
//     diff_power_harder, rules(),
//     runner = Runner::default()
//         .with_time_limit(std::time::Duration::from_secs(10))
//         .with_iter_limit(60)
//         .with_node_limit(100_000)
//         .with_explanations_enabled()
//         // HACK this needs to "see" the end expression
//         .with_expr(&"(* x (- (* 3 x) 14))".parse().unwrap()),
//     "(d x (- (pow x 3) (* 7 (pow x 2))))"
//     =>
//     "(* x (- (* 3 x) 14))"
// }

// egg::test_fn! {
//     integ_one, rules(), "(i 1 x)" => "x"
// }

// egg::test_fn! {
//     integ_sin, rules(), "(i (cos x) x)" => "(sin x)"
// }

// egg::test_fn! {
//     integ_x, rules(), "(i (pow x 1) x)" => "(/ (pow x 2) 2)"
// }

// egg::test_fn! {
//     integ_part1, rules(), "(i (* x (cos x)) x)" => "(+ (* x (sin x)) (cos x))"
// }

// egg::test_fn! {
//     integ_part2, rules(),
//     "(i (* (cos x) x) x)" => "(+ (* x (sin x)) (cos x))"
// }

// egg::test_fn! {
//     integ_part3, rules(), "(i (ln x) x)" => "(- (* x (ln x)) x)"
// }

#[test]
fn assoc_mul_saturates() {
    let expr: RecExpr<Math> = "(* x 1)".parse().unwrap();

    let runner: Runner<Math, ConstantFold> = Runner::default()
        .with_iter_limit(3)
        .with_expr(&expr)
        .run(&rules());

    assert!(matches!(runner.stop_reason, Some(StopReason::Saturated)));
}

#[test]
fn test_union_trusted() {
    let expr: RecExpr<Math> = "(+ (* x 1) y)".parse().unwrap();
    let expr2 = "20".parse().unwrap();
    let mut runner: Runner<Math, ConstantFold> = Runner::default()
        .with_explanations_enabled()
        .with_iter_limit(3)
        .with_expr(&expr)
        .run(&rules());
    let lhs = runner.egraph.add_expr(&expr);
    let rhs = runner.egraph.add_expr(&expr2);
    runner.egraph.union_trusted(lhs, rhs, "whatever");
    let proof = runner.explain_equivalence(&expr, &expr2).get_flat_strings();
    assert_eq!(proof, vec!["(+ (* x 1) y)", "(Rewrite=> whatever 20)"]);
}

#[cfg(feature = "lp")]
#[test]
fn kbc_math_lp_extract() {
    let expr: RecExpr<Math> = "(pow (+ x (+ x x)) (+ x x))".parse().unwrap();

    let runner: Runner<Math, ConstantFold> = Runner::default()
        .with_iter_limit(3)
        .with_expr(&expr)
        .run(&rules());
    let root = runner.roots[0];

    let best = Extractor::new(&runner.egraph, AstSize).find_best(root).1;
    let lp_best = LpExtractor::new(&runner.egraph, AstSize).solve(root);

    println!("input   [{}] {}", expr.len(), expr);
    println!("normal  [{}] {}", best.len(), best);
    println!("ilp cse [{}] {}", lp_best.len(), lp_best);

    assert_ne!(best, lp_best);
    assert_eq!(lp_best.len(), 4);
}

#[test]
fn kbc_math_ematching_bench() {
    let exprs = &[
        "(i (ln x) x)",
        "(i (+ x (cos x)) x)",
        "(i (* (cos x) x) x)",
        "(d x (+ 1 (* 2 x)))",
        "(d x (- (pow x 3) (* 7 (pow x 2))))",
        "(+ (* y (+ x y)) (- (+ x 2) (+ x x)))",
        "(/ 1 (- (/ (+ 1 (sqrt five)) 2) (/ (- 1 (sqrt five)) 2)))",
    ];

    let extra_patterns = &[
        "(+ ?a (+ ?b ?c))",
        "(+ (+ ?a ?b) ?c)",
        "(* ?a (* ?b ?c))",
        "(* (* ?a ?b) ?c)",
        "(+ ?a (* -1 ?b))",
        "(* ?a (pow ?b -1))",
        "(* ?a (+ ?b ?c))",
        "(pow ?a (+ ?b ?c))",
        "(+ (* ?a ?b) (* ?a ?c))",
        "(* (pow ?a ?b) (pow ?a ?c))",
        "(* ?x (/ 1 ?x))",
        "(d ?x (+ ?a ?b))",
        "(+ (d ?x ?a) (d ?x ?b))",
        "(d ?x (* ?a ?b))",
        "(+ (* ?a (d ?x ?b)) (* ?b (d ?x ?a)))",
        "(d ?x (sin ?x))",
        "(d ?x (cos ?x))",
        "(* -1 (sin ?x))",
        "(* -1 (cos ?x))",
        "(i (cos ?x) ?x)",
        "(i (sin ?x) ?x)",
        "(d ?x (ln ?x))",
        "(d ?x (pow ?f ?g))",
        "(* (pow ?f ?g) (+ (* (d ?x ?f) (/ ?g ?f)) (* (d ?x ?g) (ln ?f))))",
        "(i (pow ?x ?c) ?x)",
        "(/ (pow ?x (+ ?c 1)) (+ ?c 1))",
        "(i (+ ?f ?g) ?x)",
        "(i (- ?f ?g) ?x)",
        "(+ (i ?f ?x) (i ?g ?x))",
        "(- (i ?f ?x) (i ?g ?x))",
        "(i (* ?a ?b) ?x)",
        "(- (* ?a (i ?b ?x)) (i (* (d ?x ?a) (i ?b ?x)) ?x))",
    ];

    egg::test::bench_egraph("kbc_math", rules(), exprs, extra_patterns);
}

#[test]
fn test_basic_egraph_union_intersect() {
    let mut egraph1 = EGraph::new(ConstantFold {}).with_explanations_enabled();
    let mut egraph2 = EGraph::new(ConstantFold {}).with_explanations_enabled();
    egraph1.union_instantiations(
        &"x".parse().unwrap(),
        &"y".parse().unwrap(),
        &Default::default(),
        "",
    );
    egraph1.union_instantiations(
        &"y".parse().unwrap(),
        &"z".parse().unwrap(),
        &Default::default(),
        "",
    );
    egraph2.union_instantiations(
        &"x".parse().unwrap(),
        &"y".parse().unwrap(),
        &Default::default(),
        "",
    );
    egraph2.union_instantiations(
        &"x".parse().unwrap(),
        &"a".parse().unwrap(),
        &Default::default(),
        "",
    );

    let mut egraph3 = egraph1.egraph_intersect(&egraph2, ConstantFold {});

    egraph2.egraph_union(&egraph1);

    assert_eq!(
        egraph2.add_expr(&"x".parse().unwrap()),
        egraph2.add_expr(&"y".parse().unwrap())
    );
    assert_eq!(
        egraph3.add_expr(&"x".parse().unwrap()),
        egraph3.add_expr(&"y".parse().unwrap())
    );

    assert_eq!(
        egraph2.add_expr(&"x".parse().unwrap()),
        egraph2.add_expr(&"z".parse().unwrap())
    );
    assert_ne!(
        egraph3.add_expr(&"x".parse().unwrap()),
        egraph3.add_expr(&"z".parse().unwrap())
    );
    assert_eq!(
        egraph2.add_expr(&"x".parse().unwrap()),
        egraph2.add_expr(&"a".parse().unwrap())
    );
    assert_ne!(
        egraph3.add_expr(&"x".parse().unwrap()),
        egraph3.add_expr(&"a".parse().unwrap())
    );

    assert_eq!(
        egraph2.add_expr(&"y".parse().unwrap()),
        egraph2.add_expr(&"a".parse().unwrap())
    );
    assert_ne!(
        egraph3.add_expr(&"y".parse().unwrap()),
        egraph3.add_expr(&"a".parse().unwrap())
    );
}

#[test]
fn test_intersect_basic() {
    let mut egraph1 = EGraph::new(ConstantFold {}).with_explanations_enabled();
    let mut egraph2 = EGraph::new(ConstantFold {}).with_explanations_enabled();
    egraph1.union_instantiations(
        &"(+ x 0)".parse().unwrap(),
        &"(+ y 0)".parse().unwrap(),
        &Default::default(),
        "",
    );
    egraph2.union_instantiations(
        &"x".parse().unwrap(),
        &"y".parse().unwrap(),
        &Default::default(),
        "",
    );
    egraph2.add_expr(&"(+ x 0)".parse().unwrap());
    egraph2.add_expr(&"(+ y 0)".parse().unwrap());

    let mut egraph3 = egraph1.egraph_intersect(&egraph2, ConstantFold {});

    assert_ne!(
        egraph3.add_expr(&"x".parse().unwrap()),
        egraph3.add_expr(&"y".parse().unwrap())
    );
    assert_eq!(
        egraph3.add_expr(&"(+ x 0)".parse().unwrap()),
        egraph3.add_expr(&"(+ y 0)".parse().unwrap())
    );
}

#[test]
fn test_medium_intersect() {
    let mut egraph1 = egg::EGraph::<Math, ()>::new(());

    egraph1.add_expr(&"(sqrt (ln 1))".parse().unwrap());
    let ln = egraph1.add_expr(&"(ln 1)".parse().unwrap());
    let a = egraph1.add_expr(&"(sqrt (sin pi))".parse().unwrap());
    let b = egraph1.add_expr(&"(* 1 pi)".parse().unwrap());
    let pi = egraph1.add_expr(&"pi".parse().unwrap());
    egraph1.union(a, b);
    egraph1.union(a, pi);
    let c = egraph1.add_expr(&"(+ pi pi)".parse().unwrap());
    egraph1.union(ln, c);
    let k = egraph1.add_expr(&"k".parse().unwrap());
    let one = egraph1.add_expr(&"1".parse().unwrap());
    egraph1.union(k, one);
    egraph1.rebuild();

    assert_eq!(
        egraph1.add_expr(&"(ln k)".parse().unwrap()),
        egraph1.add_expr(&"(+ (* k pi) (* k pi))".parse().unwrap())
    );

    let mut egraph2 = egg::EGraph::<Math, ()>::new(());
    let ln = egraph2.add_expr(&"(ln 2)".parse().unwrap());
    let k = egraph2.add_expr(&"k".parse().unwrap());
    let mk1 = egraph2.add_expr(&"(* k 1)".parse().unwrap());
    egraph2.union(mk1, k);
    let two = egraph2.add_expr(&"2".parse().unwrap());
    egraph2.union(mk1, two);
    let mul2pi = egraph2.add_expr(&"(+ (* 2 pi) (* 2 pi))".parse().unwrap());
    egraph2.union(ln, mul2pi);
    egraph2.rebuild();

    assert_eq!(
        egraph2.add_expr(&"(ln k)".parse().unwrap()),
        egraph2.add_expr(&"(+ (* k pi) (* k pi))".parse().unwrap())
    );

    let mut egraph3 = egraph1.egraph_intersect(&egraph2, ());

    assert_eq!(
        egraph3.add_expr(&"(ln k)".parse().unwrap()),
        egraph3.add_expr(&"(+ (* k pi) (* k pi))".parse().unwrap())
    );
}
