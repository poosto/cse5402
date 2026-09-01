pub mod lab1;

use std::{env, sync::atomic::Ordering};

use lab1::declarations::{Play, ARGS_IDX_CONFIG_FILE, ARGS_IDX_PROGRAM_NAME, ARGS_IDX_WHINGE_MODE, ERR_BADLY_FORMED_COMMAND_LINE, MAX_COMMAND_LINE_ARGS, MIN_COMMAND_LINE_ARGS, WHINGE_MODE};

// TODO: why not &str?
fn usage(program_name: &String) {
    println!("usage: {} <configuration_file_name> [whinge]", program_name);
}

fn parse_args(config_file_name: &mut String) -> Result<(), u8> {
    let mut args = Vec::<String>::new();

    for arg in env::args() {
        args.push(arg); 
    }

    // if too few or too many args
    if args.len() < MIN_COMMAND_LINE_ARGS || args.len() > MAX_COMMAND_LINE_ARGS {
        usage(&args[ARGS_IDX_PROGRAM_NAME]);
        return Err(ERR_BADLY_FORMED_COMMAND_LINE);
    }

    // if the optional arg is provided and it is NOT "whinge"
    if args.len() == MAX_COMMAND_LINE_ARGS && args[ARGS_IDX_WHINGE_MODE] != "whinge" {
        usage(&args[ARGS_IDX_PROGRAM_NAME]);
        return Err(ERR_BADLY_FORMED_COMMAND_LINE);
    }

    config_file_name.clone_from(&args[ARGS_IDX_CONFIG_FILE]);

    // if "whinge" is provided (arg is already validated above)
    if args.len() == MAX_COMMAND_LINE_ARGS {
        debug_assert!(args[ARGS_IDX_WHINGE_MODE] == "whinge"); 
        WHINGE_MODE.store(true, Ordering::SeqCst);
    }

    Ok(())
}

fn recite(play_title: &String, play: &Play) {
    println!("TODO: PLAY TITLE: {}", play_title);

    // TODO: why not destructure in the for loop itself?
    for line_play in play {
        match line_play {
            (_, character_name, line_text) => {
                println!("TODO: {} {}", character_name, line_text);
            }
        }
    }
}

fn main() -> Result<(), u8> {
    let mut config_file_name = String::new();
    parse_args(&mut config_file_name)?;

    recite(&String::from("bobby"), &Vec::new());

    Ok(())
}

