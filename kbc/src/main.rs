mod translate;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead, BufReader};
use std::process::Command;
use translate::{egg_to_twee, twee_to_egg};

fn main() -> io::Result<()> {
    let target_num: u32 = 100;
    let args: Vec<String> = std::env::args().collect();
    let mut input_file = "/home/michi/Documents/thesis/KBC/kbc/mathRules.txt";
    if args.len() < 2 {
        // eprintln!("Usage: {} <input_file>", args[0]);
        // std::process::exit(1);
    } else {
        input_file = &args[1];
    }
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let twee_lines = egg_to_twee(lines);

    let twee_file = "/home/michi/Documents/thesis/KBC/kbc/tempOut.p";
    let mut twee = File::create(twee_file)?;
    for line in twee_lines {
        writeln!(twee, "{}", line)?;
    }

    let out = Command::new("twee")
        .arg("--max-rules")
        .arg(target_num.to_string())
        .arg(twee_file)
        .output()?;

    // Convert stdout (Vec<u8>) to a String
    let stdout = String::from_utf8(out.stdout)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    // Split into lines
    let mut lines: Vec<String> = stdout.lines().map(|s| s.to_string()).collect();

    lines = twee_to_egg(&lines);

    let output_file = input_file.replace(".txt", "KBC.txt");
    let mut output = File::create(output_file.clone())?;
    for line in lines {
        writeln!(output, "{}", line)?;
    }
    println!("Output written to {}", output_file);

    Ok(())
}
