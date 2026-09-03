use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::sync::atomic::Ordering;

use crate::lab1::declarations::ERR_FAILED_TO_GENERATE_SCRIPT;

use super::declarations::{Play, WHINGE_MODE};

type PlayConfig = Vec<(String, String)>;

const IDX_PLAY_TITLE: usize = 0;
const IDX_CHARACTER_FRAGMENTS: usize = 1;
const MIN_FRAGMENTS: usize = 2; 

const LINE_IDX_CHARACTER_NAME: usize = 0;
const LINE_IDX_CHARACTER_FILE: usize = 1;
const LINE_TOKEN_COUNT: usize = 2;

fn add_script_line(play_lines: &mut Play, character_line: &String, character_name: &String) {
    if character_line.is_empty() {
        return;
    } 

    if let Some((first_token, rest)) = character_line.split_once(char::is_whitespace) {
        match first_token.parse::<usize>() {
            Err(_) => {
                if WHINGE_MODE.load(Ordering::SeqCst) {
                    println!("(TODO) ERROR: {} does not represent a valid usize value", first_token);
                } 
            }
            Ok(line_number) => {
                play_lines.push((line_number, character_name.to_string(), rest.to_string())); 
            }
        }
    }
}

fn grab_trimmed_file_lines(file_name: &String, file_lines: &mut Vec<String>) -> Result<(), u8> {
    match File::open(file_name) {
        Err(e) => {
            println!("ERROR: Failed to open file ({:?})", e);
            Err(ERR_FAILED_TO_GENERATE_SCRIPT)
        }
        Ok(file) => {
            let mut buf = BufReader::new(file);
            let mut file_line = String::new();

            loop {
                file_line.clear();

                match buf.read_line(&mut file_line) {
                    Err(e) => {
                        println!("ERROR: Failed to read line in buffer ({:?})", e);
                        return Err(ERR_FAILED_TO_GENERATE_SCRIPT);
                    }
                    // TODO: EOF constant?
                    Ok(0) => return Ok(()),
                    Ok(_) => file_lines.push(file_line.trim().to_string())
                }
            }
        }
    }
}

fn process_config(play_lines: &mut Play, play_config: &PlayConfig) -> Result<(), u8> {
    // TODO: ok instead of match?
    for (character_name, character_file_name) in play_config {
        let mut lines = Vec::<String>::new();
        grab_trimmed_file_lines(character_file_name, &mut lines)?;

        for line in lines {
            add_script_line(play_lines, &line, character_name);
        }
    }

    Ok(())
}

fn add_config(config_line: &String, play_config: &mut PlayConfig) {
    let words: Vec<&str> = config_line.split_whitespace().collect();

    if words.len() != LINE_TOKEN_COUNT && WHINGE_MODE.load(Ordering::SeqCst) {
        println!("(TODO) WARN: Received {} tokens, expected {}", words.len(), LINE_TOKEN_COUNT);
    } 

    if words.len() >= LINE_TOKEN_COUNT {
        play_config.push((words[LINE_IDX_CHARACTER_NAME].to_string(), words[LINE_IDX_CHARACTER_FILE].to_string()));
    }
}

fn read_config(config_file: &String, play_title: &mut String, play_config: &mut PlayConfig) -> Result<(), u8> {
    let mut lines = Vec::<String>::new();
    grab_trimmed_file_lines(config_file, &mut lines)?;

    if lines.len() < MIN_FRAGMENTS {
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
