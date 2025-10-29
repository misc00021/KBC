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


("math_no_diff_int_sep_div_plus_KBC_max_rules_100_no_unorderable".to_string(), vec![

rw!("rule_1"; "(+ ?x ?y)" => "(+ ?y ?x)"),
rw!("rule_2"; "(+ ?x 0)" => "?x"),
rw!("rule_3"; "(+ 0 ?x)" => "?x"),
rw!("rule_4"; "(* ?x ?x)" => "(pow ?x 2)"),
rw!("rule_5"; "(* ?x ?y)" => "(* ?y ?x)"),
rw!("rule_6"; "(* ?x 1)" => "?x"),
rw!("rule_7"; "(* ?x 0)" => "0"),
rw!("rule_8"; "(* 1 ?x)" => "?x"),
rw!("rule_9"; "(* 0 ?x)" => "0"),
rw!("rule_10"; "(- ?x ?x)" => "0"),
rw!("rule_11"; "(- ?x 0)" => "?x"),
rw!("rule_12"; "(- 0 1)" => "-1"),
rw!("rule_13"; "(pow ?x 1)" => "?x"),
rw!("rule_14"; "(pow 1 0)" => "1"),
rw!("rule_15"; "(pow 1 2)" => "1"),
rw!("rule_16"; "(pow 0 2)" => "0"),
rw!("rule_17"; "(/ ?x 1)" => "?x"),
rw!("rule_18"; "(+ ?x -1 )" => "(- ?x 1)"),
rw!("rule_19"; "(+ -1 ?x)" => "(- ?x 1)"),
rw!("rule_20"; "(* ?x -1 )" => "(- 0 ?x)"),
rw!("rule_21"; "(* -1 ?x)" => "(- 0 ?x)"),
rw!("rule_22"; "(- ?x -1 )" => "(+ ?x 1)"),
rw!("rule_23"; "(pow 1 -1 )" => "1"),
rw!("rule_24"; "(pow -1 2)" => "1"),
rw!("rule_25"; "(+ ?x (+ ?y ?z) )" => "(+ ?y (+ ?x ?z) )"),

rw!("rule_27"; "(+ ?x (- ?y ?z) )" => "(- (+ ?x ?y) ?z)"),
rw!("rule_28"; "(+ (+ ?x ?y) ?z)" => "(+ ?x (+ ?y ?z) )"),
rw!("rule_29"; "(+ (- ?x ?y) ?z)" => "(- (+ ?x ?z) ?y)"),


rw!("rule_32"; "(* ?x (+ 1 ?x) )" => "(+ ?x (pow ?x 2) )"),



rw!("rule_36"; "(* ?x (* ?y ?z) )" => "(* ?y (* ?x ?z) )"),

rw!("rule_38"; "(* ?x (- 0 ?y) )" => "(- 0 (* ?x ?y) )"),




rw!("rule_43"; "(* (* ?x ?y) ?z)" => "(* ?x (* ?y ?z) )"),
rw!("rule_44"; "(- ?x (+ ?y ?x) )" => "(- 0 ?y)"),

rw!("rule_46"; "(- 0 (+ ?x 1) )" => "(- -1 ?x)"),
rw!("rule_47"; "(- 0 (+ 1 ?x) )" => "(- -1 ?x)"),
rw!("rule_48"; "(- 0 (- ?x 1) )" => "(- 1 ?x)"),
rw!("rule_49"; "(- (+ ?x ?y) ?y)" => "?x"),

rw!("rule_51"; "(- (- ?x ?y) ?z)" => "(- ?x (+ ?y ?z) )"),
rw!("rule_52"; "(pow ?x (+ 1 1) )" => "(pow ?x 2)"),
rw!("rule_53"; "(pow 1 (+ ?x 1) )" => "(pow 1 ?x)"),
rw!("rule_54"; "(pow 1 (+ ?x 2) )" => "(pow 1 ?x)"),
rw!("rule_55"; "(pow 1 (+ 1 ?x) )" => "(pow 1 ?x)"),
rw!("rule_56"; "(pow 1 (+ 2 ?x) )" => "(pow 1 ?x)"),
rw!("rule_57"; "(pow 1 (- 2 1) )" => "1"),


rw!("rule_60"; "(+ (* ?x ?y) (* ?x ?z) )" => "(* ?x (+ ?y ?z) )"),
rw!("rule_61"; "(+ (* ?x ?y) (* ?z ?x) )" => "(* ?x (+ ?y ?z) )"),
rw!("rule_62"; "(+ (* ?x ?y) (* ?y ?z) )" => "(* ?y (+ ?x ?z) )"),
rw!("rule_63"; "(pow ?x 0)" => "1" if is_not_zero("?x")),
rw!("rule_64"; "(/ ?x ?x)" => "1" if is_not_zero("?x")),
rw!("rule_65"; "(pow ?x -1)" => "(/ 1 ?x)" if is_not_zero("?x")),
rw!("rule_66"; "(* ?x (/ ?y ?x) )" => "?y" if is_not_zero("?x")),
rw!("rule_67"; "(/ (* ?x ?y) ?x)" => "?y" if is_not_zero("?x")),
rw!("rule_68"; "(/ (* ?y ?x) ?x)" => "?y" if is_not_zero("?x")),
rw!("rule_69"; "(/ (pow ?x 2) ?x)" => "?x" if is_not_zero("?x")),
rw!("rule_70"; "(pow ?x -1 )" => "(/ 1 ?x)" if is_not_zero("?x")),
rw!("rule_71"; "(* ?y (/ 1 ?x) )" => "(/ ?y ?x)" if is_not_zero("?x")),
rw!("rule_72"; "(* (/ 1 ?x) ?y)" => "(/ ?y ?x)" if is_not_zero("?x")),
rw!("rule_73"; "(* ?y (pow ?x -1 ) )" => "(/ ?y ?x)" if is_not_zero("?x")),
rw!("rule_74"; "(* ?x (pow ?x ?y) )" => "(pow ?x (+ ?y 1) )" if is_not_zero("?x")),
rw!("rule_75"; "(* ?y (/ ?z ?x) )" => "(/ (* ?y ?z) ?x)" if is_not_zero("?x")),
rw!("rule_76"; "(* (/ ?y ?x) ?z)" => "(/ (* ?y ?z) ?x)" if is_not_zero("?x")),
rw!("rule_77"; "(/ ?y (/ ?y ?x) )" => "?x" if is_not_zero("?x") if is_not_zero("?y")),
rw!("rule_78"; "(* (pow ?x ?y) (pow ?x ?z) )" => "(pow ?x (+ ?y ?z) )" if is_not_zero("?x")),
rw!("rule_79"; "(/ (/ ?x ?y) ?x)" => "(/ 1 ?y)" if is_not_zero("?x") if is_not_zero("?y")),
rw!("rule_80"; "(/ (/ ?z ?y) ?x)" => "(/ (/ ?z ?x) ?y)" if is_not_zero("?x") if is_not_zero("?y")),
]),

