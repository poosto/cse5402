// TODO: add a file header comment explaining this module's purpose (lost 1 pt on Lab 1 for missing it).

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

pub const MIN_COMMAND_LINE_ARGS: usize = 2;
pub const MAX_COMMAND_LINE_ARGS: usize = 3;

pub const ARGS_IDX_PROGRAM_NAME: usize = 0;
pub const ARGS_IDX_SCRIPT_FILE: usize = 1;
pub const ARGS_IDX_WHINGE_MODE: usize = 2;

pub const ERR_BADLY_FORMED_COMMAND_LINE: u8 = 1;
pub const ERR_FAILED_TO_GENERATE_SCRIPT: u8 = 2;

// read_line reports zero bytes read only at end of file
pub const BYTES_READ_AT_EOF: usize = 0;

pub static WHINGE_MODE: AtomicBool = AtomicBool::new(false);

pub fn grab_trimmed_file_lines(file_name: &str, file_lines: &mut Vec<String>) -> Result<(), u8> {
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
