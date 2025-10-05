use std::collections::{HashSet, VecDeque};
use std::slice;
use std::vec::Vec;

#[derive(PartialEq, Clone)]
struct Symbol {
    name: String,
    length: usize,
}

#[derive(Clone)]
pub struct Rule {
    ordered: bool,
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

pub fn egg_to_flat(lines: Vec<String>) -> Vec<Rule> {
    // Convert Egg format to flatterms
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
            ordered: true,
            name: name.to_string().replace('-', "_"),
            lhs: lhs_twee,
            rhs: rhs_twee,
            cond: condition,
        };
        // println!("{}", rule_dump(&rule));
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

fn parse_twee_term(term: &mut VecDeque<char>) -> Vec<Symbol> {
    let mut ret = Vec::new();
    let mut char;
    let mut lhs = Vec::new();
    let mut name = String::new();
    loop {
        let c = term.pop_front();
        if c.is_none() {
            return ret;
        } else if !c.unwrap().is_whitespace() {
            char = c.unwrap();
            break;
        }
    }
    if char == '(' {
        lhs.append(parse_twee_term(term).as_mut());
        while term.front().is_some() && *term.front().unwrap() != ')' {
            term.pop_front();
        }
        term.pop_front();
        let c = term.pop_front();
        if c.is_none() {
            return lhs;
        }
        char = c.unwrap();
    } else {
        loop {
            name.push(char);
            let c = term.pop_front();
            if c.is_none() {
                ret.push(Symbol { name, length: 1 });
                return ret;
            } else {
                char = c.unwrap();
                if char == '(' || char.is_whitespace() || char == ')' || char == ',' {
                    break;
                }
            }
        }
        if char == '(' {
            lhs.push(Symbol { name, length: 1 });
            loop {
                let mut arg = parse_twee_term(term);
                if arg.is_empty() {
                    break;
                }
                lhs[0].length += arg.len();
                lhs.append(arg.as_mut());
                if term.front().is_some_and(|c| *c == ')') {
                    term.pop_front();
                    break;
                } else if term.front().is_some_and(|c| *c == ',') {
                    term.pop_front();
                }
            }
            let c = term.pop_front();
            if c.is_some() {
                char = c.unwrap();
            }
        } else if char == ',' || char == ')' {
            term.push_front(char);
            ret.push(Symbol { name, length: 1 });
            return ret;
        } else {
            lhs.push(Symbol { name, length: 1 });
        }
    }
    if !char.is_whitespace() {
        term.push_front(char);
        return lhs;
    }
    while char.is_whitespace() {
        char = term.pop_front().unwrap();
    }
    let mut infix_op = String::new();
    while !char.is_whitespace() {
        infix_op.push(char);
        char = term.pop_front().unwrap();
    }
    let mut rhs = parse_twee_term(term);
    ret.push(Symbol {
        name: infix_op,
        length: 1 + lhs.len() + rhs.len(),
    });
    ret.append(lhs.as_mut());
    ret.append(rhs.as_mut());

    return ret;
}

// Split a term on ',' but only when not inside parentheses
fn true_split(term: &str) -> Vec<String> {
    let mut ret = Vec::<String>::new();
    let mut paren = 0;
    ret.push(String::new());
    for c in term.chars() {
        match c {
            '(' => {
                paren += 1;
                ret.last_mut().unwrap().push(c);
            }
            ')' => {
                paren -= 1;
                ret.last_mut().unwrap().push(c);
            }
            ',' if paren == 0 => {
                ret.push(String::new());
            }
            _ => {
                ret.last_mut().unwrap().push(c);
            }
        }
    }

    ret
}

fn twee_term_to_flat(mut term: String) -> (Option<Vec<Symbol>>, Vec<Symbol>) {
    let mut cond = Vec::new();
    let flat;
    let mut working: Vec<String>;
    while term.trim().starts_with("ifeq") {
        let parts = term.splitn(2, '(').collect::<Vec<&str>>();
        if parts.len() < 2 {
            break; // Malformed term
        }
        // True split on commas
        working = true_split(parts[1]);
        // Extract condition
        let mut condition = parse_twee_term(&mut working[0].chars().collect::<VecDeque<char>>());
        cond.append(condition.as_mut());
        // Remove true2
        term = working[2].clone();
    }
    let mut chars = term.trim().chars().collect::<VecDeque<char>>();
    flat = parse_twee_term(&mut chars);
    let mut cond_ret = None;
    if !cond.is_empty() {
        cond_ret = Some(cond);
    }
    return (cond_ret, flat);
}

fn twee_out_to_flat(lines: &Vec<String>) -> Vec<Rule> {
    // Convert Egg format to flatterms
    let mut i = 0;
    let mut j = 0;
    let mut rules = Vec::new();
    while i < lines.len() && !lines[i].contains("final rewrite system:") {
        i += 1;
    }
    i += 1; // Skip the "final rewrite system:" line
    while i < lines.len() && !lines[i].is_empty() {
        // println!("Processing line: {}", lines[i]);
        if lines[i].trim() == "ifeq(X, X, Y, Z) -> Y" {
            i += 1;
            continue;
        }
        let working;
        let mut ordered = true;
        if lines[i].contains(" = ") {
            ordered = false;
            working = lines[i].split('=').collect::<Vec<&str>>();
        } else if lines[i].contains(" -> ") {
            working = lines[i].split("->").collect::<Vec<&str>>();
        } else if lines[i].contains(" <-> ") {
            working = lines[i].split("<->").collect::<Vec<&str>>();
        } else {
            i += 1;
            continue; // Skip malformed lines
        }
        let lhs_ret;
        let cond_ret;
        match twee_term_to_flat(working[0].trim().to_string()) {
            (Some(cond), lhs) => {
                lhs_ret = lhs;
                cond_ret = cond;
            }
            (None, lhs) => {
                lhs_ret = lhs;
                cond_ret = Vec::new();
            }
        }
        let rhs = twee_term_to_flat(working[1].trim().to_string()).1;
        let name = format!("rule_{}", j);
        i += 1;
        j += 1;
        let rule = Rule {
            ordered,
            name,
            lhs: lhs_ret,
            rhs,
            cond: cond_ret,
        };
        // println!("{}", rule_dump(&rule));
        rules.push(rule);
    }
    return rules;
}

fn egg_print(term: &mut Vec<Symbol>) -> String {
    let mut ret = String::new();
    ret.push('"');
    let mut paren_at = Vec::new();
    let mut i = 0;
    while i < term.len() {
        let s = term.get(i).unwrap();
        if s.length == 1 {
            if s.name.chars().next().unwrap().is_uppercase() {
                ret.push_str(format!("?{}", s.name.to_ascii_lowercase()).as_str());
            } else {
                ret.push_str(format!("{}", s.name).as_str());
            }
            i += 1;
            if !paren_at.is_empty() {
                if paren_at[paren_at.len() - 1] != i {
                    ret.push(' ');
                } else {
                    while !paren_at.is_empty() && paren_at[paren_at.len() - 1] == i {
                        ret.push(')');
                        paren_at.pop();
                        if !paren_at.is_empty() {
                            ret.push(' ');
                        }
                    }
                }
            }
        } else {
            paren_at.push(i + s.length);
            ret.push_str(format!("({} ", s.name).as_str());
            i += 1;
        }
    }
    while !paren_at.is_empty() {
        ret.push(')');
        paren_at.pop();
    }
    ret.push('"');
    return ret;
}

fn egg_print_cond(cond: &mut Vec<Symbol>) -> String {
    let mut ret = String::new();
    let mut i = 0;
    while i < cond.len() {
        if i != 0 {
            ret.push_str(" ");
        }
        let s = cond.get(i).unwrap();
        ret.push_str(&format!("if {}(", s.name));
        for j in 1..s.length {
            i += 1;
            ret.push_str(egg_print(&mut vec![cond[i].clone()]).as_str());
            if j < s.length - 1 {
                ret.push_str(", ");
            }
        }
        ret.push(')');
        i += 1;
    }
    return ret;
}

fn order_rule(rule: &mut Rule) {
    // println!("Ordering rule: {}", rule_dump(&rule));
    let mut lhs_count = 0;
    let mut rhs_count = 0;
    for s in &rule.lhs {
        if s.length == 1 && s.name.chars().next().unwrap().is_uppercase() {
            lhs_count += 1;
        }
    }
    for s in &rule.rhs {
        if s.length == 1 && s.name.chars().next().unwrap().is_uppercase() {
            rhs_count += 1;
        }
    }
    // Leaves properly ordered rules as they are
    if rhs_count > lhs_count {
        let temp = rule.lhs.clone();
        rule.lhs = rule.rhs.clone();
        rule.rhs = temp;
    }
    rule.ordered = true;
}

pub fn flat_to_egg(rule: &mut Rule, delete_unorderable: bool) -> String {
    let mut ret = String::new();
    // order_rule(rule);
    if rule.ordered == false && delete_unorderable {
        return String::new(); // Skip unordered rules
    }
    ret.push_str(&format!(
        "rw!(\"{}\"; {} => {}",
        rule.name,
        egg_print(&mut rule.lhs),
        egg_print(&mut rule.rhs)
    ));

    if !rule.cond.is_empty() {
        // Skip rules which have wrong ordering due to conditions, i.e. X!= 0 => 0 => 0/X
        let lhs_vars = lockdown(rule.lhs.clone());
        let rhs_vars = lockdown(rule.rhs.clone());
        for var in rhs_vars {
            if !lhs_vars.contains(&var) {
                return String::new();
            }
        }
        let cond = egg_print_cond(&mut rule.cond);
        if !cond.contains('?') {
            return String::new(); // Skip rules without variables in conditions
        }
        ret.push_str(&format!(" {}", cond));
    }
    if rule.lhs == rule.rhs {
        return String::new(); // Skip rules that are identical on both sides
    }
    ret.push_str("),");
    if rule.ordered == false && !delete_unorderable {
        ret.push_str(&format!(
            "\nrw!(\"{}\"; {} => {}",
            format!("{}_rev", rule.name),
            egg_print(&mut rule.rhs),
            egg_print(&mut rule.lhs)
        ));
        if !rule.cond.is_empty() {
            // Skip rules which have wrong ordering due to conditions, i.e. X!= 0 => 0 => 0/X
            let lhs_vars = lockdown(rule.lhs.clone());
            let rhs_vars = lockdown(rule.rhs.clone());
            for var in rhs_vars {
                if !lhs_vars.contains(&var) {
                    return String::new();
                }
            }
            let cond = egg_print_cond(&mut rule.cond);
            if !cond.contains('?') {
                return String::new(); // Skip rules without variables in conditions
            }
            ret.push_str(&format!(" {}", cond));
        }
        ret.push_str("),");
    }
    return ret.replace("(neg 1)", "-1");
}

pub fn twee_to_egg(lines: &Vec<String>, delete_unorderable: bool) -> Vec<String> {
    let rules = twee_out_to_flat(&lines);
    let mut lines = Vec::new();
    for mut rule in rules {
        lines.push(flat_to_egg(&mut rule, delete_unorderable));
    }
    return lines;
}

fn lockdown(slice: Vec<Symbol>) -> HashSet<String> {
    let mut vars = HashSet::new();
    if slice.is_empty() {
        return vars;
    }
    for s in slice {
        if s.length == 1 && s.name.chars().next().unwrap().is_uppercase() {
            vars.insert(s.name);
        }
    }
    return vars;
}

// Traverse the term and add guards for variables that could lead to division by zero
fn add_guard(slice: Vec<Symbol>) -> HashSet<String> {
    let mut nonzero_vars = HashSet::new();
    if slice.is_empty() {
        return nonzero_vars;
    }
    if slice[0].name == "/" {
        let arg1len = slice[1].length;
        if arg1len != 1 {
            nonzero_vars.extend(add_guard(slice[1..(1 + arg1len)].to_vec()));
        }
        let arg2len = slice[1 + arg1len].length;
        if arg2len != 1 {
            nonzero_vars.extend(lockdown(slice[(1 + arg1len)..].to_vec()));
        } else if slice[1 + arg1len]
            .name
            .chars()
            .next()
            .unwrap()
            .is_uppercase()
        {
            nonzero_vars.insert(slice[1 + arg1len].name.clone());
        }
    } else if slice[0].name == "pow" {
        let arg1len = slice[1].length;
        if arg1len != 1 {
            nonzero_vars.extend(add_guard(slice[1..(1 + arg1len)].to_vec()));
        } else if slice[1].name.chars().next().unwrap().is_uppercase()
            && slice[2].name.chars().next().unwrap().is_uppercase()
        {
            nonzero_vars.insert(slice[1].name.clone());
        }
        let arg2len = slice[1 + arg1len].length;
        if arg2len != 1 {
            nonzero_vars.extend(add_guard(slice[(1 + arg1len)..].to_vec()));
        }
    } else if slice[0].length != 1 {
        let arg1len = slice[1].length;
        nonzero_vars.extend(add_guard(slice[1..(1 + arg1len)].to_vec()));
        nonzero_vars.extend(add_guard(slice[(1 + arg1len)..].to_vec()));
    }
    return nonzero_vars;
}

pub fn add_guards(lines: Vec<String>, delete_unorderable: bool) -> Vec<String> {
    let rules = twee_out_to_flat(&lines);
    let mut out = Vec::new();
    for rule in rules {
        let mut nonzero_vars = add_guard(rule.lhs.clone());
        nonzero_vars.extend(add_guard(rule.rhs.clone()));
        let mut new_rule = rule.clone();
        for var in nonzero_vars {
            new_rule.cond.push(Symbol {
                name: "is_not_zero".to_string(),
                length: 2,
            });
            new_rule.cond.push(Symbol {
                name: var,
                length: 1,
            });
        }
        out.push(flat_to_egg(&mut new_rule, delete_unorderable));
    }
    return out;
}