("math_no_diff_int_sep_div_plus_KBC_max_rules_150_no_unorderable".to_string(), vec![

rw!("rule_1"; "(+ ?x ?y)" => "(+ ?y ?x)"),
rw!("rule_2"; "(+ ?x 0)" => "?x"),
rw!("rule_3"; "(+ 0 ?x)" => "?x"),
rw!("rule_4"; "(* ?x ?x)" => "(pow ?x 2)"),
rw!("rule_5"; "(* ?x ?y)" => "(* ?y ?x)"),
rw!("rule_6"; "(* ?x 1)" => "?x"),
rw!("rule_7"; "(* ?x 0)" => "0"),
rw!("rule_8"; "(* 1 ?x)" => "?x"),
rw!("rule_9"; "(* 0 ?x)" => "0"),
rw!("rule_10"; "(- ?x ?x)" => "0"),
rw!("rule_11"; "(- ?x 0)" => "?x"),
rw!("rule_12"; "(- 0 1)" => "-1"),
rw!("rule_13"; "(pow ?x 1)" => "?x"),
rw!("rule_14"; "(pow 1 0)" => "1"),
rw!("rule_15"; "(pow 1 2)" => "1"),
rw!("rule_16"; "(pow 0 2)" => "0"),
rw!("rule_17"; "(/ ?x 1)" => "?x"),
rw!("rule_18"; "(+ ?x -1 )" => "(- ?x 1)"),
rw!("rule_19"; "(+ -1 ?x)" => "(- ?x 1)"),
rw!("rule_20"; "(* ?x -1 )" => "(- 0 ?x)"),
rw!("rule_21"; "(* -1 ?x)" => "(- 0 ?x)"),
rw!("rule_22"; "(- ?x -1 )" => "(+ ?x 1)"),
rw!("rule_23"; "(pow 1 -1 )" => "1"),
rw!("rule_24"; "(pow -1 2)" => "1"),
rw!("rule_25"; "(+ ?x (+ ?y ?z) )" => "(+ ?y (+ ?x ?z) )"),

rw!("rule_27"; "(+ ?x (- ?y ?z) )" => "(- (+ ?x ?y) ?z)"),
rw!("rule_28"; "(+ (+ ?x ?y) ?z)" => "(+ ?x (+ ?y ?z) )"),
rw!("rule_29"; "(+ (- ?x ?y) ?z)" => "(- (+ ?x ?z) ?y)"),



rw!("rule_33"; "(* ?x (+ 1 ?x) )" => "(+ ?x (pow ?x 2) )"),



rw!("rule_37"; "(* ?x (* ?y ?z) )" => "(* ?y (* ?x ?z) )"),
rw!("rule_38"; "(* ?x (- ?x 1) )" => "(- (pow ?x 2) ?x)"),

rw!("rule_40"; "(* ?x (- 1 ?x) )" => "(- ?x (pow ?x 2) )"),

rw!("rule_42"; "(* ?x (- 0 ?y) )" => "(- 0 (* ?x ?y) )"),




rw!("rule_47"; "(* (* ?x ?y) ?z)" => "(* ?x (* ?y ?z) )"),


rw!("rule_50"; "(* (- 0 ?x) ?y)" => "(- 0 (* ?x ?y) )"),
rw!("rule_51"; "(- ?x (+ ?y ?x) )" => "(- 0 ?y)"),

rw!("rule_53"; "(- ?x (- ?x ?y) )" => "?y"),
rw!("rule_54"; "(- ?x (- 0 ?y) )" => "(+ ?x ?y)"),

rw!("rule_56"; "(- 0 (+ ?x 1) )" => "(- -1 ?x)"),
rw!("rule_57"; "(- 0 (+ 1 ?x) )" => "(- -1 ?x)"),
rw!("rule_58"; "(- 0 (- ?x ?y) )" => "(- ?y ?x)"),
rw!("rule_59"; "(- (+ ?x ?y) ?y)" => "?x"),
rw!("rule_60"; "(- (+ ?x 1) ?y)" => "(- ?x (- ?y 1) )"),
rw!("rule_61"; "(- (+ 1 ?x) ?y)" => "(- ?x (- ?y 1) )"),
rw!("rule_62"; "(- (- ?x ?y) ?z)" => "(- ?x (+ ?y ?z) )"),
rw!("rule_63"; "(pow ?x (+ 1 1) )" => "(pow ?x 2)"),
rw!("rule_64"; "(pow 1 (+ ?x 1) )" => "(pow 1 ?x)"),
rw!("rule_65"; "(pow 1 (+ ?x 2) )" => "(pow 1 ?x)"),
rw!("rule_66"; "(pow 1 (+ 1 ?x) )" => "(pow 1 ?x)"),
rw!("rule_67"; "(pow 1 (+ 2 ?x) )" => "(pow 1 ?x)"),
rw!("rule_68"; "(pow 1 (- ?x 1) )" => "(pow 1 ?x)"),
rw!("rule_69"; "(pow 1 (- 1 ?x) )" => "(pow 1 (- 0 ?x) )"),
rw!("rule_70"; "(pow 1 (- 0 2) )" => "1"),
rw!("rule_71"; "(pow 1 (- 2 ?x) )" => "(pow 1 (- 0 ?x) )"),

rw!("rule_73"; "(pow (- 0 ?x) 2)" => "(pow ?x 2)"),


rw!("rule_76"; "(- ?x (- -1 1) )" => "(+ ?x (+ 1 1) )"),
rw!("rule_77"; "(- -1 (- ?x 1) )" => "(- 0 ?x)"),
rw!("rule_78"; "(pow 1 (- -1 2) )" => "1"),
rw!("rule_79"; "(+ (* ?x ?y) (* ?x ?z) )" => "(* ?x (+ ?y ?z) )"),
rw!("rule_80"; "(+ (* ?x ?y) (* ?y ?z) )" => "(* ?y (+ ?x ?z) )"),
rw!("rule_81"; "(+ (* ?x ?y) (* ?z ?y) )" => "(* ?y (+ ?x ?z) )"),
rw!("rule_82"; "(+ (pow ?x 2) (* ?y ?x) )" => "(* ?x (+ ?x ?y) )"),
rw!("rule_83"; "(* ?x (* ?x (pow ?x 2) ) )" => "(pow (pow ?x 2) 2)"),
rw!("rule_84"; "(* ?x (* ?x (pow ?y 2) ) )" => "(pow (* ?x ?y) 2)"),
rw!("rule_85"; "(* ?x (* (pow ?y 2) ?x) )" => "(pow (* ?x ?y) 2)"),
rw!("rule_86"; "(- (+ ?x ?y) (+ ?x ?z) )" => "(- ?y ?z)"),
rw!("rule_87"; "(- (+ ?x (+ ?y ?z) ) ?y)" => "(+ ?x ?z)"),
rw!("rule_88"; "(- (* ?x (+ ?y 1) ) ?x)" => "(* ?x ?y)"),
rw!("rule_89"; "(- (* ?x (+ 1 ?y) ) ?x)" => "(* ?x ?y)"),
rw!("rule_90"; "(- (* (+ ?x 1) ?y) ?y)" => "(* ?x ?y)"),
rw!("rule_91"; "(pow 1 (+ ?x (+ ?y 1) ) )" => "(pow 1 (+ ?x ?y) )"),
rw!("rule_92"; "(pow 1 (+ ?x (+ ?y 2) ) )" => "(pow 1 (+ ?x ?y) )"),
rw!("rule_93"; "(pow 1 (+ ?x (+ 1 ?y) ) )" => "(pow 1 (+ ?x ?y) )"),
rw!("rule_94"; "(pow 1 (+ ?x (+ 2 ?y) ) )" => "(pow 1 (+ ?x ?y) )"),
rw!("rule_95"; "(pow 1 (- (+ ?x 2) ?y) )" => "(pow 1 (- ?x ?y) )"),

rw!("rule_97"; "(pow ?x 0)" => "1" if is_not_zero("?x")),
rw!("rule_98"; "(/ ?x ?x)" => "1" if is_not_zero("?x")),
rw!("rule_99"; "(/ 0 ?x)" => "0" if is_not_zero("?x")),



rw!("rule_103"; "(pow ?x -1)" => "(/ 1 ?x)" if is_not_zero("?x")),

rw!("rule_105"; "(* ?x (/ ?y ?x) )" => "?y" if is_not_zero("?x")),
rw!("rule_106"; "(/ (* ?x ?y) ?x)" => "?y" if is_not_zero("?x")),
rw!("rule_107"; "(/ (* ?y ?x) ?x)" => "?y" if is_not_zero("?x")),
rw!("rule_108"; "(/ (pow ?x 2) ?x)" => "?x" if is_not_zero("?x")),
rw!("rule_109"; "(pow ?x -1 )" => "(/ 1 ?x)" if is_not_zero("?x")),
rw!("rule_110"; "(* ?y (/ 1 ?x) )" => "(/ ?y ?x)" if is_not_zero("?x")),
rw!("rule_111"; "(* (/ 1 ?x) ?y)" => "(/ ?y ?x)" if is_not_zero("?x")),
rw!("rule_112"; "(* ?y (pow ?x -1 ) )" => "(/ ?y ?x)" if is_not_zero("?x")),
rw!("rule_113"; "(* ?x (pow ?x ?y) )" => "(pow ?x (+ ?y 1) )" if is_not_zero("?x")),
rw!("rule_114"; "(* ?y (/ ?z ?x) )" => "(/ (* ?y ?z) ?x)" if is_not_zero("?x")),
rw!("rule_115"; "(* (/ ?y ?x) ?z)" => "(/ (* ?y ?z) ?x)" if is_not_zero("?x")),
rw!("rule_116"; "(/ ?y (/ ?y ?x) )" => "?x" if is_not_zero("?x") if is_not_zero("?y")),
rw!("rule_117"; "(* (pow ?x ?y) (pow ?x ?z) )" => "(pow ?x (+ ?y ?z) )" if is_not_zero("?x")),
rw!("rule_118"; "(/ (/ ?x ?y) ?x)" => "(/ 1 ?y)" if is_not_zero("?x") if is_not_zero("?y")),
rw!("rule_119"; "(/ (/ ?z ?y) ?x)" => "(/ (/ ?z ?x) ?y)" if is_not_zero("?x") if is_not_zero("?y")),
]),

