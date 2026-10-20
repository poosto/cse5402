// TODO: add a file header comment explaining this module's purpose (grader feedback from Lab 1).
// TODO: add a summary comment above every function (grader feedback from Lab 1).

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::atomic::Ordering;

use super::declarations::{Play, ERR_FAILED_TO_GENERATE_SCRIPT, WHINGE_MODE};

// read_line reports zero bytes read only at end of file
const BYTES_READ_AT_EOF: usize = 0;

fn grab_trimmed_file_lines(file_name: &str, file_lines: &mut Vec<String>) -> Result<(), u8> {
    match File::open(file_name) {
        Err(e) => {
            if WHINGE_MODE.load(Ordering::SeqCst) {
                eprintln!("WARNING: Failed to open file {} ({})", file_name, e);
            }

            Err(ERR_FAILED_TO_GENERATE_SCRIPT)
        }
        Ok(file) => {
            let mut buf = BufReader::new(file);
            let mut file_line = String::new();

            loop {
                file_line.clear();

                match buf.read_line(&mut file_line) {
                    Err(e) => {
                        if WHINGE_MODE.load(Ordering::SeqCst) {
                            eprintln!(
                                "WARNING: Failed to read a line from file {} ({})",
                                file_name, e
                            );
                        }

                        return Err(ERR_FAILED_TO_GENERATE_SCRIPT);
                    }
                    Ok(BYTES_READ_AT_EOF) => return Ok(()),
                    Ok(_) => file_lines.push(file_line.trim().to_string()),
                }
            }
        }
    }
}

pub fn script_gen(
    config_file: &str,
    play_title: &mut String,
    play_lines: &mut Play,
) -> Result<(), u8> {
    let mut config = PlayConfig::new();
    read_config(config_file, play_title, &mut config)?;

    process_config(play_lines, &config)
}
