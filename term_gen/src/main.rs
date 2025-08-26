use rand;
use std::collections::HashSet;
use std::io::Write;

fn gen_term(
    var_count: &mut usize,
    ops: &Vec<(&str, usize)>,
    rng: &mut impl rand::Rng,
    depth: usize,
) -> (String, usize) {
    let vars = vec!["a", "b", "c", "d", "e"];
    let consts = vec!["0", "1", "-1"];
    let op;
    let arity;
    // Limit depth to avoid stack overflow
    if depth > 8 {
        arity = 0;
        op = if rng.random() { "var" } else { "const" };
    } else {
        (op, arity) = ops[rng.random_range(0..ops.len())];
    }
    if arity == 0 {
        if op == "var" {
            if (rng.random() && *var_count < vars.len()) || *var_count == 0 {
                *var_count += 1;
            }
            (vars[rng.random_range(0..*var_count)].to_string(), 0)
        } else {
            (consts[rng.random_range(0..consts.len())].to_string(), 0)
        }
    } else {
        let arg1 = gen_term(var_count, ops, rng, depth + 1);
        // Limit depth of second argument based on size of first argument to avoid explosion
        let arg2 = gen_term(
            var_count,
            ops,
            rng,
            depth + (f32::log2(arg1.1 as f32 + 1.0) as usize + 1),
        );
        (
            format!("({} {} {})", op, arg1.0, arg2.0),
            1 + arg1.1 + arg2.1,
        )
    }
}

fn term_gen() {
    let mut rng = rand::rng();
    let mut terms = Vec::new();
    let ops = vec![
        ("+", 2),
        ("-", 2),
        ("*", 2),
        ("/", 2),
        ("pow", 2),
        ("var", 0),
        ("const", 0),
    ];
    let mut var_count;
    for _i in 1..=100000 {
        var_count = 0;
        let term = gen_term(&mut var_count, &ops, &mut rng, 0);
        while terms.len() < term.1 + 1 {
            terms.push(HashSet::new());
        }
        terms[term.1].insert(term);
    }
    // let mut sum = 0;
    // for (i, ts) in terms.iter().enumerate() {
    //     println!("Length {}: {} terms", i, ts.len());
    //     sum += ts.len();
    // }
    // println!("Total: {} terms", sum);
    // let out_file = std::fs::File::create("terms.txt").unwrap();
    // let mut out = std::io::BufWriter::new(out_file);
    // for ts in terms.iter() {
    //     for t in ts.iter() {
    //         writeln!(out, "{}", t.0).unwrap();
    //     }
    // }
    let out_file = std::fs::File::create("terms_max_len4.txt").unwrap();
    let mut out = std::io::BufWriter::new(out_file);
    for ts in terms.iter().take(5) {
        for t in ts.iter() {
            writeln!(out, "{}", t.0).unwrap();
        }
    }
    let out_file = std::fs::File::create("terms_max_len5.txt").unwrap();
    let mut out = std::io::BufWriter::new(out_file);
    for ts in terms.iter().take(6) {
        for t in ts.iter() {
            writeln!(out, "{}", t.0).unwrap();
        }
    }
    let out_file = std::fs::File::create("terms_len6_to_10.txt").unwrap();
    let mut out = std::io::BufWriter::new(out_file);
    for ts in terms.iter().skip(6).take(5) {
        for t in ts.iter() {
            writeln!(out, "{}", t.0).unwrap();
        }
    }
    let out_file = std::fs::File::create("terms_len11_plus.txt").unwrap();
    let mut out = std::io::BufWriter::new(out_file);
    for ts in terms.iter().skip(11) {
        for t in ts.iter() {
            writeln!(out, "{}", t.0).unwrap();
        }
    }
}

fn main() {
    term_gen();
}
