use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    process::id,
    time::Instant,
};

// Suggested by ChatGPT to avoid recomputing precedence map
use once_cell::sync::Lazy;
static OP_PRECEDENCE_MAP: Lazy<HashMap<&'static str, usize>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for (i, &op) in ["+", "-", "*", "/", "pow"].iter().enumerate() {
        map.insert(op, i);
    }
    map
});

#[derive(Clone, Debug, PartialEq)]
struct Symbol {
    name: String,
    is_var: bool,
    is_num: bool,
    length: usize,
}

#[derive(Clone, Debug)]
pub struct Rule {
    name: String,
    lhs: Vec<Symbol>,
    rhs: Vec<Symbol>,
    cond: Vec<Symbol>,
}

/* Merges two sets of variable substitutions.
If a variable is bound in both, the bindings must be identical.
Returns None if there is a conflict.
*/
fn merge_subst<'a>(
    mut s1: HashMap<&'a str, (usize, usize)>,
    s2: HashMap<&'a str, (usize, usize)>,
) -> Option<HashMap<&'a str, (usize, usize)>> {
    for (var, val2) in s2 {
        if let Some(val1) = s1.get(&var) {
            if val1 != &val2 {
                return None;
            }
        } else {
            s1.insert(var, val2);
        }
    }
    Some(s1)
}

/* Unifies two terms, e.g.LHS of a rule and the term to rewrite.
Works recursively top-down and left-right on expression tree.
Returns None if not unifiable, or a mapping from variables to subterms.
Only works on binary operators, constants, and variables.*/
// fn unify<'a>(
//     lhs: &'a [Symbol],
//     term: &'a [Symbol],
//     parent_match: bool,
// ) -> Option<(HashMap<&'a str, (usize, usize)>, usize)> {
//     // println!(
//     //     "Trying to match {} on {}",
//     //     debug_print(&lhs.to_vec()),
//     //     debug_print(&term.to_vec())
//     // );

//     if lhs[0].length == 1 {
//         // Base cases variable or constant
//         if lhs[0].is_var {
//             let mut map = HashMap::with_capacity(lhs.len());
//             map.insert(lhs[0].name.as_str(), term);
//             return Some((map, 0));
//         } else if lhs[0].name == term[0].name {
//             return Some((Default::default(), 0));
//         } else {
//             return None;
//         }
//     } else if lhs[0].length > term.len() {
//         // Pattern longer than term
//         return None;
//     } else {
//         // Same root symbol, try to match children
//         if lhs[0].name == term[0].name {
//             let lhs_idx = lhs[1].length;
//             let term_idx = term[1].length;
//             let rest_lhs = &lhs[1..lhs_idx + 1];
//             let term_lhs = &term[1..term_idx + 1];
//             if let Some(map) = unify(&rest_lhs, &term_lhs, true) {
//                 // LHS matched
//                 // println!("Map1: {:?}", map);
//                 let rest_rhs = &lhs[lhs_idx + 1..];
//                 let term_rhs = &term[term_idx + 1..];
//                 if let Some(rest_map) = unify(&rest_rhs, &term_rhs, true) {
//                     // RHS matched
//                     // println!("Map2: {:?}", rest_map);
//                     if let Some(merged) = merge_subst(map.0, rest_map.0) {
//                         // Merged successfully
//                         // println!("Merged: {:?}", merged);
//                         return Some((merged, 0));
//                     }
//                 }
//             }
//         }
//         if !parent_match {
//             // Descend into term to match rule on subterms.
//             if let Some(map) = unify(lhs, &term[1..(term[1].length + 1)], parent_match) {
//                 // println!("Map 3: {:?}", map);
//                 return Some((map.0, 1 + map.1));
//             } else {
//                 if let Some(map) = unify(lhs, &term[(term[1].length + 1)..], parent_match) {
//                     // println!("Map 4: {:?}", map);
//                     return Some((map.0, term[1].length + 1 + map.1));
//                 } else {
//                     return None;
//                 }
//             }
//         }
//     }
//     return None;
// }

