use serde::Serialize;
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    hash::Hash,
    io::{BufRead, BufReader, Write},
    time::Instant,
};

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

fn merge_subst<'a>(
    mut s1: HashMap<&'a str, Vec<Symbol>>,
    s2: HashMap<&'a str, Vec<Symbol>>,
) -> Option<HashMap<&'a str, Vec<Symbol>>> {
    for (var, val2) in s2 {
        if let Some(val1) = s1.get(&var) {
            // Already bound: must check consistency
            if val1 != &val2 {
                return None; // conflict -> unification fails
            }
        } else {
            // New binding, insert
            s1.insert(var, val2);
        }
    }
    Some(s1)
}

fn unify<'a>(
    lhs: &'a [Symbol],
    term: &'a [Symbol],
    parent_match: bool,
) -> Option<(HashMap<&'a str, Vec<Symbol>>, usize)> {
    // println!(
    //     "Trying to match {} on {}",
    //     debug_print(&lhs.to_vec()),
    //     debug_print(&term.to_vec())
    // );
    if lhs[0].length == 1 {
        if lhs[0].is_var {
            let mut map = HashMap::with_capacity(lhs.len());
            map.insert(lhs[0].name.as_str(), term.to_vec());
            return Some((map, 0));
        } else if lhs[0].name == term[0].name {
            return Some((Default::default(), 0));
        } else {
            return None;
        }
    } else if lhs[0].length > term.len() {
        return None;
    } else {
        if lhs[0].name == term[0].name {
            let lhs_idx = lhs[1].length;
            let term_idx = term[1].length;
            let rest_lhs = &lhs[1..lhs_idx + 1];
            let term_lhs = &term[1..term_idx + 1];
            if let Some(map) = unify(&rest_lhs, &term_lhs, true) {
                // println!("Map1: {:?}", map);
                let rest_rhs = &lhs[lhs_idx + 1..];
                let term_rhs = &term[term_idx + 1..];
                if let Some(rest_map) = unify(&rest_rhs, &term_rhs, true) {
                    // println!("Map2: {:?}", rest_map);
                    if let Some(merged) = merge_subst(map.0, rest_map.0) {
                        // println!("Merged: {:?}", merged);
                        return Some((merged, 0));
                    }
                }
            }
        }
        if !parent_match {
            if let Some(map) = unify(lhs, &term[1..(term[1].length + 1)], parent_match) {
                // println!("Map 3: {:?}", map);
                return Some((map.0, 1 + map.1));
            } else {
                if let Some(map) = unify(lhs, &term[(term[1].length + 1)..], parent_match) {
                    // println!("Map 4: {:?}", map);
                    return Some((map.0, term[1].length + 1 + map.1));
                } else {
                    return None;
                }
            }
        }
    }
    return None;
}

fn check_conditions(cond: &Vec<Symbol>, subst: &HashMap<&str, Vec<Symbol>>) -> bool {
    for var in cond {
        // println!("Checking condition on variable: {}", var.name);
        if let Some(sym) = subst.get(var.name.as_str()) {
            if sym.len() == 1 && sym[0].name == "0" {
                return false;
            }
        }
    }
    true
}

fn insert(new_term: Vec<Symbol>, old_term: &mut Vec<Symbol>, idx: usize) -> Vec<Symbol> {
    // println!("Old term: {:?}", pretty(old_term));
    // println!("New term: {:?}", pretty(&new_term));
    let mut i = 0;
    let diff = old_term[idx].length - new_term.len();
    while i < idx {
        if old_term[i].length != 1 {
            old_term[i].length -= diff;
        }
        i += 1;
        if old_term[i].length < idx - i + 1 {
            i += old_term[i].length;
        }
    }
    let mut result = old_term[..idx].to_vec();
    result.extend(new_term);
    // println!("Inserted at index {}: {:?}", idx, pretty(&result));
    result.extend(old_term[idx + old_term[idx].length..].to_vec());
    result
}

fn apply_subst(term: &[Symbol], subst: &HashMap<&str, Vec<Symbol>>) -> Vec<Symbol> {
    fn apply_rec(term: &[Symbol], subst: &HashMap<&str, Vec<Symbol>>) -> (Vec<Symbol>, usize) {
        let first = &term[0];
        if first.length == 1 {
            if first.is_var {
                if let Some(repl) = subst.get(first.name.as_str()) {
                    let result = repl.clone();
                    return (result, repl.len());
                } else {
                    return (vec![first.clone()], 1);
                }
            } else {
                return (vec![first.clone()], 1);
            }
        } else {
            let mut result = vec![first.clone()];
            let left = &term[1..term[1].length + 1];
            let (mut sub_result, len_first) = apply_rec(&left, subst);
            result.append(&mut sub_result);
            let rest = &term[term[1].length + 1..];
            let (mut rest_result, len_rest) = apply_rec(&rest, subst);
            result.append(&mut rest_result);
            result[0].length = len_first + len_rest + 1;
            let length = result[0].length;
            return (result, length);
        }
    }
    apply_rec(term, subst).0
}

