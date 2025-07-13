use std::collections::VecDeque;
use std::vec::Vec;

struct Symbol {
    name: String,
    length: usize,
}

struct Rule {
    name: String,
    lhs: Vec<Symbol>,
    rhs: Vec<Symbol>,
    cond: Vec<Symbol>,
}

fn term_dump(term: &Vec<Symbol>) -> String {
    let mut ret = String::new();
    for s in term {
        ret.push_str(&format!("[{},{}] ", s.name, s.length));
    }
    ret
}

fn rule_dump(rule: &Rule) -> String {
    let lhs = term_dump(&rule.lhs);
    let rhs = term_dump(&rule.rhs);
    let cond = term_dump(&rule.cond);
    format!(
        "Rule: {} LHS: {} RHS: {} Cond: {}",
        rule.name, lhs, rhs, cond
    )
}

// Parsing as flatterms (idea from twee paper)
fn parse_egg_term(chars: &mut VecDeque<char>) -> Vec<Symbol> {
    let mut flat = Vec::new();
    let mut c = chars.pop_front();
    while c.is_some() && c.unwrap().is_whitespace() {
        c = chars.pop_front();
    }
    if c.is_none() {
        return flat; // Empty term
    }
    match c.unwrap() {
        '?' => {
            let mut name = String::new();
            while let Some(ch) = chars.pop_front() {
                if ch.is_whitespace() {
                    break;
                }
                if ch == ')' {
                    chars.push_front(ch);
                    break;
                }
                name.push(ch.to_ascii_uppercase());
            }
            flat.push(Symbol { name, length: 1 });
        }
        '(' => {
            let mut name = String::new();
            while let Some(ch) = chars.pop_front() {
                if ch.is_whitespace() {
                    break;
                }
                name.push(ch);
            }
            flat.push(Symbol { name, length: 1 });
            let mut child = parse_egg_term(chars);
            while !child.is_empty() {
                flat[0].length += child.len();
                flat.append(&mut child);
                child = parse_egg_term(chars);
            }
        }
        ')' => {
            return flat;
        }
        _ => {
            let mut name = String::new();
            name.push(c.unwrap());
            while let Some(ch) = chars.pop_front() {
                if ch.is_whitespace() {
                    break;
                }
                if ch == ')' {
                    chars.push_front(ch);
                    break;
                }
                name.push(ch.to_ascii_lowercase());
            }
            flat.push(Symbol { name, length: 1 });
        }
    }
    return flat;
}

fn get_cond(cond: Vec<&str>) -> Vec<Symbol> {
    if cond.is_empty() {
        return Vec::new();
    }
    let mut ret = Vec::new();
    for c in cond.iter() {
        let mut condition = Vec::new();
        let mut chars = c
            .trim()
            .replace('"', "")
            .replace(',', "")
            .chars()
            .collect::<VecDeque<char>>();
        let mut name = String::new();
        while let Some(ch) = chars.pop_front()
            && !(ch == '(')
        {
            if ch.is_whitespace() {
                continue;
            }
            name.push(ch);
        }
        condition.push(Symbol { name, length: 1 });
        let mut child = parse_egg_term(&mut chars);
        while !child.is_empty() {
            condition[0].length += child.len();
            condition.append(&mut child);
            child = parse_egg_term(&mut chars);
        }
        ret.append(&mut condition);
    }
    return ret;
}