fn unify<'a>(
    lhs: &'a [Symbol],
    term: &'a [Symbol],
    map: &mut HashMap<String, (usize, usize)>,
    index: Option<usize>,
) -> Option<usize> {
    // println!(
    //     "Trying to match {} on {}",
    //     debug_print(&lhs.to_vec()),
    //     debug_print(&term.to_vec())
    // );

    let mut roots = Vec::new();
    if let Some(offset) = index {
        if term[offset].name == lhs[0].name && term[offset].length >= lhs[0].length {
            roots.push(offset);
        } else {
            return None;
        }
    } else {
        for (i, sym) in term.iter().enumerate() {
            if sym.name == lhs[0].name && sym.length >= lhs[0].length {
                roots.push(i);
            }
        }
    }

    for root in roots {
        // println!("Trying root at index {}, {}", root, index.is_some());
        let mut i = 1;
        let mut j = root + 1;
        let end = root + term[root].length;
        while j < end {
            // println!(
            //     "Matching lhs[{}] = {:?} with term[{}] = {:?}",
            //     i, lhs[i], j, term[j]
            // );
            if lhs[i].is_num {
                if term[j].is_num && lhs[i].name == term[j].name {
                    i += 1;
                    j += 1;
                } else {
                    map.clear();
                    break;
                }
            } else if lhs[i].is_var {
                let var_name = lhs[i].name.as_str();
                let sub_start = j;
                let sub_length = term[j].length;
                let sub = &term[sub_start..sub_start + sub_length];
                if let Some(existing) = map.get(var_name) {
                    if &term[existing.0..existing.1] != sub {
                        map.clear();
                        break;
                    }
                } else {
                    map.insert(var_name.to_string(), (sub_start, sub_start + sub_length));
                }
                i += 1;
                j += sub_length;
            } else {
                if lhs[i].name == term[j].name {
                    i += 1;
                    j += 1;
                } else {
                    map.clear();
                    break;
                }
            }
        }
        if i == lhs.len() && j == end {
            // println!("Unified with map: {:?}", map);
            return Some(root);
        }
        map.clear();
    }
    return None;
}

// Makes sure that all conditions on variables are satisfied.
// Currently only checks division by 0.
fn check_conditions<'a>(
    cond: &Vec<Symbol>,
    term: &'a [Symbol],
    subst: &HashMap<String, (usize, usize)>,
) -> bool {
    for var in cond {
        // println!("Checking condition on variable: {}", var.name);
        if let Some(idxs) = subst.get(var.name.as_str()) {
            if term[idxs.0].length == 1 && term[idxs.0].name == "0" {
                return false;
            }
        }
    }
    true
}

// Inserts new_term into old_term at index idx, adjusting lengths of parent terms.
fn insert(new_term: Vec<Symbol>, old_term: &mut Vec<Symbol>, idx: usize) -> Vec<Symbol> {
    // println!("Old term: {:?}", debug_print(old_term));
    // println!("New term: {:?}", debug_print(&new_term));
    // let mut i = 0;
    // let diff = old_term[idx].length - new_term.len();
    // while i < idx {
    //     if old_term[i].length != 1 {
    //         old_term[i].length -= diff;
    //     }
    //     i += 1;
    //     if old_term[i].length < idx - i + 1 {
    //         i += old_term[i].length;
    //     }
    // }
    let mut result = old_term[..idx].to_vec();
    result.extend(new_term);
    // println!("Inserted at index {}: {:?}", idx, debug_print(&result));
    result.extend(old_term[idx + old_term[idx].length..].to_vec());
    // println!("After insertion: {:?}", debug_print(&result));
    let mut i = result.len();
    while i > 0 {
        // println!(
        //     "Adjusting lengths, at index {}: {}",
        //     i - 1,
        //     debug_print(&result)
        // );
        i -= 1;
        if result[i].length > 1 {
            let arg1_len = result[i + 1].length;
            let arg2_len = result[i + 1 + arg1_len].length;
            result[i].length = 1 + arg1_len + arg2_len;
        }
    }
    // println!("Result: {:?}", debug_print(&result));
    result
}