("math_no_diff_int_sep_div_plus_KBC_max_rules_200_no_unorderable".to_string(), vec![

rw!("rule_1"; "(+ ?x ?y)" => "(+ ?y ?x)"),
rw!("rule_2"; "(+ ?x 0)" => "?x"),
rw!("rule_3"; "(+ 0 ?x)" => "?x"),
rw!("rule_4"; "(* ?x ?x)" => "(pow ?x 2)"),
rw!("rule_5"; "(* ?x ?y)" => "(* ?y ?x)"),
rw!("rule_6"; "(* ?x 1)" => "?x"),
rw!("rule_7"; "(* ?x 0)" => "0"),
rw!("rule_8"; "(* 1 ?x)" => "?x"),
rw!("rule_9"; "(* 0 ?x)" => "0"),
rw!("rule_10"; "(- ?x ?x)" => "0"),
rw!("rule_11"; "(- ?x 0)" => "?x"),
rw!("rule_12"; "(- 0 1)" => "-1"),
rw!("rule_13"; "(pow ?x 1)" => "?x"),
rw!("rule_14"; "(pow 1 0)" => "1"),
rw!("rule_15"; "(pow 1 2)" => "1"),
rw!("rule_16"; "(pow 0 2)" => "0"),
rw!("rule_17"; "(/ ?x 1)" => "?x"),
rw!("rule_18"; "(+ ?x -1 )" => "(- ?x 1)"),
rw!("rule_19"; "(+ -1 ?x)" => "(- ?x 1)"),
rw!("rule_20"; "(* ?x -1 )" => "(- 0 ?x)"),
rw!("rule_21"; "(* -1 ?x)" => "(- 0 ?x)"),
rw!("rule_22"; "(- ?x -1 )" => "(+ ?x 1)"),
rw!("rule_23"; "(pow 1 -1 )" => "1"),
rw!("rule_24"; "(pow -1 2)" => "1"),

rw!("rule_26"; "(+ ?x (+ ?y ?z) )" => "(+ ?y (+ ?x ?z) )"),

rw!("rule_28"; "(+ ?x (- ?y ?z) )" => "(- ?x (- ?z ?y) )"),

rw!("rule_30"; "(+ (+ ?x ?y) ?z)" => "(+ ?x (+ ?y ?z) )"),
rw!("rule_31"; "(+ (- ?x ?y) ?z)" => "(- ?x (- ?y ?z) )"),



rw!("rule_35"; "(* ?x (+ 1 ?x) )" => "(+ ?x (pow ?x 2) )"),



rw!("rule_39"; "(* ?x (* ?y ?z) )" => "(* ?y (* ?x ?z) )"),
rw!("rule_40"; "(* ?x (- ?x 1) )" => "(- (pow ?x 2) ?x)"),

rw!("rule_42"; "(* ?x (- 1 ?x) )" => "(- ?x (pow ?x 2) )"),

rw!("rule_44"; "(* ?x (- 0 ?y) )" => "(- 0 (* ?x ?y) )"),




rw!("rule_49"; "(* (* ?x ?y) ?z)" => "(* ?x (* ?y ?z) )"),


rw!("rule_52"; "(* (- 0 ?x) ?y)" => "(- 0 (* ?x ?y) )"),
rw!("rule_53"; "(- ?x (+ ?y ?x) )" => "(- 0 ?y)"),


rw!("rule_56"; "(- ?x (- 0 ?y) )" => "(+ ?x ?y)"),
rw!("rule_57"; "(- ?x (- ?y ?z) )" => "(- ?z (- ?y ?x) )"),


rw!("rule_60"; "(- 0 (+ ?x 1) )" => "(- -1 ?x)"),
rw!("rule_61"; "(- 0 (+ 1 ?x) )" => "(- -1 ?x)"),
rw!("rule_62"; "(- (+ ?x ?y) ?z)" => "(- ?x (- ?z ?y) )"),
rw!("rule_63"; "(- (- ?x ?y) ?z)" => "(- ?x (+ ?y ?z) )"),
rw!("rule_64"; "(pow ?x (+ 1 1) )" => "(pow ?x 2)"),
rw!("rule_65"; "(pow 1 (+ ?x 1) )" => "(pow 1 ?x)"),
rw!("rule_66"; "(pow 1 (+ ?x 2) )" => "(pow 1 ?x)"),
rw!("rule_67"; "(pow 1 (+ 1 ?x) )" => "(pow 1 ?x)"),
rw!("rule_68"; "(pow 1 (+ 2 ?x) )" => "(pow 1 ?x)"),
rw!("rule_69"; "(pow 1 (- ?x 1) )" => "(pow 1 ?x)"),
rw!("rule_70"; "(pow 1 (- ?x 2) )" => "(pow 1 ?x)"),
rw!("rule_71"; "(pow 1 (- 1 ?x) )" => "(pow 1 (- 0 ?x) )"),
rw!("rule_72"; "(pow 1 (- 2 ?x) )" => "(pow 1 (- 0 ?x) )"),

rw!("rule_74"; "(pow (- 0 ?x) 2)" => "(pow ?x 2)"),


rw!("rule_77"; "(- ?x (- -1 1) )" => "(+ ?x (+ 1 1) )"),
rw!("rule_78"; "(pow 1 (- -1 ?x) )" => "(pow 1 (- 0 ?x) )"),
rw!("rule_79"; "(pow (- -1 ?x) 2)" => "(pow (+ ?x 1) 2)"),




rw!("rule_84"; "(+ ?x (* ?x (- ?y 1) ) )" => "(* ?x ?y)"),
rw!("rule_85"; "(+ ?x (* (- ?y 1) ?x) )" => "(* ?x ?y)"),
rw!("rule_86"; "(+ (* ?x ?y) (* ?x ?z) )" => "(* ?x (+ ?y ?z) )"),
rw!("rule_87"; "(+ (* ?x ?y) (* ?y ?z) )" => "(* ?y (+ ?x ?z) )"),
rw!("rule_88"; "(+ (* ?x ?y) (* ?z ?y) )" => "(* ?y (+ ?x ?z) )"),
rw!("rule_89"; "(+ (pow ?x 2) (* ?y ?x) )" => "(* ?x (+ ?x ?y) )"),


rw!("rule_92"; "(* ?x (+ 1 (* ?x ?y) ) )" => "(+ ?x (* (pow ?x 2) ?y) )"),
rw!("rule_93"; "(* ?x (+ 1 (* ?y ?x) ) )" => "(+ ?x (* ?y (pow ?x 2) ) )"),


rw!("rule_96"; "(* ?x (* ?x (+ 1 1) ) )" => "(+ (pow ?x 2) (pow ?x 2) )"),
rw!("rule_97"; "(* ?x (* ?x (pow ?x 2) ) )" => "(pow (pow ?x 2) 2)"),
rw!("rule_98"; "(* ?x (* ?x (pow ?y 2) ) )" => "(pow (* ?x ?y) 2)"),




rw!("rule_103"; "(* ?x (* (pow ?y 2) ?x) )" => "(pow (* ?x ?y) 2)"),




rw!("rule_108"; "(* (pow ?x 2) (pow ?y 2) )" => "(pow (* ?x ?y) 2)"),
rw!("rule_109"; "(* (+ ?x (* ?x ?y) ) ?z)" => "(* ?x (* (+ ?y 1) ?z) )"),

rw!("rule_111"; "(- ?x (* (+ 1 1) ?x) )" => "(- 0 ?x)"),
rw!("rule_112"; "(- ?x (* (- 1 ?y) ?x) )" => "(* ?x ?y)"),
rw!("rule_113"; "(- ?x (- ?y (+ ?z ?w) ) )" => "(- ?z (- ?y (+ ?x ?w) ) )"),
rw!("rule_114"; "(- (* ?x (+ ?y 1) ) ?x)" => "(* ?x ?y)"),
rw!("rule_115"; "(- (* ?x (+ 1 ?y) ) ?x)" => "(* ?x ?y)"),
rw!("rule_116"; "(- (* (+ ?x 1) ?y) ?y)" => "(* ?x ?y)"),
rw!("rule_117"; "(- (* (+ 1 ?x) ?y) ?y)" => "(* ?x ?y)"),
rw!("rule_118"; "(pow 1 (+ ?x (+ ?y 1) ) )" => "(pow 1 (+ ?x ?y) )"),
rw!("rule_119"; "(pow 1 (+ ?x (+ ?y 2) ) )" => "(pow 1 (+ ?x ?y) )"),
rw!("rule_120"; "(pow 1 (+ ?x (+ 1 ?y) ) )" => "(pow 1 (+ ?x ?y) )"),
rw!("rule_121"; "(pow 1 (+ ?x (+ 2 ?y) ) )" => "(pow 1 (+ ?x ?y) )"),
rw!("rule_122"; "(pow 1 (- ?x (+ ?y 1) ) )" => "(pow 1 (- ?x ?y) )"),
rw!("rule_123"; "(pow 1 (- ?x (- ?y 1) ) )" => "(pow 1 (- ?x ?y) )"),
rw!("rule_124"; "(pow 1 (- ?x (- ?y 2) ) )" => "(pow 1 (- ?x ?y) )"),

rw!("rule_126"; "(pow ?x 0)" => "1" if is_not_zero("?x")),
rw!("rule_127"; "(/ ?x ?x)" => "1" if is_not_zero("?x")),
rw!("rule_128"; "(/ 0 ?x)" => "0" if is_not_zero("?x")),



rw!("rule_132"; "(pow ?x -1)" => "(/ 1 ?x)" if is_not_zero("?x")),

rw!("rule_134"; "(* ?x (/ ?y ?x) )" => "?y" if is_not_zero("?x")),
rw!("rule_135"; "(/ (* ?x ?y) ?x)" => "?y" if is_not_zero("?x")),
rw!("rule_136"; "(/ (* ?y ?x) ?x)" => "?y" if is_not_zero("?x")),
rw!("rule_137"; "(/ (pow ?x 2) ?x)" => "?x" if is_not_zero("?x")),
rw!("rule_138"; "(pow ?x -1 )" => "(/ 1 ?x)" if is_not_zero("?x")),
rw!("rule_139"; "(* ?y (/ 1 ?x) )" => "(/ ?y ?x)" if is_not_zero("?x")),
rw!("rule_140"; "(* (/ 1 ?x) ?y)" => "(/ ?y ?x)" if is_not_zero("?x")),
rw!("rule_141"; "(* ?y (pow ?x -1 ) )" => "(/ ?y ?x)" if is_not_zero("?x")),
rw!("rule_142"; "(* ?x (pow ?x ?y) )" => "(pow ?x (+ ?y 1) )" if is_not_zero("?x")),
rw!("rule_143"; "(* ?y (/ ?z ?x) )" => "(/ (* ?y ?z) ?x)" if is_not_zero("?x")),
rw!("rule_144"; "(* (/ ?y ?x) ?z)" => "(/ (* ?y ?z) ?x)" if is_not_zero("?x")),
rw!("rule_145"; "(/ ?y (/ ?y ?x) )" => "?x" if is_not_zero("?x") if is_not_zero("?y")),
rw!("rule_146"; "(* (pow ?x ?y) (pow ?x ?z) )" => "(pow ?x (+ ?y ?z) )" if is_not_zero("?x")),
rw!("rule_147"; "(/ (/ ?x ?y) ?x)" => "(/ 1 ?y)" if is_not_zero("?x") if is_not_zero("?y")),
rw!("rule_148"; "(/ (/ ?z ?y) ?x)" => "(/ (/ ?z ?x) ?y)" if is_not_zero("?x") if is_not_zero("?y")),
]),

