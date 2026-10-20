// Preston Meek -- meek@wustl.edu
// Lab 2 -- main.rs
// The entry point for the Lab 2 program. Read the command line, prepare the play
// from the script file, and recite it. Return a different exit code for each kind of failure.

pub mod lab2;

use lab2::{declarations::*, play::Play, return_wrapper::ReturnWrapper};
use std::{env, sync::atomic::Ordering};

// The text a user supplies as the optional third argument to turn on complaints
// about problems the program handles rather than failing on.
const WHINGE_ARG: &str = "whinge";

// env::args() yields nothing at all when the program is started with an empty
// argument vector, so the usage message falls back to this rather than indexing
// a position that may not exist.
const FALLBACK_PROGRAM_NAME: &str = "lab2";

// Print how to run the program.
fn usage(program_name: &str) {
    println!("usage: {} <script_file_name> [whinge]", program_name);
}

// Check the command line arguments. Store the script file name, and turn on whinge mode
// if "whinge" is given. If the arguments are wrong, print the usage message and return an error.
fn parse_args(script_file_name: &mut String) -> Result<(), u8> {
    let mut args = Vec::<String>::new();

    for arg in env::args() {
        args.push(arg);
    }

    let program_name = args
        .get(ARGS_IDX_PROGRAM_NAME)
        .map(String::as_str)
        .unwrap_or(FALLBACK_PROGRAM_NAME);

    // if too few or too many args
    if args.len() < MIN_COMMAND_LINE_ARGS || args.len() > MAX_COMMAND_LINE_ARGS {
        usage(program_name);
        return Err(ERR_BADLY_FORMED_COMMAND_LINE);
    }

    // if the optional arg is provided and it is NOT "whinge"
    if args.len() == MAX_COMMAND_LINE_ARGS && args[ARGS_IDX_WHINGE_MODE] != WHINGE_ARG {
        usage(program_name);
        return Err(ERR_BADLY_FORMED_COMMAND_LINE);
    }

    script_file_name.clone_from(&args[ARGS_IDX_SCRIPT_FILE]);

    // if "whinge" is provided (arg is already validated above)
    if args.len() == MAX_COMMAND_LINE_ARGS {
        debug_assert!(args[ARGS_IDX_WHINGE_MODE] == WHINGE_ARG);
        WHINGE_MODE.store(true, Ordering::SeqCst);
    }

    Ok(())
}

// TODO: document the run() design choice in the ReadMe's Return Wrapper subsection: main's logic lives in run() -> Result<(), u8> so it can use ? (which can't target ReturnWrapper on stable Rust, since the Try/FromResidual traits are unstable), and main wraps run()'s single Result with ReturnWrapper::new instead of unwrapping and rewrapping each Ok/Err.
// Do the program's main work. Parse the arguments, prepare the play, and recite it.
// Return the first error code that comes up, otherwise an empty Ok.
fn run() -> Result<(), u8> {
    let mut script_file_name = String::new();

    parse_args(&mut script_file_name)?;

    let mut play = Play::new();

    play.prepare(&script_file_name)?;
    play.recite();

    Ok(())
}

// Run the program and turn its result into the exit code through ReturnWrapper.
fn main() -> ReturnWrapper {
    ReturnWrapper::new(run())
}
