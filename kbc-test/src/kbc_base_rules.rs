use crate::math::{ConstantFold, Math};
use egg::{rewrite as rw, *};

pub type EGraph = egg::EGraph<Math, ConstantFold>;
pub type Rewrite = egg::Rewrite<Math, ConstantFold>;

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
pub fn rules() -> Vec<(String, Vec<Rewrite>)> { vec![


("math_no_diff_int".to_string(), vec![
    rw!("comm-add";  "(+ ?x ?y)"        => "(+ ?y ?x)"),
    rw!("comm-mul";  "(* ?x ?y)"        => "(* ?y ?x)"),
    rw!("assoc-add"; "(+ ?x (+ ?y ?z))" => "(+ (+ ?x ?y) ?z)"),
    rw!("assoc-mul"; "(* ?x (* ?y ?z))" => "(* (* ?x ?y) ?z)"),

    rw!("sub-canon"; "(- ?x ?y)" => "(+ ?x (* (neg 1) ?y))"),
    rw!("div-canon"; "(/ ?x ?y)" => "(* ?x (pow ?y (neg 1)))" if is_not_zero("?y")),
    // rw!("canon-sub"; "(+ ?x (* (neg 1) ?y))"   => "(- ?x ?y)"),
    // rw!("canon-div"; "(* ?x (pow ?y (neg 1)))" => "(/ ?x ?y)" if is_not_zero("?y")),

    rw!("zero-add"; "(+ ?x 0)" => "?x"),
    rw!("zero-mul"; "(* ?x 0)" => "0"),
    rw!("one-mul";  "(* ?x 1)" => "?x"),

    rw!("add-zero"; "?x" => "(+ ?x 0)"),
    rw!("mul-one";  "?x" => "(* ?x 1)"),

    rw!("cancel-sub"; "(- ?x ?x)" => "0"),
    rw!("cancel-div"; "(/ ?x ?x)" => "1" if is_not_zero("?x")),

    rw!("distribute"; "(* ?x (+ ?y ?z))"        => "(+ (* ?x ?y) (* ?x ?z))"),
    rw!("factor"    ; "(+ (* ?x ?y) (* ?x ?z))" => "(* ?x (+ ?y ?z))"),

    rw!("pow-mul"; "(* (pow ?x ?y) (pow ?x ?z))" => "(pow ?x (+ ?y ?z))" if is_not_zero("?x")),
    rw!("pow0"; "(pow ?x 0)" => "1" if is_not_zero("?x")),
    rw!("pow1"; "(pow ?x 1)" => "?x"),
    rw!("pow2"; "(pow ?x 2)" => "(* ?x ?x)"),
    rw!("pow-recip"; "(pow ?x (neg 1))" => "(/ 1 ?x)" if is_not_zero("?x")),
    rw!("recip-mul-div"; "(* ?x (/ 1 ?x))" => "1" if is_not_zero("?x")),

]),

("math_no_diff_int_no_div_no_pow".to_string(), vec![
rw!("comm-add";  "(+ ?x ?y)"        => "(+ ?y ?x)"),
    rw!("comm-mul";  "(* ?x ?y)"        => "(* ?y ?x)"),
    rw!("assoc-add"; "(+ ?x (+ ?y ?z))" => "(+ (+ ?x ?y) ?z)"),
    rw!("assoc-mul"; "(* ?x (* ?y ?z))" => "(* (* ?x ?y) ?z)"),

    rw!("sub-canon"; "(- ?x ?y)" => "(+ ?x (* -1 ?y))"),
    // rw!("div-canon"; "(/ ?x ?y)" => "(* ?x (pow ?y -1))" if is_not_zero("?y")),
    // rw!("canon-sub"; "(+ ?x (* -1 ?y))"   => "(- ?x ?y)"),
    // rw!("canon-div"; "(* ?x (pow ?y -1))" => "(/ ?x ?y)" if is_not_zero("?y")),

    rw!("zero-add"; "(+ ?x 0)" => "?x"),
    rw!("zero-mul"; "(* ?x 0)" => "0"),
    rw!("one-mul";  "(* ?x 1)" => "?x"),

    rw!("add-zero"; "?x" => "(+ ?x 0)"),
    rw!("mul-one";  "?x" => "(* ?x 1)"),

    rw!("cancel-sub"; "(- ?x ?x)" => "0"),
    // rw!("cancel-div"; "(/ ?x ?x)" => "1" if is_not_zero("?x")),

    rw!("distribute"; "(* ?x (+ ?y ?z))"        => "(+ (* ?x ?y) (* ?x ?z))"),
    rw!("factor"    ; "(+ (* ?x ?y) (* ?x ?z))" => "(* ?x (+ ?y ?z))"),

    // rw!("pow-mul"; "(* (pow ?x ?y) (pow ?x ?z))" => "(pow ?x (+ ?y ?z))" if is_not_zero("?x")),
    // rw!("pow0"; "(pow ?x 0)" => "1"
    //    if is_not_zero("?x")),
    // rw!("pow1"; "(pow ?x 1)" => "?x"),
    // rw!("pow2"; "(pow ?x 2)" => "(* ?x ?x)"),
    // rw!("pow-recip"; "(pow ?x -1)" => "(/ 1 ?x)"
    //    if is_not_zero("?x")),
    // rw!("recip-mul-div"; "(* ?x (/ 1 ?x))" => "1" if is_not_zero("?x")),

]),

]}