// Applies a substitution to a term, replacing variables with their bindings.
// fn apply_subst<'a>(term: &[Symbol], subst: &HashMap<&str, &'a [Symbol]>) -> Vec<Symbol> {
//     fn apply_rec<'a>(
//         term: &[Symbol],
//         subst: &HashMap<&str, &'a [Symbol]>,
//         out: &mut Vec<Symbol>,
//     ) -> usize {
//         let first = &term[0];

//         if first.length == 1 {
//             if first.is_var {
//                 if let Some(&repl) = subst.get(first.name.as_str()) {
//                     out.extend_from_slice(repl);
//                     return repl.len();
//                 }
//             }
//             out.push(first.clone());
//             return 1;
//         }
//         let mut length = 1;
//         let left = &term[1..1 + term[1].length];
//         let right = &term[1 + term[1].length..];

//         let start_idx = out.len();
//         out.push(first.clone());
//         length += apply_rec(left, subst, out);
//         length += apply_rec(right, subst, out);
//         out[start_idx].length = length;

//         length
//     }
//     let mut result = Vec::with_capacity(term.len() * 2);
//     apply_rec(term, subst, &mut result);
//     result
// }

fn apply_subst<'a>(
    term: &[Symbol],
    original: &[Symbol],
    subst: &HashMap<String, (usize, usize)>,
) -> Vec<Symbol> {
    // println!("Original term: {:?}", debug_print(&original.to_vec()));
    // println!("Substitution: {:?}", subst);
    // println!("Applying subst to term: {:?}", debug_print(&term.to_vec()));
    let length = term.len();
    let mut result = Vec::with_capacity(length * 2);
    let mut i = 0;
    let mut update = Vec::new();
    while i < length {
        let sym = &term[i];
        if sym.is_var {
            if let Some(&(start, end)) = subst.get(sym.name.as_str()) {
                result.extend_from_slice(&original[start..end]);
            } else {
                result.push(sym.clone());
            }
        } else {
            let current = sym.clone();
            if current.length > 1 {
                // println!(
                //     "Marking for update at index {}: {:?}, sym: {:?}",
                //     i,
                //     debug_print(&result),
                //     current
                // );
                update.push(result.len());
            }
            result.push(current);
        }
        i += 1;
    }
    // println!("Applied subst: {:?}", debug_print(&result));
    // println!("To update: {:?}", update);
    while let Some(idx) = update.pop() {
        let arg1_len = result[idx + 1].length;
        let arg2_len = result[idx + 1 + arg1_len].length;
        result[idx].length = 1 + arg1_len + arg2_len;
        // println!(
        //     "Updated length at index {}: {:?}, sym: {:?}",
        //     idx,
        //     debug_print(&result),
        //     result[idx]
        // );
    }
    result
}

// Tries to apply each rule in order to the term, returning the first successful rewrite.
fn rewrite_one_step<'a>(
    rules: &'a [Rule],
    term: &mut Vec<Symbol>,
    map: &mut HashMap<String, (usize, usize)>,
) -> Option<(Vec<Symbol>, &'a str)> {
    for rule in rules {
        // println!("Trying rule: {}", rule.name);
        // println!("On term: {:?}", debug_print(term));
        if let Some(idx) = unify(&rule.lhs, term, map, None) {
            if !check_conditions(&rule.cond, term, map) {
                map.clear();
                continue;
            }
            let new_term = apply_subst(&rule.rhs, term, map);
            map.clear();
            let rewritten = insert(new_term, term, idx);
            // println!("Rewritten term: {:?}", debug_print(&rewritten));
            return Some((rewritten, rule.name.as_str()));
        }
    }
    None
}