("math_no_diff_int_sep_div_plus_KBC_max_rules_40_no_unorderable".to_string(), vec![
rw!("rule_0"; "(+ ?x ?y)" => "(+ ?y ?x)"),
rw!("rule_1"; "(+ ?x 0)" => "?x"),
rw!("rule_2"; "(+ 0 ?x)" => "?x"),
rw!("rule_3"; "(* ?x ?x)" => "(pow ?x 2)"),
rw!("rule_4"; "(* ?x ?y)" => "(* ?y ?x)"),
rw!("rule_5"; "(* ?x 1)" => "?x"),
rw!("rule_6"; "(* ?x 0)" => "0"),
rw!("rule_7"; "(* 1 ?x)" => "?x"),
rw!("rule_8"; "(- ?x ?x)" => "0"),
rw!("rule_9"; "(- ?x 0)" => "?x"),
rw!("rule_10"; "(pow ?x 1)" => "?x"),
rw!("rule_11"; "(pow 1 2)" => "1"),
rw!("rule_12"; "(/ ?x 1)" => "?x"),
rw!("rule_13"; "(+ (+ ?x ?y) ?z)" => "(+ ?x (+ ?y ?z) )"),
rw!("rule_14"; "(* ?x (* ?y ?z) )" => "(* ?y (* ?x ?z) )"),
rw!("rule_15"; "(* (* ?x ?y) ?z)" => "(* ?x (* ?y ?z) )"),
rw!("rule_16"; "(pow ?x (+ 1 1) )" => "(pow ?x 2)"),
rw!("rule_17"; "(pow 1 (+ ?x 1) )" => "(pow 1 ?x)"),
rw!("rule_18"; "(pow 1 (+ ?x 2) )" => "(pow 1 ?x)"),
rw!("rule_19"; "(+ ?x (* -1 ?y) )" => "(- ?x ?y)"),
rw!("rule_20"; "(+ (* ?x ?y) (* ?x ?z) )" => "(* ?x (+ ?y ?z) )"),
rw!("rule_21"; "(pow ?x 0)" => "1" if is_not_zero("?x")),
rw!("rule_22"; "(/ ?x ?x)" => "1" if is_not_zero("?x")),
rw!("rule_23"; "(pow ?x -1)" => "(/ 1 ?x)" if is_not_zero("?x")),
rw!("rule_24"; "(* ?x (/ ?y ?x) )" => "?y" if is_not_zero("?x")),
rw!("rule_25"; "(/ (* ?x ?y) ?x)" => "?y" if is_not_zero("?x")),
rw!("rule_26"; "(/ (* ?y ?x) ?x)" => "?y" if is_not_zero("?x")),
rw!("rule_27"; "(/ (pow ?x 2) ?x)" => "?x" if is_not_zero("?x")),
rw!("rule_28"; "(pow ?x -1 )" => "(/ 1 ?x)" if is_not_zero("?x")),
rw!("rule_29"; "(* ?y (/ 1 ?x) )" => "(/ ?y ?x)" if is_not_zero("?x")),
rw!("rule_30"; "(* (/ 1 ?x) ?y)" => "(/ ?y ?x)" if is_not_zero("?x")),
rw!("rule_31"; "(* ?y (pow ?x -1 ) )" => "(/ ?y ?x)" if is_not_zero("?x")),
rw!("rule_32"; "(* ?x (pow ?x ?y) )" => "(pow ?x (+ ?y 1) )" if is_not_zero("?x")),
rw!("rule_33"; "(* ?y (/ ?z ?x) )" => "(/ (* ?y ?z) ?x)" if is_not_zero("?x")),
rw!("rule_34"; "(* (/ ?y ?x) ?z)" => "(/ (* ?y ?z) ?x)" if is_not_zero("?x")),
rw!("rule_35"; "(/ ?y (/ ?y ?x) )" => "?x" if is_not_zero("?x") if is_not_zero("?y")),
rw!("rule_36"; "(* (pow ?x ?y) (pow ?x ?z) )" => "(pow ?x (+ ?y ?z) )" if is_not_zero("?x")),
rw!("rule_37"; "(/ (/ ?x ?y) ?x)" => "(/ 1 ?y)" if is_not_zero("?x") if is_not_zero("?y")),
rw!("rule_38"; "(/ (/ ?z ?y) ?x)" => "(/ (/ ?z ?x) ?y)" if is_not_zero("?x") if is_not_zero("?y")),
]),

