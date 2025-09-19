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
) -> Option<(HashMap<&'a str, Vec<Symbol>>, usize)> {
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
            if let Some(map) = unify(&rest_lhs, &term_lhs) {
                let rest_rhs = &lhs[lhs_idx + 1..];
                let term_rhs = &term[term_idx + 1..];
                if let Some(rest_map) = unify(&rest_rhs, &term_rhs) {
                    if let Some(merged) = merge_subst(map.0, rest_map.0) {
                        return Some((merged, 0));
                    } else {
                        return None;
                    }
                }
            }
        }
        if let Some(map) = unify(lhs, &term[1..(term[1].length + 1)]) {
            return Some((map.0, 1 + map.1));
        } else {
            if let Some(map) = unify(lhs, &term[(term[1].length + 1)..]) {
                return Some((map.0, term[1].length + 1 + map.1));
            } else {
                return None;
            }
        }
    }
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
        // println!("On term: {:?}", term);
        if let Some(subst) = unify(&rule.lhs, term) {
            if !check_conditions(&rule.cond, &subst.0) {
                continue;
            }
            let new_term = apply_subst(&rule.rhs, &subst.0);
            let rewritten = insert(new_term, term, subst.1);
            // println!("Rewritten term: {:?}", &rewritten);
            return Some((rewritten, rule.name.as_str()));
        }
    }
    None
}

fn rewrite<'a>(rules: &'a [Rule], term: &[Symbol]) -> (Vec<Symbol>, Vec<&'a str>) {
    let mut applied_rules = Vec::new();
    let mut current_term = term.to_vec(); // clone once, we’ll mutate this

    while let Some((new_term, rule_name)) = rewrite_one_step(rules, &mut current_term) {
        applied_rules.push(rule_name);
        current_term = new_term;
    }

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
            if part.chars().next().unwrap() == '?' {
                symbols.push(Symbol {
                    name: part[1..].to_ascii_uppercase().to_string(),
                    is_var: true,
                    length: 1,
                });
            } else {
                symbols.push(Symbol {
                    name: part.to_string(),
                    is_var: false,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("Usage: {} <rules-file> <term-file>", args[0]);
        std::process::exit(1);
    }
    let rules_file = &args[1];
    let term_file = &args[2];
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
        let (rewritten_term, applied_rules) = rewrite(&rules, term);
        rewritten_terms.push((
            rewritten_term.clone(),
            applied_rules.iter().map(|s| s.to_string()).collect(),
        ));
        // println!("Rewrote {} terms...", i);
        // println!("Original: {}", pretty(term));
    }
    let duration = start.elapsed();
    println!("Rewriting completed in: {:?}", duration);
    println!("Rewrote {} terms.", rewritten_terms.len());
    println!(
        "Average time per term: {:?}",
        duration / rewritten_terms.len() as u32
    );
    // println!("Rewriting completed.");

    let out_file = "rewritten_terms.jsonl";
    let mut out = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(out_file)?;

    for (i, (rewritten_term, applied_rules)) in rewritten_terms.iter().enumerate() {
        let original_term_str = term_lines[i].clone();
        let rewritten_term_str = pretty(&rewritten_term);
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
            eprintln!("Failed to write to output file '{}': {}", out_file, e);
            e
        })?;
    }
    // println!("Rewritten terms written to {}", out_file);

    Ok(())
}