// Folds constant expressions in the term, e.g. (+ 2 3) -> 5
fn fold_constants(term: &mut Vec<Symbol>) {
    let mut i = term.len();
    while i > 0 {
        i -= 1;
        let sym = &term[i];
        if sym.length != 3 {
            continue;
        }
        let lhs = &term[i + 1];
        let rhs = &term[i + 2];
        if lhs.is_num && rhs.is_num {
            // println!("Folding constants: {} {} {}", lhs.name, sym.name, rhs.name);
            let folded = match sym.name.as_str() {
                "+" => lhs.name.parse::<f64>().unwrap() + rhs.name.parse::<f64>().unwrap(),
                "-" => lhs.name.parse::<f64>().unwrap() - rhs.name.parse::<f64>().unwrap(),
                "*" => lhs.name.parse::<f64>().unwrap() * rhs.name.parse::<f64>().unwrap(),
                "/" => {
                    let denom = rhs.name.parse::<f64>().unwrap();
                    if denom == 0.0 {
                        continue;
                    }
                    lhs.name.parse::<f64>().unwrap() / denom
                }
                _ => continue,
            };
            term.splice(
                i..i + 3,
                [Symbol {
                    name: folded.to_string(),
                    length: 1,
                    is_var: false,
                    is_num: true,
                }],
            );
            let mut j = 0;
            while j < i {
                if term[j].length > 1 && j + term[j].length > i {
                    term[j].length -= 2;
                }
                j += 1;
            }
        }
    }
}

// Implements lexicographic ordering with w(var) = 1, w(const) = 2
// Pushes constants to the right for constant folding
fn smaller(a: &[Symbol], b: &[Symbol]) -> bool {
    let weight = |sym: &Symbol| {
        if sym.is_num {
            2
        } else {
            if sym.length > 1 { 3 } else { 1 }
        }
    };
    let mut a_weight = 0;
    let mut b_weight = 0;
    for sym in a {
        a_weight += weight(sym);
    }
    for sym in b {
        b_weight += weight(sym);
    }
    // println!(
    //     "Comparing weights: w({:?}) = {}, w({:?}) = {}",
    //     debug_print(&a.to_vec()),
    //     a_weight,
    //     debug_print(&b.to_vec()),
    //     b_weight
    // );
    if a_weight < b_weight {
        return true;
    } else if a_weight > b_weight {
        return false;
    } else {
        for (sym_a, sym_b) in a.iter().zip(b.iter()) {
            let w_a = weight(sym_a);
            let w_b = weight(sym_b);
            // println!(
            //     "Comparing symbols: w({}) = {}, w({}) = {}",
            //     sym_a.name, w_a, sym_b.name, w_b
            // );
            let prec_a = *OP_PRECEDENCE_MAP
                .get(sym_a.name.as_str())
                .unwrap_or(&usize::MAX);
            let prec_b = *OP_PRECEDENCE_MAP
                .get(sym_b.name.as_str())
                .unwrap_or(&usize::MAX);
            if w_a < w_b {
                // println!(
                //     "Smaller: {:?} < {:?}",
                //     debug_print(&a.to_vec()),
                //     debug_print(&b.to_vec())
                // );
                return true;
            } else if w_a > w_b {
                // println!(
                //     "Not smaller: {:?} > {:?}",
                //     debug_print(&a.to_vec()),
                //     debug_print(&b.to_vec())
                // );
                return false;
            } else if prec_a < prec_b {
                // println!(
                //     "Smaller by precedence: {:?} < {:?}",
                //     debug_print(&a.to_vec()),
                //     debug_print(&b.to_vec())
                // );
                return true;
            } else if prec_a > prec_b {
                // println!(
                //     "Not smaller by precedence: {:?} > {:?}",
                //     debug_print(&a.to_vec()),
                //     debug_print(&b.to_vec())
                // );
                return false;
            }
        }
        return false;
    }
}