("math_no_diff_int_sep_div_plus_KBC_max_rules_60_no_unorderable".to_string(), vec![
rw!("rule_0"; "(+ ?x ?y)" => "(+ ?y ?x)"),
rw!("rule_1"; "(+ ?x 0)" => "?x"),
rw!("rule_2"; "(+ 0 ?x)" => "?x"),
rw!("rule_3"; "(* ?x ?x)" => "(pow ?x 2)"),
rw!("rule_4"; "(* ?x ?y)" => "(* ?y ?x)"),
rw!("rule_5"; "(* ?x 1)" => "?x"),
rw!("rule_6"; "(* ?x 0)" => "0"),
rw!("rule_7"; "(* 1 ?x)" => "?x"),
rw!("rule_8"; "(* 0 ?x)" => "0"),
rw!("rule_9"; "(- ?x ?x)" => "0"),
rw!("rule_10"; "(- ?x 0)" => "?x"),
rw!("rule_11"; "(- 0 1)" => "-1"),
rw!("rule_12"; "(pow ?x 1)" => "?x"),
rw!("rule_13"; "(pow 1 0)" => "1"),
rw!("rule_14"; "(pow 1 2)" => "1"),
rw!("rule_15"; "(pow 0 2)" => "0"),
rw!("rule_16"; "(/ ?x 1)" => "?x"),
rw!("rule_17"; "(+ ?x -1 )" => "(- ?x 1)"),
rw!("rule_18"; "(+ -1 ?x)" => "(- ?x 1)"),
rw!("rule_19"; "(* ?x -1 )" => "(- 0 ?x)"),
rw!("rule_20"; "(* -1 ?x)" => "(- 0 ?x)"),
rw!("rule_21"; "(pow 1 -1 )" => "1"),
rw!("rule_22"; "(+ ?x (+ ?y ?z) )" => "(+ ?y (+ ?x ?z) )"),
rw!("rule_23"; "(+ ?x (- ?y ?z) )" => "(- (+ ?x ?y) ?z)"),
rw!("rule_24"; "(+ (+ ?x ?y) ?z)" => "(+ ?x (+ ?y ?z) )"),



rw!("rule_28"; "(* ?x (* ?y ?z) )" => "(* ?y (* ?x ?z) )"),
rw!("rule_29"; "(* (* ?x ?y) ?z)" => "(* ?x (* ?y ?z) )"),
rw!("rule_30"; "(- (+ ?x ?y) ?y)" => "?x"),
rw!("rule_31"; "(pow ?x (+ 1 1) )" => "(pow ?x 2)"),
rw!("rule_32"; "(pow 1 (+ ?x 1) )" => "(pow 1 ?x)"),
rw!("rule_33"; "(pow 1 (+ ?x 2) )" => "(pow 1 ?x)"),
rw!("rule_34"; "(pow 1 (+ 1 ?x) )" => "(pow 1 ?x)"),
rw!("rule_35"; "(pow 1 (+ 2 ?x) )" => "(pow 1 ?x)"),
rw!("rule_36"; "(pow 1 (- 2 1) )" => "1"),

rw!("rule_38"; "(+ (* ?x ?y) (* ?x ?z) )" => "(* ?x (+ ?y ?z) )"),
rw!("rule_39"; "(pow ?x 0)" => "1" if is_not_zero("?x")),
rw!("rule_40"; "(/ ?x ?x)" => "1" if is_not_zero("?x")),
rw!("rule_41"; "(pow ?x -1)" => "(/ 1 ?x)" if is_not_zero("?x")),
rw!("rule_42"; "(* ?x (/ ?y ?x) )" => "?y" if is_not_zero("?x")),
rw!("rule_43"; "(/ (* ?x ?y) ?x)" => "?y" if is_not_zero("?x")),
rw!("rule_44"; "(/ (* ?y ?x) ?x)" => "?y" if is_not_zero("?x")),
rw!("rule_45"; "(/ (pow ?x 2) ?x)" => "?x" if is_not_zero("?x")),
rw!("rule_46"; "(pow ?x -1 )" => "(/ 1 ?x)" if is_not_zero("?x")),
rw!("rule_47"; "(* ?y (/ 1 ?x) )" => "(/ ?y ?x)" if is_not_zero("?x")),
rw!("rule_48"; "(* (/ 1 ?x) ?y)" => "(/ ?y ?x)" if is_not_zero("?x")),
rw!("rule_49"; "(* ?y (pow ?x -1 ) )" => "(/ ?y ?x)" if is_not_zero("?x")),
rw!("rule_50"; "(* ?x (pow ?x ?y) )" => "(pow ?x (+ ?y 1) )" if is_not_zero("?x")),
rw!("rule_51"; "(* ?y (/ ?z ?x) )" => "(/ (* ?y ?z) ?x)" if is_not_zero("?x")),
rw!("rule_52"; "(* (/ ?y ?x) ?z)" => "(/ (* ?y ?z) ?x)" if is_not_zero("?x")),
rw!("rule_53"; "(/ ?y (/ ?y ?x) )" => "?x" if is_not_zero("?x") if is_not_zero("?y")),
rw!("rule_54"; "(* (pow ?x ?y) (pow ?x ?z) )" => "(pow ?x (+ ?y ?z) )" if is_not_zero("?x")),
rw!("rule_55"; "(/ (/ ?x ?y) ?x)" => "(/ 1 ?y)" if is_not_zero("?x") if is_not_zero("?y")),
rw!("rule_56"; "(/ (/ ?z ?y) ?x)" => "(/ (/ ?z ?x) ?y)" if is_not_zero("?x") if is_not_zero("?y")),
]),