fn rewrite_one_step<'a>(
    rules: &'a [Rule],
    term: &mut Vec<Symbol>,
) -> Option<(Vec<Symbol>, &'a str)> {
    for rule in rules {
        // println!("Trying rule: {}", rule.name);
        // println!("On term: {:?}", debug_print(term));
        if let Some(subst) = unify(&rule.lhs, term, false) {
            if !check_conditions(&rule.cond, &subst.0) {
                continue;
            }
            let new_term = apply_subst(&rule.rhs, &subst.0);
            let rewritten = insert(new_term, term, subst.1);
            // println!("Rewritten term: {:?}", debug_print(&rewritten));
            return Some((rewritten, rule.name.as_str()));
        }
    }
    None
}

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
fn smaller(a: &Vec<Symbol>, b: &Vec<Symbol>) -> bool {
    let weight = |sym: &Symbol| if sym.is_num { 2 } else { 1 };
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
    //     pretty(a),
    //     a_weight,
    //     pretty(b),
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
            if w_a < w_b {
                // println!("Smaller: {:?} < {:?}", pretty(a), pretty(b));
                return true;
            } else if w_a > w_b {
                // println!("Not smaller: {:?} > {:?}", pretty(a), pretty(b));
                return false;
            }
        }
        return false;
    }
}

fn canonicalize(old: &mut Vec<Symbol>, canonicalizers: &[Rule]) -> bool {
    // println!("Canonicalizing: {}", debug_print(old));
    let mut canonicalized = false;
    for rule in canonicalizers {
        let mut i = old.len();
        while i > 0 {
            i -= 1;
            if let Some(subst) = unify(&rule.lhs, &old[i..i + old[i].length], false) {
                if !check_conditions(&rule.cond, &subst.0) {
                    continue;
                }
                let new_term = apply_subst(&rule.rhs, &subst.0);
                let rewritten = insert(new_term, old, i + subst.1);

                if smaller(&rewritten, old) {
                    *old = rewritten;
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

fn rewrite<'a>(
    rules: &'a [Rule],
    canonicalizers: &[Rule],
    term: &[Symbol],
) -> (Vec<Symbol>, Vec<&'a str>) {
    let mut applied_rules = Vec::new();
    let mut current_term = term.to_vec();
    let mut canonicalizable = true;
    while canonicalizable {
        while let Some((new_term, rule_name)) = rewrite_one_step(rules, &mut current_term) {
            applied_rules.push(rule_name);
            current_term = new_term;
            fold_constants(&mut current_term);
        }
        // println!("After rewriting: {}", debug_print(&current_term));
        canonicalizable = canonicalize(&mut current_term, canonicalizers);
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

fn check_canonicalizer(rule: &Rule) -> bool {
    if rule.lhs.len() <= rule.rhs.len() {
        return true;
    }
    if rule.lhs.len() != rule.rhs.len() {
        return false;
    }
    let mut lhs_vars = HashMap::new();
    let mut rhs_vars = HashMap::new();
    for i in 0..rule.lhs.len() {
        let lhs_sym = &rule.lhs[i];
        let rhs_sym = &rule.rhs[i];
        if lhs_sym.length == 1 {
            if !lhs_vars.contains_key(lhs_sym.name.as_str()) {
                lhs_vars.insert(lhs_sym.name.as_str(), 1);
            } else {
                lhs_vars.insert(lhs_sym.name.as_str(), lhs_vars[lhs_sym.name.as_str()] + 1);
            }
        }
        if rhs_sym.length == 1 {
            if !rhs_vars.contains_key(rhs_sym.name.as_str()) {
                rhs_vars.insert(rhs_sym.name.as_str(), 1);
            } else {
                rhs_vars.insert(rhs_sym.name.as_str(), rhs_vars[rhs_sym.name.as_str()] + 1);
            }
        }
    }
    lhs_vars == rhs_vars
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

        let lhs = parse_term(&lhs_str);
        let rhs = parse_term(&rhs_str);
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

    let (rules, canonicalizers) = parse_rules(&egg_rules);

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
    let start = Instant::now();
    for (i, term) in terms.iter().enumerate() {
        let (rewritten_term, applied_rules) = rewrite(&rules, &canonicalizers, term);
        rewritten_terms.push((
            rewritten_term.clone(),
            applied_rules.iter().map(|s| s.to_string()).collect(),
        ));
        // println!("Rewrote {} terms...", i);
        // println!("Original: {}", pretty(term));
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