fn egg_to_flat(lines: Vec<String>) -> Vec<Rule> {
    // Convert Egg format to Twee format
    let mut i = 0;
    let mut rules = Vec::new();
    while i < lines.len() {
        let mut working = lines[i].trim().to_string();
        if working.len() < 6 {
            i += 1;
            continue; // Skip empty or malformed lines
        }
        while !working.ends_with(",") {
            if i >= lines.len() {
                break; // Prevent out-of-bounds access
            }
            i += 1;
            working.push(' ');
            working.push_str(lines[i].trim());
        }
        if working.starts_with("//") {
            i += 1;
            continue; // Skip comments
        }
        working = working.trim().trim_end_matches(',').to_string();
        let working = &working[4..working.len() - 1];
        println!("Processing line: {}", working);
        let working = working.split(';').collect::<Vec<&str>>();
        let name = working[0];
        let name = name.trim().trim_matches('"');
        let body = working[1].split("=>").collect::<Vec<&str>>();
        let lhs = body[0];
        let rhs = body[1];
        let mut condition = Vec::new();
        let rhs_twee;
        if rhs.contains(" if ") {
            let rhs_vec = rhs.split(" if ").collect::<Vec<&str>>();
            condition = get_cond(rhs_vec[1..].to_vec());
            rhs_twee = parse_egg_term(
                &mut rhs_vec[0]
                    .trim()
                    .trim_matches('"')
                    .chars()
                    .collect::<VecDeque<char>>(),
            );
        } else {
            rhs_twee = parse_egg_term(
                &mut rhs
                    .trim()
                    .trim_matches('"')
                    .chars()
                    .collect::<VecDeque<char>>(),
            );
        }
        let lhs_twee = parse_egg_term(
            &mut lhs
                .trim()
                .trim_matches('"')
                .chars()
                .collect::<VecDeque<char>>(),
        );
        let rule = Rule {
            name: name.to_string().replace('-', "_"),
            lhs: lhs_twee,
            rhs: rhs_twee,
            cond: condition,
        };
        println!("{}", rule_dump(&rule));
        i += 1;
        rules.push(rule);
    }
    return rules;
}

fn twee_print(term: &mut Vec<Symbol>, is_cond: bool) -> String {
    let mut ret = String::new();
    let mut paren_at = Vec::new();
    let mut i = 0;
    if is_cond {
        ret.push_str("(");
    }
    while i < term.len() {
        let s = term.get(i).unwrap();
        if s.length == 1 {
            if s.name.char_indices().next().unwrap().1.is_uppercase() {
                ret.push_str(s.name.as_str());
            } else {
                ret.push_str(&format!("'{}'", s.name));
            }
            i += 1;
            if !paren_at.is_empty() && paren_at[paren_at.len() - 1] != i {
                ret.push_str(", ");
            }
        } else {
            paren_at.push(i + s.length);
            if s.name.char_indices().next().unwrap().1.is_alphabetic() {
                ret.push_str(&format!("{}(", s.name));
            } else {
                ret.push_str(&format!("'{}'(", s.name));
            }
            i += 1;
        }
        if !paren_at.is_empty() && paren_at[paren_at.len() - 1] == i {
            while !paren_at.is_empty() && paren_at[paren_at.len() - 1] == i {
                ret.push(')');
                paren_at.pop();
            }
            if i < term.len() {
                if !is_cond {
                    ret.push_str(", ");
                } else {
                    ret.push_str(" & ");
                }
            }
        }
    }
    while !paren_at.is_empty() {
        ret.push(')');
        paren_at.pop();
    }
    if is_cond {
        ret.push(')');
    }
    ret
}

fn flat_to_twee(rule: &mut Rule) -> String {
    let mut ret = String::new();
    ret.push_str(&format!("cnf({}, axiom, ", &rule.name));
    if !rule.cond.is_empty() {
        ret.push_str(format!("{} => ", twee_print(&mut rule.cond, true)).as_str());
    }
    ret.push_str(&format!(
        "{} = {}).",
        twee_print(&mut rule.lhs, false),
        twee_print(&mut rule.rhs, false)
    ));
    ret
}

pub fn egg_to_twee(lines: Vec<String>) -> Vec<String> {
    let rules = egg_to_flat(lines);
    let mut twee_lines = Vec::new();
    for mut rule in rules {
        twee_lines.push(flat_to_twee(&mut rule));
    }
    twee_lines.push(format!("cnf(goal, conjecture, true = false)."));
    return twee_lines;
}

pub fn twee_to_egg(lines: &mut Vec<String>) -> &mut Vec<String> {
    // Convert Twee format to Egg format
    return lines;
}