("math_no_diff_int_sep_div_plus_KBC_max_rules_80_no_unorderable".to_string(), vec![

rw!("rule_1"; "(+ ?x ?y)" => "(+ ?y ?x)"),
rw!("rule_2"; "(+ ?x 0)" => "?x"),
rw!("rule_3"; "(+ 0 ?x)" => "?x"),
rw!("rule_4"; "(* ?x ?x)" => "(pow ?x 2)"),
rw!("rule_5"; "(* ?x ?y)" => "(* ?y ?x)"),
rw!("rule_6"; "(* ?x 1)" => "?x"),
rw!("rule_7"; "(* ?x 0)" => "0"),
rw!("rule_8"; "(* 1 ?x)" => "?x"),
rw!("rule_9"; "(* 0 ?x)" => "0"),
rw!("rule_10"; "(- ?x ?x)" => "0"),
rw!("rule_11"; "(- ?x 0)" => "?x"),
rw!("rule_12"; "(- 0 1)" => "-1"),
rw!("rule_13"; "(pow ?x 1)" => "?x"),
rw!("rule_14"; "(pow 1 0)" => "1"),
rw!("rule_15"; "(pow 1 2)" => "1"),
rw!("rule_16"; "(pow 0 2)" => "0"),
rw!("rule_17"; "(/ ?x 1)" => "?x"),
rw!("rule_18"; "(+ ?x -1 )" => "(- ?x 1)"),
rw!("rule_19"; "(+ -1 ?x)" => "(- ?x 1)"),
rw!("rule_20"; "(* ?x -1 )" => "(- 0 ?x)"),
rw!("rule_21"; "(* -1 ?x)" => "(- 0 ?x)"),
rw!("rule_22"; "(pow 1 -1 )" => "1"),
rw!("rule_23"; "(pow -1 2)" => "(- 0 -1 )"),
rw!("rule_24"; "(+ ?x (+ ?y ?z) )" => "(+ ?y (+ ?x ?z) )"),

rw!("rule_26"; "(+ ?x (- ?y ?z) )" => "(- (+ ?x ?y) ?z)"),

rw!("rule_28"; "(+ (+ ?x ?y) ?z)" => "(+ ?x (+ ?y ?z) )"),
rw!("rule_29"; "(+ (- ?x ?y) ?z)" => "(- (+ ?x ?z) ?y)"),
rw!("rule_30"; "(* ?x (+ ?x 1) )" => "(+ ?x (pow ?x 2) )"),




rw!("rule_35"; "(* ?x (* ?y ?z) )" => "(* ?y (* ?x ?z) )"),


rw!("rule_38"; "(* (* ?x ?y) ?z)" => "(* ?x (* ?y ?z) )"),
rw!("rule_39"; "(- ?x (+ ?x 1) )" => "-1"),
rw!("rule_40"; "(- ?x (+ 1 ?x) )" => "-1"),
rw!("rule_41"; "(- 0 (+ ?x 1) )" => "(- -1 ?x)"),
rw!("rule_42"; "(- 0 (+ 1 ?x) )" => "(- -1 ?x)"),
rw!("rule_43"; "(- (+ ?x ?y) ?y)" => "?x"),
rw!("rule_44"; "(- (- ?x ?y) ?x)" => "(- 0 ?y)"),
rw!("rule_45"; "(- (- ?x ?y) 1)" => "(- ?x (+ ?y 1) )"),
rw!("rule_46"; "(- (- ?x ?y) ?z)" => "(- (- ?x ?z) ?y)"),
rw!("rule_47"; "(pow ?x (+ 1 1) )" => "(pow ?x 2)"),
rw!("rule_48"; "(pow 1 (+ ?x 1) )" => "(pow 1 ?x)"),
rw!("rule_49"; "(pow 1 (+ ?x 2) )" => "(pow 1 ?x)"),
rw!("rule_50"; "(pow 1 (+ 1 ?x) )" => "(pow 1 ?x)"),
rw!("rule_51"; "(pow 1 (+ 2 ?x) )" => "(pow 1 ?x)"),
rw!("rule_52"; "(pow 1 (- 2 1) )" => "1"),


rw!("rule_55"; "(+ (* ?x ?y) (* ?x ?z) )" => "(* ?x (+ ?y ?z) )"),
rw!("rule_56"; "(pow ?x 0)" => "1" if is_not_zero("?x")),
rw!("rule_57"; "(/ ?x ?x)" => "1" if is_not_zero("?x")),
rw!("rule_58"; "(pow ?x -1)" => "(/ 1 ?x)" if is_not_zero("?x")),
rw!("rule_59"; "(* ?x (/ ?y ?x) )" => "?y" if is_not_zero("?x")),
rw!("rule_60"; "(/ (* ?x ?y) ?x)" => "?y" if is_not_zero("?x")),
rw!("rule_61"; "(/ (* ?y ?x) ?x)" => "?y" if is_not_zero("?x")),
rw!("rule_62"; "(/ (pow ?x 2) ?x)" => "?x" if is_not_zero("?x")),
rw!("rule_63"; "(pow ?x -1 )" => "(/ 1 ?x)" if is_not_zero("?x")),
rw!("rule_64"; "(* ?y (/ 1 ?x) )" => "(/ ?y ?x)" if is_not_zero("?x")),
rw!("rule_65"; "(* (/ 1 ?x) ?y)" => "(/ ?y ?x)" if is_not_zero("?x")),
rw!("rule_66"; "(* ?y (pow ?x -1 ) )" => "(/ ?y ?x)" if is_not_zero("?x")),
rw!("rule_67"; "(* ?x (pow ?x ?y) )" => "(pow ?x (+ ?y 1) )" if is_not_zero("?x")),
rw!("rule_68"; "(* ?y (/ ?z ?x) )" => "(/ (* ?y ?z) ?x)" if is_not_zero("?x")),
rw!("rule_69"; "(* (/ ?y ?x) ?z)" => "(/ (* ?y ?z) ?x)" if is_not_zero("?x")),
rw!("rule_70"; "(/ ?y (/ ?y ?x) )" => "?x" if is_not_zero("?x") if is_not_zero("?y")),
rw!("rule_71"; "(* (pow ?x ?y) (pow ?x ?z) )" => "(pow ?x (+ ?y ?z) )" if is_not_zero("?x")),
rw!("rule_72"; "(/ (/ ?x ?y) ?x)" => "(/ 1 ?y)" if is_not_zero("?x") if is_not_zero("?y")),
rw!("rule_73"; "(/ (/ ?z ?y) ?x)" => "(/ (/ ?z ?x) ?y)" if is_not_zero("?x") if is_not_zero("?y")),
]),

]}
