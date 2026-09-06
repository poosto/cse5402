use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::sync::atomic::Ordering;

use crate::lab1::declarations::ERR_FAILED_TO_GENERATE_SCRIPT;

use super::declarations::{Play, WHINGE_MODE};

type PlayConfig = Vec<(String, String)>;

const IDX_PLAY_TITLE: usize = 0;
const IDX_CHARACTER_FRAGMENTS: usize = 1;
const MIN_CONFIG_LINES: usize = 2;

const LINE_IDX_CHARACTER_NAME: usize = 0;
const LINE_IDX_CHARACTER_FILE: usize = 1;
const LINE_TOKEN_COUNT: usize = 2;

// read_line reports zero bytes read only at end of file
const BYTES_READ_AT_EOF: usize = 0;

fn add_script_line(play_lines: &mut Play, character_line: &String, character_name: &String) {
    if character_line.is_empty() {
        return;
    } 

    if let Some((first_token, rest)) = character_line.split_once(char::is_whitespace) {
        match first_token.parse::<usize>() {
            Err(_) => {
                if WHINGE_MODE.load(Ordering::SeqCst) {
                    println!("WARNING: Skipping {}'s line because its first token \"{}\" does not represent a valid usize value", character_name, first_token);
                } 
            }
            Ok(line_number) => {
                play_lines.push((line_number, character_name.to_string(), rest.trim().to_string()));
            }
        }
    } else if WHINGE_MODE.load(Ordering::SeqCst) {
        // no whitespace at all means the line holds a single token, so it cannot
        // carry both a line number and the text the character speaks
        println!("WARNING: Skipping {}'s line \"{}\" because it has only one token", character_name, character_line);
    }
}

fn grab_trimmed_file_lines(file_name: &String, file_lines: &mut Vec<String>) -> Result<(), u8> {
    match File::open(file_name) {
        Err(e) => {
            if WHINGE_MODE.load(Ordering::SeqCst) {
                println!("WARNING: Failed to open file {} ({})", file_name, e);
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
                            println!("WARNING: Failed to read a line from file {} ({})", file_name, e);
                        }

                        return Err(ERR_FAILED_TO_GENERATE_SCRIPT);
                    }
                    Ok(BYTES_READ_AT_EOF) => return Ok(()),
                    Ok(_) => file_lines.push(file_line.trim().to_string())
                }
            }
        }
    }
}

fn process_config(play_lines: &mut Play, play_config: &PlayConfig) -> Result<(), u8> {
    let mut valid_character_count = 0;

    for (character_name, character_file_name) in play_config {
        let mut lines = Vec::<String>::new();

        // a part file that cannot be opened makes its configuration line badly
        // formed rather than fatal, so skip that character and keep going
        if grab_trimmed_file_lines(character_file_name, &mut lines).is_err() {
            if WHINGE_MODE.load(Ordering::SeqCst) {
                println!("WARNING: Skipping character {} whose part file {} could not be read", character_name, character_file_name);
            }

            continue;
        }

        valid_character_count += 1;

        for line in lines {
            add_script_line(play_lines, &line, character_name);
        }
    }

    // a configuration naming no character whose part file opened cannot yield a script
    if valid_character_count == 0 {
        println!("ERROR: No character in the configuration had a part file that could be read");
        return Err(ERR_FAILED_TO_GENERATE_SCRIPT);
    }

    Ok(())
}

fn add_config(config_line: &String, play_config: &mut PlayConfig) {
    let words: Vec<&str> = config_line.split_whitespace().collect();

    if words.len() != LINE_TOKEN_COUNT && WHINGE_MODE.load(Ordering::SeqCst) {
        println!("WARNING: Configuration line \"{}\" has {} tokens, expected {}", config_line, words.len(), LINE_TOKEN_COUNT);
    } 

    if words.len() >= LINE_TOKEN_COUNT {
        play_config.push((words[LINE_IDX_CHARACTER_NAME].to_string(), words[LINE_IDX_CHARACTER_FILE].to_string()));
    }
}

fn read_config(config_file: &String, play_title: &mut String, play_config: &mut PlayConfig) -> Result<(), u8> {
    let mut lines = Vec::<String>::new();

    // unlike a character's part file, a missing configuration file is fatal
    if grab_trimmed_file_lines(config_file, &mut lines).is_err() {
        println!("ERROR: Failed to read configuration file {}", config_file);
        return Err(ERR_FAILED_TO_GENERATE_SCRIPT);
    }

    // Lines are already trimmed, but this also removes empty whitespace lines
    lines.retain(|line| !line.is_empty());

    if lines.len() < MIN_CONFIG_LINES {
        println!("ERROR: Configuration file {} needs a title and at least one character definition", config_file);
        return Err(ERR_FAILED_TO_GENERATE_SCRIPT);
    }

    *play_title = lines[IDX_PLAY_TITLE].clone();

    for line in &lines[IDX_CHARACTER_FRAGMENTS..] {
        add_config(line, play_config);
    }

    Ok(())
}

pub fn script_gen(config_file: &String, play_title: &mut String, play_lines: &mut Play) -> Result<(), u8> {
    let mut config = PlayConfig::new(); 
    read_config(config_file, play_title, &mut config)?;

    process_config(play_lines, &config)
}
