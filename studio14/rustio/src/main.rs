use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
    process::ExitCode,
};

fn main() -> ExitCode {
    let Some(file_name) = args().nth(1) else {
        eprintln!("usage: rustio <file_name>");
        return ExitCode::FAILURE;
    };
    println!("File name: {file_name}");

    let file = match File::open(&file_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Unable to open file \"{file_name}\" ({e})");
            return ExitCode::FAILURE;
        }
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) if !line.trim().is_empty() => println!("{line}"),
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to read line: {e}");
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}