// Applies all canonicalization rules to the term until no more apply.
// Only accepts rewrites that make the term lexicographically smaller.
fn canonicalize<'a>(
    old: &'a mut Vec<Symbol>,
    canonicalizers: &'a [Rule],
    map: &mut HashMap<String, (usize, usize)>,
) -> bool {
    // println!("Canonicalizing: {}", debug_print(old));
    let mut canonicalized = false;
    for rule in canonicalizers {
        // println!(
        //     "Trying canonicalization rule: {} on {}",
        //     rule.name,
        //     debug_print(old)
        // );
        let mut i = old.len();
        while i > 0 {
            i -= 1;
            // println!("rule {} on {} at index {}", rule.name, debug_print(old), i);
            if let Some(idx) = unify(&rule.lhs, &old, map, Some(i)) {
                if !check_conditions(&rule.cond, old, map) {
                    map.clear();
                    continue;
                }
                let new_term = apply_subst(&rule.rhs, old, map);
                let abs_start = idx;
                let abs_end = abs_start + old[abs_start].length;
                map.clear();
                if smaller(&new_term, &old[abs_start..abs_end]) {
                    // println!(
                    //     "Canonicalizing with rule {} at index {}: {} -> {}",
                    //     rule.name,
                    //     i,
                    //     debug_print(&old[abs_start..abs_end].to_vec()),
                    //     debug_print(&new_term)
                    // );
                    *old = insert(new_term, old, idx);
                    canonicalized = true;
                }
            }
        }
    }
    // println!("Became {}", debug_print(old));
    canonicalized
}

fn debug_print(syms: &Vec<Symbol>) -> String {
    let mut ret = String::new();
    for sym in syms {
        ret.push_str(format!("[{}:{}]", sym.name, sym.length).as_str());
    }
    return ret;
}

// Repeatedly applies rewriting and canonicalization until no more changes occur.
fn rewrite<'a>(
    rules: &'a [Rule],
    canonicalizers: &'a [Rule],
    term: &[Symbol],
    map: &mut HashMap<String, (usize, usize)>,
) -> (Vec<Symbol>, Vec<&'a str>) {
    let mut applied_rules = Vec::new();
    let mut current_term = term.to_vec();
    let mut canonicalizable = true;
    while canonicalizable {
        while let Some((new_term, rule_name)) = rewrite_one_step(rules, &mut current_term, map) {
            applied_rules.push(rule_name);
            current_term = new_term;
            fold_constants(&mut current_term);
        }
        canonicalizable = canonicalize(&mut current_term, canonicalizers, map);
        fold_constants(&mut current_term);
    }
    // println!("rewritten: {}", debug_print(&current_term));
    (current_term, applied_rules)
}

fn parse_term(term_str: &str) -> Vec<Symbol> {
    let mut symbols = vec![];
    let term_str = term_str
        .replace("\"", "")
        .replace(")", " )")
        .replace("(", "( ");
    let parts: Vec<&str> = term_str.split_whitespace().collect();
    let mut i = 0;
    let mut stack = vec![];
    for part in parts {
        if part == "(" {
            stack.push(i);
        } else if part == ")" {
            stack.pop();
        } else {
            if part.parse::<f64>().is_ok() {
                symbols.push(Symbol {
                    name: part.to_string(),
                    is_var: false,
                    is_num: true,
                    length: 1,
                });
            } else if part.chars().next().unwrap() == '?' {
                symbols.push(Symbol {
                    name: part[1..].to_ascii_uppercase().to_string(),
                    is_var: true,
                    is_num: false,
                    length: 1,
                });
            } else {
                symbols.push(Symbol {
                    name: part.to_string(),
                    is_var: false,
                    is_num: false,
                    length: 1,
                });
            }
            for idx in stack.iter() {
                if idx != &i {
                    symbols[*idx].length += 1;
                }
            }
            i += 1;
        }
    }
    symbols
}

