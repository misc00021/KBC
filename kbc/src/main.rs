mod translate;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;
use translate::{egg_to_twee, twee_to_egg};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input_file> --num_rules=<num_rules>", args[0]);
        std::process::exit(1);
    }
    let input_file = &args[1];
    let rule_target: Option<i32> = if args[2].contains('=') {
        let parts: Vec<&str> = args[2].split('=').collect();
        if parts.len() == 2 && parts[1].chars().all(|c| c.is_ascii_digit()) {
            parts[1].parse::<i32>().ok()
        } else {
            None
        }
    } else {
        if args[2].chars().all(|c| c.is_ascii_digit()) {
            args[2].parse::<i32>().ok()
        } else {
            None
        }
    };

    if rule_target.is_none() {
        eprintln!("Usage: {} <input_file> <num_rules>", args[0]);
        eprintln!("Error: <num_rules> must be a valid integer (e.g. 100 or --max-rules=100).");
        std::process::exit(1);
    }
    // let rule_target = Some(100);
    // let input_file = "/home/michi/Documents/Thesis/KBC/kbc/base_rules/as_egg/test.txt";
    let file = File::open(input_file).map_err(|e| {
        eprintln!("Failed to open input file '{}': {}", input_file, e);
        e
    })?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let twee_lines = egg_to_twee(lines);

    let twee_file_name = Path::new(input_file)
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| format!("{}{}.p", s, twee_lines.len()))
        .unwrap_or_else(|| "temp.p".to_string());

    let twee_file_path: PathBuf = ["base_rules/as_tptp", &twee_file_name].iter().collect();
    let mut twee = File::create(&twee_file_path).map_err(|e| {
        eprintln!(
            "Failed to create twee file '{}': {}",
            twee_file_path.display(),
            e
        );
        e
    })?;
    for line in twee_lines {
        writeln!(twee, "{}", line)?;
    }

    let out = Command::new("twee")
        .arg("--max-rules")
        .arg(rule_target.unwrap().to_string())
        .arg(twee_file_path)
        .output()?;

    // Convert stdout (Vec<u8>) to a String
    let stdout = String::from_utf8(out.stdout)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let mut lines: Vec<String> = stdout.lines().map(|s| s.to_string()).collect();

    let std_out_file_name = Path::new(input_file)
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| format!("{}_KBC_twee{}.txt", s, lines.len()))
        .unwrap_or_else(|| "temp_KBC_twee.txt".to_string());

    let std_out_file_path: PathBuf = ["extended_rules/twee_out", &std_out_file_name]
        .iter()
        .collect();

    let mut std_out_file = File::create(&std_out_file_path).map_err(|e| {
        eprintln!(
            "Failed to create output file '{}': {}",
            std_out_file_path.display(),
            e
        );
        e
    })?;

    std_out_file.write_all(stdout.as_bytes()).map_err(|e| {
        eprintln!(
            "Failed to write to output file '{}': {}",
            std_out_file_path.display(),
            e
        );
        e
    })?;

    lines = twee_to_egg(&lines);


    let output_file_name = Path::new(input_file)
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| format!("{}_KBC{}.txt", s, lines.len()))
        .unwrap_or_else(|| "temp_KBC.txt".to_string());

    let output_path: PathBuf = ["extended_rules/as_egg", &output_file_name]
        .iter()
        .collect();

    let mut output = File::create(output_path)?;
    for line in lines {
        writeln!(output, "{}", line)?;
    }

    Ok(())
}
