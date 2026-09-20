pub mod lab1;

use std::{env, sync::atomic::Ordering};

use lab1::{declarations::{Play, ARGS_IDX_CONFIG_FILE, ARGS_IDX_PROGRAM_NAME, ARGS_IDX_WHINGE_MODE, ERR_BADLY_FORMED_COMMAND_LINE, MAX_COMMAND_LINE_ARGS, MIN_COMMAND_LINE_ARGS, WHINGE_MODE}, script_gen::script_gen};

// The text a user supplies as the optional third argument to turn on complaints
// about problems the program handles rather than failing on.
const WHINGE_ARG: &str = "whinge";

// env::args() yields nothing at all when the program is started with an empty
// argument vector, so the usage message falls back to this rather than indexing
// a position that may not exist.
const FALLBACK_PROGRAM_NAME: &str = "lab1";

fn usage(program_name: &str) {
    println!("usage: {} <configuration_file_name> [whinge]", program_name);
}

fn parse_args(config_file_name: &mut String) -> Result<(), u8> {
    let mut args = Vec::<String>::new();

    for arg in env::args() {
        args.push(arg); 
    }

    let program_name = args.get(ARGS_IDX_PROGRAM_NAME)
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

    config_file_name.clone_from(&args[ARGS_IDX_CONFIG_FILE]);

    // if "whinge" is provided (arg is already validated above)
    if args.len() == MAX_COMMAND_LINE_ARGS {
        debug_assert!(args[ARGS_IDX_WHINGE_MODE] == WHINGE_ARG);
        WHINGE_MODE.store(true, Ordering::SeqCst);
    }

    Ok(())
}

fn recite(play_title: &str, play: &Play) {
    println!("{}", play_title);

    let mut prev_character_name = String::new();
    for (_, character_name, line_text) in play {
        if prev_character_name != *character_name {
            println!("\n{}.", character_name);
        } 

        println!("{}", line_text);

        prev_character_name = character_name.clone();
    }
}

fn main() -> Result<(), u8> {
    let mut config_file_name = String::new();
    parse_args(&mut config_file_name)?;

    let mut play_title = String::new();
    let mut play_lines = Play::new();

    script_gen(&config_file_name, &mut play_title, &mut play_lines)?;

    play_lines.sort();

    recite(&play_title, &play_lines);

    Ok(())
}