fn parse_conditions(cond_str: &str) -> Vec<Symbol> {
    let mut conditions = vec![];
    let parts: Vec<&str> = cond_str.split("if").collect();
    for part in parts {
        let var_pos = part.find("?").unwrap() + 1;
        let var = part.chars().nth(var_pos).unwrap();
        conditions.push(Symbol {
            name: var.to_ascii_uppercase().to_string(),
            is_var: true,
            is_num: false,
            length: 1,
        });
    }
    conditions
}

/* Checks if a rule is a canonicalizer:
    - LHS should be at least as long as RHS
    - LHS must contain all variables in RHS with at least same multiplicity
*/
fn check_canonicalizer(rule: &Rule) -> bool {
    if rule.lhs.len() <= rule.rhs.len() {
        return true;
    }
    return false;
    // if rule.lhs.len() != rule.rhs.len() {
    //     return false;
    // }
    // let mut lhs_vars = HashMap::new();
    // let mut rhs_vars = HashMap::new();
    // for i in 0..rule.lhs.len() {
    //     let lhs_sym = &rule.lhs[i];
    //     let rhs_sym = &rule.rhs[i];

    //     if lhs_sym.length == 1 {
    //         *lhs_vars.entry(lhs_sym.name.as_str()).or_insert(0) += 1;
    //     }
    //     if rhs_sym.length == 1 {
    //         *rhs_vars.entry(rhs_sym.name.as_str()).or_insert(0) += 1;
    //     }
    // }
    // lhs_vars == rhs_vars
}

fn parse_rules(egg_rules: &Vec<String>) -> (Vec<Rule>, Vec<Rule>) {
    let mut rules = vec![];
    let mut canonicalizers = vec![];
    for line in egg_rules {
        if line.trim().is_empty() || line.trim().starts_with("//") {
            continue;
        }
        let parts: Vec<&str> = line.split("=>").collect();
        let lhs_parts = parts[0].trim().split(';').collect::<Vec<&str>>();
        let name = lhs_parts[0].replace("rw!(", "").replace("\"", "");
        let lhs_str = lhs_parts[1].trim();
        let rhs_cond_str = parts[1].trim();
        let rhs_parts: Vec<&str> = rhs_cond_str.split("if").collect();
        let rhs_str = rhs_parts[0].trim().replace(",", "");
        let cond_str = if rhs_parts.len() > 1 {
            rhs_parts[1].trim()
        } else {
            ""
        };

        let mut lhs = parse_term(&lhs_str);
        let mut rhs = parse_term(&rhs_str);
        fold_constants(&mut lhs);
        fold_constants(&mut rhs);
        let cond = if cond_str.is_empty() {
            vec![]
        } else {
            parse_conditions(&cond_str)
        };
        let rule = Rule {
            name,
            lhs,
            rhs,
            cond,
        };
        if check_canonicalizer(&rule) {
            if rule.lhs.len() < rule.rhs.len() {
                continue;
            }
            canonicalizers.push(rule);
        } else {
            rules.push(rule);
        }
    }
    (rules, canonicalizers)
}

#[derive(serde::Serialize)]
struct OutputEntry {
    original_term: String,
    rewritten_term: String,
    applied_rules: Vec<String>,
}

fn sym_count(term: String) -> usize {
    let clean = term.replace('(', " ").replace('"', " ").replace(')', " ");
    return clean
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .len();
}

