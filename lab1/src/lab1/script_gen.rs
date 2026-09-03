use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::sync::atomic::Ordering;

use super::declarations::{Play, WHINGE_MODE};

type PlayConfig = Vec<(String, String)>;

const IDX_PLAY_TITLE: usize = 0;
const IDX_CHARACTER_FRAGMENTS: usize = 1;

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
