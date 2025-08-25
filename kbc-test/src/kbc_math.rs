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
rw!("rule_0"; "(* (+ 1 1) ?x)" => "(+ ?x ?x)"),
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
rw!("rule_14"; "(pow 1 2)" => "1"),
rw!("rule_15"; "(pow 0 2)" => "0"),
rw!("rule_16"; "(+ ?x -1 )" => "(- ?x 1)"),
rw!("rule_17"; "(+ -1 ?x)" => "(- ?x 1)"),
rw!("rule_18"; "(* ?x -1 )" => "(- 0 ?x)"),
rw!("rule_19"; "(* -1 ?x)" => "(- 0 ?x)"),
rw!("rule_20"; "(- ?x -1 )" => "(+ ?x 1)"),
rw!("rule_21"; "(pow -1 2)" => "1"),
rw!("rule_22"; "(+ (* ?x (+ 1 1) ) ?y)" => "(+ ?x (+ ?x ?y) )"),
rw!("rule_23"; "(+ (* (+ 1 1) ?x) ?y)" => "(+ ?x (+ ?x ?y) )"),
rw!("rule_24"; "(* ?x (+ 1 ?y) )" => "(+ ?x (* ?x ?y) )"),
rw!("rule_25"; "(+ ?x (- ?y ?z) )" => "(- ?x (- ?z ?y) )"),
rw!("rule_26"; "(+ ?y (* (+ 1 1) ?x) )" => "(+ ?x (+ ?y ?x) )"),
rw!("rule_27"; "(+ ?x (+ ?x ?y) )" => "(+ ?y (+ ?x ?x) )"),
rw!("rule_28"; "(+ ?x (+ ?y ?z) )" => "(+ ?y (+ ?x ?z) )"),
rw!("rule_29"; "(+ (+ ?x ?y) ?z)" => "(+ ?x (+ ?y ?z) )"),
rw!("rule_30"; "(+ (- ?x ?y) ?z)" => "(- ?x (- ?y ?z) )"),
rw!("rule_31"; "(+ (pow ?x 2) (pow ?x 2) )" => "(* ?x (+ ?x ?x) )"),
rw!("rule_32"; "(* ?x (+ ?y ?y) )" => "(* (+ ?x ?x) ?y)"),
rw!("rule_33"; "(* ?x (+ ?y 1) )" => "(+ ?x (* ?y ?x) )"),
rw!("rule_34"; "(* ?x (+ 1 ?x) )" => "(+ ?x (pow ?x 2) )"),
rw!("rule_35"; "(* ?x (+ 1 ?y) )" => "(+ ?x (* ?y ?x) )"),
rw!("rule_36"; "(* ?x (+ 1 1) )" => "(+ ?x ?x)"),
rw!("rule_37"; "(* (pow ?x 2) ?y)" => "(* ?x (* ?x ?y) )"),
rw!("rule_38"; "(* ?x (* ?y ?z) )" => "(* ?y (* ?x ?z) )"),
rw!("rule_39"; "(* ?x (- ?x 1) )" => "(- (pow ?x 2) ?x)"),
rw!("rule_40"; "(* ?x (- ?y 1) )" => "(- (* ?y ?x) ?x)"),
rw!("rule_41"; "(* ?x (- 1 ?x) )" => "(- ?x (pow ?x 2) )"),
rw!("rule_42"; "(* ?x (- 1 ?y) )" => "(- ?x (* ?x ?y) )"),
rw!("rule_43"; "(* ?x (- 0 ?y) )" => "(- 0 (* ?x ?y) )"),
rw!("rule_44"; "(* ?x (+ ?y ?y) )" => "(* ?y (+ ?x ?x) )"),
rw!("rule_45"; "(* ?y (* (+ 1 1) ?x) )" => "(* ?x (+ ?y ?y) )"),
rw!("rule_46"; "(* ?x (pow ?y 2) )" => "(* ?y (* ?y ?x) )"),
rw!("rule_47"; "(* ?x (* ?y (+ 1 1) ) )" => "(* (+ ?x ?x) ?y)"),
rw!("rule_48"; "(* (+ ?x 1) ?y)" => "(+ ?y (* ?x ?y) )"),
rw!("rule_49"; "(* (+ 1 1) (* ?y ?x) )" => "(* (+ ?x ?x) ?y)"),
rw!("rule_50"; "(* (+ 1 ?x) ?y)" => "(+ ?y (* ?x ?y) )"),
rw!("rule_51"; "(* (* ?x ?y) ?z)" => "(* ?x (* ?y ?z) )"),
rw!("rule_52"; "(* (- ?x 1) ?y)" => "(- (* ?x ?y) ?y)"),
rw!("rule_53"; "(* (- 1 ?x) ?y)" => "(- ?y (* ?x ?y) )"),
rw!("rule_54"; "(* (- 0 ?x) ?y)" => "(- 0 (* ?x ?y) )"),
rw!("rule_55"; "(- ?x (+ ?y ?x) )" => "(- 0 ?y)"),
rw!("rule_56"; "(* ?x (- 1 ?y) )" => "(- ?x (* ?y ?x) )"),
rw!("rule_57"; "(- (* (+ 1 1) ?x) 1)" => "(- ?x (- 1 ?x) )"),
rw!("rule_58"; "(- ?x (- 0 ?y) )" => "(+ ?x ?y)"),
rw!("rule_59"; "(- 0 (* (- ?y 1) ?x) )" => "(- ?x (* ?y ?x) )"),
rw!("rule_60"; "(- ?x (- ?y ?z) )" => "(- ?z (- ?y ?x) )"),
rw!("rule_61"; "(* ?x (- -1 1) )" => "(- 0 (+ ?x ?x) )"),
rw!("rule_62"; "(* (- -1 1) ?x)" => "(- 0 (+ ?x ?x) )"),
rw!("rule_63"; "(- 0 (+ ?x 1) )" => "(- -1 ?x)"),
rw!("rule_64"; "(- 0 (+ 1 ?x) )" => "(- -1 ?x)"),
rw!("rule_65"; "(- (+ ?x ?y) ?z)" => "(- ?x (- ?z ?y) )"),
rw!("rule_66"; "(- (- ?x ?y) ?z)" => "(- ?x (+ ?y ?z) )"),
rw!("rule_67"; "(pow (- ?x ?y) 2)" => "(pow (- ?y ?x) 2)"),
// rw!("rule_68"; "0" => "1" if is_not_zero("0")),
rw!("rule_69"; "(- ?x (- -1 ?y) )" => "(+ ?x (+ ?y 1) )"),
rw!("rule_70"; "(* (+ ?x ?y) (+ 1 1) )" => "(+ ?x (+ ?x (+ ?y ?y) ) )"),
rw!("rule_71"; "(+ ?y (* ?x (+ 1 ?z) ) )" => "(+ ?x (+ ?y (* ?x ?z) ) )"),
rw!("rule_72"; "(+ ?y (* ?x (+ ?z 1) ) )" => "(+ ?x (+ ?y (* ?z ?x) ) )"),
rw!("rule_73"; "(+ ?y (* ?x (+ 1 ?z) ) )" => "(+ ?x (+ ?y (* ?z ?x) ) )"),
rw!("rule_74"; "(+ (* ?x (+ 1 ?z) ) ?y)" => "(+ ?x (+ ?y (* ?x ?z) ) )"),
rw!("rule_75"; "(+ (* (+ ?z 1) ?x) ?y)" => "(+ ?x (+ ?y (* ?x ?z) ) )"),
rw!("rule_76"; "(+ (* (+ 1 ?z) ?x) ?y)" => "(+ ?x (+ ?y (* ?x ?z) ) )"),
rw!("rule_77"; "(* ?x (+ ?y (+ ?z 1) ) )" => "(+ ?x (* ?x (+ ?y ?z) ) )"),
rw!("rule_78"; "(* ?x (+ ?y (+ 1 ?z) ) )" => "(+ ?x (* ?x (+ ?y ?z) ) )"),
rw!("rule_79"; "(+ ?x (* ?y (+ ?x 1) ) )" => "(+ ?y (* ?x (+ ?y 1) ) )"),
rw!("rule_80"; "(+ ?x (* ?y (+ ?z 1) ) )" => "(+ ?y (+ (* ?y ?z) ?x) )"),
rw!("rule_81"; "(+ ?x (* ?y (+ 1 1) ) )" => "(+ ?y (+ ?y ?x) )"),
rw!("rule_82"; "(+ ?x (* ?y (- ?z ?w) ) )" => "(- ?x (* ?y (- ?w ?z) ) )"),
rw!("rule_83"; "(* ?x (+ ?y (+ ?z 1) ) )" => "(+ ?x (* (+ ?y ?z) ?x) )"),
rw!("rule_84"; "(* ?x (+ ?y (+ 1 ?z) ) )" => "(+ ?x (* (+ ?y ?z) ?x) )"),
rw!("rule_85"; "(* ?x (- ?y (- ?z 1) ) )" => "(+ ?x (* (- ?y ?z) ?x) )"),
rw!("rule_86"; "(+ ?x (* (- ?y 1) ?x) )" => "(* ?x ?y)"),
rw!("rule_87"; "(+ ?y (* (+ ?z 1) ?x) )" => "(+ ?x (+ ?y (* ?z ?x) ) )"),
rw!("rule_88"; "(+ ?y (* (+ ?z 1) ?x) )" => "(+ ?x (+ ?y (* ?x ?z) ) )"),
rw!("rule_89"; "(+ ?y (* (+ 1 ?z) ?x) )" => "(+ ?x (+ ?y (* ?x ?z) ) )"),
rw!("rule_90"; "(* (+ ?y (+ ?z 1) ) ?x)" => "(+ ?x (* ?x (+ ?y ?z) ) )"),
rw!("rule_91"; "(* (+ ?y (+ 1 ?z) ) ?x)" => "(+ ?x (* ?x (+ ?y ?z) ) )"),
rw!("rule_92"; "(* (+ ?y (+ ?z 1) ) ?x)" => "(+ ?x (* (+ ?y ?z) ?x) )"),
rw!("rule_93"; "(+ 1 (+ 1 (+ 1 1) ) )" => "(pow (+ 1 1) 2)"),
rw!("rule_94"; "(+ (* ?x ?y) (* ?x ?y) )" => "(* (+ ?x ?x) ?y)"),
rw!("rule_95"; "(+ (* ?x ?y) (* ?x ?z) )" => "(* ?x (+ ?y ?z) )"),
rw!("rule_96"; "(+ (* ?x ?y) (* ?y ?z) )" => "(* ?y (+ ?x ?z) )"),
rw!("rule_97"; "(+ (* ?x ?y) (* ?z ?y) )" => "(* ?y (+ ?x ?z) )"),
rw!("rule_98"; "(+ (pow ?x 2) (* ?y ?x) )" => "(* ?x (+ ?x ?y) )"),
rw!("rule_99"; "(+ (* ?x (+ ?y 1) ) ?z)" => "(+ ?x (+ ?z (* ?y ?x) ) )"),
rw!("rule_100"; "(+ (* ?x (+ ?y 1) ) ?z)" => "(+ ?x (+ (* ?x ?y) ?z) )"),
rw!("rule_101"; "(* (+ 1 (+ 1 1) ) (pow ?x 2) )" => "(* ?x (+ ?x (+ ?x ?x) ) )"),
rw!("rule_102"; "(+ ?x (* (pow ?x 2) (+ 1 1) ) )" => "(* ?x (+ ?x (+ ?x 1) ) )"),
rw!("rule_103"; "(* ?x (+ ?x (+ 1 1) ) )" => "(+ ?x (+ ?x (pow ?x 2) ) )"),
rw!("rule_104"; "(* ?x (+ ?y (+ ?y 1) ) )" => "(+ ?x (* ?y (+ ?x ?x) ) )"),
rw!("rule_105"; "(* ?x (+ ?y (+ 1 1) ) )" => "(+ ?x (+ ?x (* ?y ?x) ) )"),
rw!("rule_106"; "(* ?y (+ ?x (pow ?x 2) ) )" => "(* ?x (+ ?y (* ?y ?x) ) )"),
rw!("rule_107"; "(* ?x (+ 1 (+ ?x 1) ) )" => "(+ ?x (+ ?x (pow ?x 2) ) )"),
rw!("rule_108"; "(* ?x (+ 1 (+ ?y ?y) ) )" => "(+ ?x (* ?y (+ ?x ?x) ) )"),
rw!("rule_109"; "(* ?x (+ 1 (+ 1 ?y) ) )" => "(+ ?x (+ ?x (* ?x ?y) ) )"),
rw!("rule_110"; "(* ?x (+ 1 (+ 1 ?y) ) )" => "(+ ?x (+ ?x (* ?y ?x) ) )"),
rw!("rule_111"; "(* ?x (+ 1 (+ 1 1) ) )" => "(+ ?x (+ ?x ?x) )"),
rw!("rule_112"; "(* ?x (+ 1 (* ?x ?y) ) )" => "(+ ?x (* ?y (pow ?x 2) ) )"),
rw!("rule_113"; "(* ?x (+ 1 (* ?y ?x) ) )" => "(+ ?x (* ?y (pow ?x 2) ) )"),
rw!("rule_114"; "(* ?x (+ 1 (* ?y ?z) ) )" => "(+ ?x (* ?y (* ?x ?z) ) )"),
rw!("rule_115"; "(* ?x (+ 1 (* ?y ?z) ) )" => "(+ ?x (* ?y (* ?z ?x) ) )"),
rw!("rule_116"; "(* ?x (+ 1 (pow ?y 2) ) )" => "(+ ?x (* ?y (* ?y ?x) ) )"),
rw!("rule_117"; "(* ?x (* ?x (+ 1 1) ) )" => "(+ (pow ?x 2) (pow ?x 2) )"),
rw!("rule_118"; "(* ?y (* ?z (pow ?x 2) ) )" => "(* ?x (* ?x (* ?y ?z) ) )"),
rw!("rule_119"; "(* ?x (* ?x (pow ?x 2) ) )" => "(pow (pow ?x 2) 2)"),
rw!("rule_120"; "(* ?x (* ?x (pow ?y 2) ) )" => "(pow (* ?x ?y) 2)"),
rw!("rule_121"; "(* (pow ?x 2) (* (+ 1 1) ?y) )" => "(* ?x (* ?y (+ ?x ?x) ) )"),
rw!("rule_122"; "(* ?x (* ?y (+ ?x 1) ) )" => "(* ?y (+ ?x (pow ?x 2) ) )"),
rw!("rule_123"; "(* ?x (* ?y (+ ?z ?z) ) )" => "(* ?y (* (+ ?x ?x) ?z) )"),
rw!("rule_124"; "(* ?x (* ?y (+ ?z 1) ) )" => "(* ?y (+ ?x (* ?x ?z) ) )"),
rw!("rule_125"; "(* ?x (* ?y (+ ?z 1) ) )" => "(* ?y (+ ?x (* ?z ?x) ) )"),
rw!("rule_126"; "(* ?x (* ?y (+ 1 ?z) ) )" => "(* ?y (+ ?x (* ?x ?z) ) )"),
rw!("rule_127"; "(* ?x (* ?y (+ 1 ?z) ) )" => "(* ?y (+ ?x (* ?z ?x) ) )"),
rw!("rule_128"; "(* ?x (* ?y (+ 1 1) ) )" => "(* ?y (+ ?x ?x) )"),
rw!("rule_129"; "(* ?x (* (+ ?y ?y) ?z) )" => "(* (+ ?x ?x) (* ?y ?z) )"),
rw!("rule_130"; "(* ?x (* (+ ?y ?y) ?z) )" => "(* (+ ?x ?x) (* ?z ?y) )"),
rw!("rule_131"; "(* ?x (* (+ ?y 1) ?z) )" => "(* ?z (+ ?x (* ?x ?y) ) )"),
rw!("rule_132"; "(* ?x (* (+ 1 1) ?x) )" => "(+ (pow ?x 2) (pow ?x 2) )"),
rw!("rule_133"; "(* ?x (* (+ 1 1) ?y) )" => "(* (+ ?x ?x) ?y)"),
rw!("rule_134"; "(* ?x (* (pow ?y 2) ?x) )" => "(pow (* ?x ?y) 2)"),
rw!("rule_135"; "(* ?x (* (pow ?y 2) ?z) )" => "(* ?y (* ?y (* ?x ?z) ) )"),
rw!("rule_136"; "(* ?x (* (pow ?y 2) ?z) )" => "(* ?y (* ?y (* ?z ?x) ) )"),
rw!("rule_137"; "(* ?x (- ?y (* ?z ?y) ) )" => "(* ?y (- ?x (* ?x ?z) ) )"),
rw!("rule_138"; "(* ?x (pow (+ 1 1) 2) )" => "(* (+ 1 1) (+ ?x ?x) )"),
rw!("rule_139"; "(* ?y (+ (pow ?x 2) (pow ?x 2) ) )" => "(* ?x (* ?y (+ ?x ?x) ) )"),
rw!("rule_140"; "(* ?x (+ ?y (* ?z ?y) ) )" => "(* ?y (+ ?x (* ?z ?x) ) )"),
rw!("rule_141"; "(* ?x (+ ?y (* ?y ?z) ) )" => "(* ?y (+ ?x (* ?z ?x) ) )"),
rw!("rule_142"; "(* ?x (+ ?y (* ?y ?z) ) )" => "(* ?y (+ ?x (* ?x ?z) ) )"),
rw!("rule_143"; "(* ?x (* ?y (+ ?z ?z) ) )" => "(* ?z (* ?y (+ ?x ?x) ) )"),
rw!("rule_144"; "(* ?x (* ?y (+ ?z ?z) ) )" => "(* ?z (* (+ ?x ?x) ?y) )"),
rw!("rule_145"; "(* ?x (* ?y (+ ?z ?z) ) )" => "(* ?y (* (+ ?z ?z) ?x) )"),
rw!("rule_146"; "(* ?x (* ?y (+ ?z 1) ) )" => "(* ?y (* (+ ?z 1) ?x) )"),
rw!("rule_147"; "(* (+ ?x ?x) (pow ?y 2) )" => "(* ?y (* ?x (+ ?y ?y) ) )"),
rw!("rule_148"; "(* (+ ?x 1) (* ?y ?z) )" => "(* ?y (+ ?z (* ?x ?z) ) )"),
rw!("rule_149"; "(* (+ ?x 1) (* ?y ?z) )" => "(* ?y (+ ?z (* ?z ?x) ) )"),
rw!("rule_150"; "(* (+ ?x 1) (pow ?y 2) )" => "(* ?y (+ ?y (* ?x ?y) ) )"),
rw!("rule_151"; "(* (+ ?x 1) (pow ?y 2) )" => "(* ?y (+ ?y (* ?y ?x) ) )"),
rw!("rule_152"; "(* (+ 1 ?x) (* ?y ?z) )" => "(* ?y (+ ?z (* ?z ?x) ) )"),
rw!("rule_153"; "(* (+ 1 ?x) (pow ?y 2) )" => "(* ?y (+ ?y (* ?y ?x) ) )"),
rw!("rule_154"; "(* (+ 1 1) (+ ?x 1) )" => "(+ 1 (+ 1 (+ ?x ?x) ) )"),
rw!("rule_155"; "(* (+ 1 1) (* ?x ?y) )" => "(* ?x (+ ?y ?y) )"),
rw!("rule_156"; "(* (+ 1 1) (* ?x ?y) )" => "(* (+ ?x ?x) ?y)"),
rw!("rule_157"; "(* (+ 1 1) (pow ?x 2) )" => "(* ?x (+ ?x ?x) )"),
rw!("rule_158"; "(* (pow ?x 2) (+ ?x 1) )" => "(* ?x (+ ?x (pow ?x 2) ) )"),
rw!("rule_159"; "(* (pow ?x 2) (+ ?y ?y) )" => "(* ?x (* ?y (+ ?x ?x) ) )"),
rw!("rule_160"; "(* (pow ?x 2) (+ ?y ?y) )" => "(* ?y (* ?x (+ ?x ?x) ) )"),
rw!("rule_161"; "(* (pow ?x 2) (+ ?y 1) )" => "(* ?x (+ ?x (* ?y ?x) ) )"),
rw!("rule_162"; "(* (pow ?x 2) (+ ?y 1) )" => "(* ?x (* (+ ?y 1) ?x) )"),
rw!("rule_163"; "(* (pow ?x 2) (+ 1 ?y) )" => "(* ?x (+ ?x (* ?x ?y) ) )"),
rw!("rule_164"; "(* (pow ?x 2) (+ 1 1) )" => "(* ?x (+ ?x ?x) )"),
rw!("rule_165"; "(* (pow ?x 2) (pow ?y 2) )" => "(pow (* ?x ?y) 2)"),
rw!("rule_166"; "(* (+ ?x (+ ?x 1) ) ?y)" => "(+ ?y (* ?x (+ ?y ?y) ) )"),
rw!("rule_167"; "(* (+ ?x (+ 1 1) ) ?y)" => "(+ ?y (+ ?y (* ?x ?y) ) )"),
rw!("rule_168"; "(* (+ ?x (+ 1 1) ) ?y)" => "(+ ?y (+ ?y (* ?y ?x) ) )"),
rw!("rule_169"; "(* (+ ?x (* ?x ?y) ) ?z)" => "(* ?x (* ?z (+ ?y 1) ) )"),
rw!("rule_170"; "(* (+ ?x (* ?y ?x) ) ?z)" => "(* ?x (+ ?z (* ?z ?y) ) )"),
rw!("rule_171"; "(* (+ ?x (* ?y ?x) ) ?z)" => "(* ?x (* (+ ?y 1) ?z) )"),
rw!("rule_172"; "(* (+ ?x (pow ?x 2) ) ?y)" => "(* ?x (* (+ ?x 1) ?y) )"),
rw!("rule_173"; "(* (+ 1 (+ ?x 1) ) ?y)" => "(+ ?y (+ ?y (* ?y ?x) ) )"),
rw!("rule_174"; "(* (+ 1 (+ 1 1) ) ?x)" => "(+ ?x (+ ?x ?x) )"),
rw!("rule_175"; "(* (+ 1 (* ?x ?y) ) ?z)" => "(+ ?z (* ?x (* ?z ?y) ) )"),
rw!("rule_176"; "(* (+ 1 (pow ?x 2) ) ?y)" => "(+ ?y (* ?x (* ?x ?y) ) )"),
rw!("rule_177"; "(- ?x (+ ?y (+ ?x ?x) ) )" => "(- 0 (+ ?x ?y) )"),
rw!("rule_178"; "(- ?x (+ ?y (+ ?x ?y) ) )" => "(* ?y (- -1 1) )"),
rw!("rule_179"; "(- ?x (* ?x (+ ?y 1) ) )" => "(- 0 (* ?x ?y) )"),
rw!("rule_180"; "(* ?x (- ?z (- ?y 1) ) )" => "(- ?x (* ?x (- ?y ?z) ) )"),
rw!("rule_181"; "(- ?x (* ?x (- 1 ?y) ) )" => "(* ?x ?y)"),
rw!("rule_182"; "(- ?x (* (+ ?y 1) ?x) )" => "(- 0 (* ?x ?y) )"),
rw!("rule_183"; "(- ?x (* (+ 1 ?y) ?x) )" => "(- 0 (* ?x ?y) )"),
rw!("rule_184"; "(- ?x (* (- 1 ?y) ?x) )" => "(* ?x ?y)"),
rw!("rule_185"; "(- ?x (- ?x (+ ?y 1) ) )" => "(+ ?y 1)"),
rw!("rule_186"; "(- ?x (- ?x (* ?x ?y) ) )" => "(* ?x ?y)"),
rw!("rule_187"; "(* (- ?x ?y) (+ 1 1) )" => "(- ?x (- ?y (- ?x ?y) ) )"),
rw!("rule_188"; "(- ?x (- ?y (- ?z ?x) ) )" => "(- ?z ?y)"),
rw!("rule_189"; "(- ?x (- ?y (+ ?z ?w) ) )" => "(- ?z (- ?y (+ ?x ?w) ) )"),
rw!("rule_190"; "(* (- ?z (- ?y 1) ) ?x)" => "(- ?x (* ?x (- ?y ?z) ) )"),
rw!("rule_191"; "(- ?x (- ?y (- ?z ?w) ) )" => "(- ?z (- ?y (- ?x ?w) ) )"),
rw!("rule_192"; "(- 0 (+ ?x (+ ?y 1) ) )" => "(- -1 (+ ?x ?y) )"),
rw!("rule_193"; "(- 0 (+ ?x (+ 1 ?y) ) )" => "(- -1 (+ ?x ?y) )"),
rw!("rule_194"; "(- 0 (+ ?x (* ?y ?x) ) )" => "(* ?x (- -1 ?y) )"),
rw!("rule_195"; "(- 0 (+ ?x (pow ?x 2) ) )" => "(* ?x (- -1 ?x) )"),
rw!("rule_196"; "(- 0 (* ?x (+ ?y 1) ) )" => "(* ?x (- -1 ?y) )"),
rw!("rule_197"; "(- 0 (* ?x (+ 1 ?y) ) )" => "(* ?x (- -1 ?y) )"),
rw!("rule_198"; "(- 0 (* ?x (- ?y ?z) ) )" => "(* ?x (- ?z ?y) )"),
rw!("rule_199"; "(- 0 (* (+ ?x 1) ?y) )" => "(* ?y (- -1 ?x) )"),
rw!("rule_200"; "(- 0 (* (+ 1 ?x) ?y) )" => "(* ?y (- -1 ?x) )"),
rw!("rule_201"; "(- (* ?x ?y) (* ?x ?z) )" => "(* ?x (- ?y ?z) )"),
rw!("rule_202"; "(- (pow ?x 2) (* ?x ?y) )" => "(* ?x (- ?x ?y) )"),
rw!("rule_203"; "(- (* ?x (+ ?y 1) ) ?z)" => "(- ?x (- ?z (* ?x ?y) ) )"),
rw!("rule_204"; "(- (* ?x (+ 1 ?y) ) ?x)" => "(* ?x ?y)"),
rw!("rule_205"; "(- (* ?x (+ 1 1) ) ?y)" => "(- ?x (- ?y ?x) )"),
rw!("rule_206"; "(- (* ?x (- 1 ?y) ) ?x)" => "(- 0 (* ?x ?y) )"),
rw!("rule_207"; "(- (* (+ ?x 1) ?y) ?y)" => "(* ?x ?y)"),
rw!("rule_208"; "(- (* (+ 1 ?x) ?y) ?y)" => "(* ?x ?y)"),
rw!("rule_209"; "(/ 0 ?x)" => "0" if is_not_zero("?x")),
rw!("rule_210"; "(pow ?x 0)" => "1" if is_not_zero("?x")),
rw!("rule_211"; "(/ ?x ?x)" => "1" if is_not_zero("?x")),
rw!("rule_212"; "(+ ?x (+ (pow ?x 2) (pow ?x 2) ) )" => "(* ?x (+ ?x (+ ?x 1) ) )"),
rw!("rule_213"; "(+ (* ?x ?y) (+ ?z (* ?x ?w) ) )" => "(+ ?z (* ?x (+ ?y ?w) ) )"),
rw!("rule_214"; "(+ (pow ?x 2) (+ ?y (pow ?x 2) ) )" => "(+ ?y (* ?x (+ ?x ?x) ) )"),
rw!("rule_215"; "(+ (pow ?x 2) (+ (pow ?x 2) ?y) )" => "(+ (* ?x (+ ?x ?x) ) ?y)"),
rw!("rule_216"; "(* ?x (+ (pow ?y 2) (pow ?y 2) ) )" => "(* ?y (* ?y (+ ?x ?x) ) )"),
rw!("rule_217"; "(- (pow (+ ?x 1) 2) (+ ?x 1) )" => "(+ ?x (pow ?x 2) )"),
rw!("rule_218"; "(* ?x (/ 1 ?x) )" => "1" if is_not_zero("?x")),
rw!("rule_219"; "(pow ?x -1 )" => "(/ 1 ?x)" if is_not_zero("?x")),
rw!("rule_220"; "(- ?x (- (* (+ ?x ?x) (- 1 ?y) ) ?x) )" => "(* (+ ?x ?x) ?y)"),
rw!("rule_221"; "(* ?y (pow ?x -1 ) )" => "(/ ?y ?x)" if is_not_zero("?x")),
rw!("rule_222"; "(* (pow ?x ?y) (pow ?x ?z) )" => "(pow ?x (+ ?y ?z) )" if is_not_zero("?x")),
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