fn pretty(term: &Vec<Symbol>) -> String {
    if term.is_empty() {
        return "".to_string();
    }
    if term[0].length == 1 {
        return term[0].name.clone();
    } else {
        return format!(
            "({} {} {})",
            term[0].name,
            pretty(&term[1..term[1].length + 1].to_vec()),
            pretty(&term[term[1].length + 1..].to_vec())
        );
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("Usage: {} <rules-file> <term-file>", args[0]);
        std::process::exit(1);
    }
    let rules_file = &args[1];
    let term_file = &args[2];
    let out_dir = std::path::Path::new("results/all");
    let name = std::path::Path::new(rules_file)
        .file_stem()
        .unwrap()
        .to_string_lossy();
    let input_file = std::path::Path::new(term_file)
        .file_stem()
        .unwrap()
        .to_string_lossy();
    let out_path = out_dir.join(format!("Greedy-{}-{}.jsonl", name, input_file));
    let file = File::open(rules_file).map_err(|e| {
        eprintln!("Failed to open input file '{}': {}", rules_file, e);
        e
    })?;
    let reader = BufReader::new(file);

    let egg_rules: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let (mut rules, canonicalizers) = parse_rules(&egg_rules);

    rules.sort_by(|a, b| {
        let score_a = 1.0 - (a.rhs.len() as f32 / a.lhs.len() as f32);
        let score_b = 1.0 - (b.rhs.len() as f32 / b.lhs.len() as f32);
        score_b.partial_cmp(&score_a).unwrap()
    });

    let file = File::open(term_file).map_err(|e| {
        eprintln!("Failed to open term file '{}': {}", term_file, e);
        e
    })?;
    let reader = BufReader::new(file);
    let term_lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let mut terms = Vec::new();
    for term_str in &term_lines {
        terms.push(parse_term(&term_str));
    }
    // println!("Parsed {} terms.", terms.len());

    let mut rewritten_terms: Vec<(Vec<Symbol>, Vec<String>)> = Vec::new();
    let mut map: HashMap<String, (usize, usize)> = HashMap::new();
    let start = Instant::now();
    for (i, term) in terms.iter().enumerate() {
        // println!("Rewriting term {}/{}", i + 1, terms.len());
        // println!("Original term: {}", debug_print(term));
        let (rewritten_term, applied_rules) = rewrite(&rules, &canonicalizers, term, &mut map);
        rewritten_terms.push((
            rewritten_term.clone(),
            applied_rules.iter().map(|s| s.to_string()).collect(),
        ));
        // println!("Rewritten term: {}", debug_print(&rewritten_term));
        // println!(
        //     "Diff: {}",
        //     term.len() as isize - rewritten_term.len() as isize
        // );
        // println!("Rewrote {} terms...", i);
        // println!("Original: {}", debug_print(term));
    }
    let duration = start.elapsed().as_secs_f64();
    // println!("Rewriting completed in: {:?}", duration);
    // println!("Rewrote {} terms.", rewritten_terms.len());
    // println!(
    //     "Average time per term: {:?}",
    //     duration / rewritten_terms.len() as f64
    // );
    // println!("Test: {}", (-0.0));
    // println!("Rewriting completed.");

    let mut out = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&out_path)?;

    let mut total_diff = 0;

    for (i, (rewritten_term, applied_rules)) in rewritten_terms.iter().enumerate() {
        let original_term_str = term_lines[i].clone();
        let rewritten_term_str = pretty(&rewritten_term);
        total_diff += sym_count(original_term_str.clone()) as isize
            - sym_count(rewritten_term_str.clone()) as isize;
        let entry = OutputEntry {
            original_term: original_term_str,
            rewritten_term: rewritten_term_str,
            applied_rules: applied_rules.clone(),
        };
        let json = serde_json::to_string(&entry).map_err(|e| {
            eprintln!("Failed to serialize output entry to JSON: {}", e);
            e
        })?;
        writeln!(out, "{}", json).map_err(|e| {
            eprintln!(
                "Failed to write to output file '{}': {}",
                out_path.to_string_lossy(),
                e
            );
            e
        })?;
    }
    println!(
        "{},{:.10}",
        total_diff as f64 / rewritten_terms.len() as f64,
        duration / rewritten_terms.len() as f64
    );
    // println!(
    //     "Total symbol count difference (original - rewritten): {}",
    //     total_diff
    // );
    // println!(
    //     "Average symbol count difference per term: {}",
    //     total_diff as f64 / rewritten_terms.len() as f64
    // );
    // println!("Rewritten terms written to {}", out_file);

    Ok(())
}
