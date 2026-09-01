pub mod lab1;

use std::{env, sync::atomic::Ordering};

use lab1::declarations::{ARGS_IDX_CONFIG_FILE, ARGS_IDX_PROGRAM_NAME, ARGS_IDX_WHINGE_MODE, ERR_BADLY_FORMED_COMMAND_LINE, ERR_FAILED_TO_GENERATE_SCRIPT, MAX_COMMAND_LINE_ARGS, MIN_COMMAND_LINE_ARGS, WHINGE_MODE};

// TODO: why not &str?
fn usage(program_name: &String) {
    println!("usage: ./{} <configuration_file_name> [whinge]", program_name);
}

fn parse_args(config_file_name: &mut String) -> Result<(), u8> {
    let mut args = Vec::<String>::new();

    for arg in env::args() {
        args.push(arg); 
    }

    if args.len() < MIN_COMMAND_LINE_ARGS || args.len() > MAX_COMMAND_LINE_ARGS {
        usage(&args[ARGS_IDX_PROGRAM_NAME]);
        return Err(ERR_BADLY_FORMED_COMMAND_LINE);
    }

    if args.len() == MAX_COMMAND_LINE_ARGS && args[ARGS_IDX_WHINGE_MODE] != "whinge" {
        usage(&args[ARGS_IDX_PROGRAM_NAME]);
        return Err(ERR_FAILED_TO_GENERATE_SCRIPT);
    }

    *config_file_name = args[ARGS_IDX_CONFIG_FILE].clone();

    // if "whinge" is provided (arg is already validated above)
    if args.len() == MAX_COMMAND_LINE_ARGS {
        debug_assert!(args[ARGS_IDX_WHINGE_MODE] == "whinge"); 
        WHINGE_MODE.store(true, Ordering::SeqCst);
    }

    Ok(())
}

fn main() -> Result<(), u8> {
    parse_args(&mut String::from("test"))?;

    Ok(())
}

