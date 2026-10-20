// TODO: add a file header comment explaining this module's purpose (grader feedback from Lab 1).
// TODO: add a summary comment above every function (grader feedback from Lab 1).

use super::declarations::*;
use std::sync::atomic::Ordering;

type PlayLines = Vec<(usize, String)>;

pub struct Player {
    name: String,
    lines: PlayLines,
    line_index: usize,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            lines: Vec::new(),
            line_index: 0,
        }
    }

    pub fn prepare(&mut self, part_file_name: &str) -> Result<(), u8> {
        let mut lines = Vec::new();

        grab_trimmed_file_lines(part_file_name, &mut lines)?;

        for line in lines {
            self.add_script_line(&line);
        }

        // TODO: decide sort() (ties ordered by text) vs sort_by_key on line number (stable, keeps file order for duplicates); document the choice in ReadMe.txt.
        self.lines.sort();

        Ok(())
    }

    fn add_script_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }

        if let Some((first_token, rest)) = line.split_once(char::is_whitespace) {
            match first_token.parse::<usize>() {
                Err(_) => {
                    if WHINGE_MODE.load(Ordering::SeqCst) {
                        eprintln!("WARNING: Skipping {}'s line because its first token \"{}\" does not represent a valid usize value", self.name, first_token);
                    }
                }
                Ok(line_number) => {
                    self.lines.push((line_number, rest.trim().to_string()));
                }
            }
        } else if WHINGE_MODE.load(Ordering::SeqCst) {
            // no whitespace at all means the line holds a single token, so it cannot
            // carry both a line number and the text the character speaks
            eprintln!(
                "WARNING: Skipping {}'s line \"{}\" because it has only one token",
                self.name, line
            );
        }
    }

    pub fn speak(&mut self, most_recent_speaker: &mut String) {
        if self.line_index >= self.lines.len() {
            return;
        }

        if self.name != *most_recent_speaker {
            println!("\n{}.", self.name);
            most_recent_speaker.clone_from(&self.name);
        }

        let (_, line_text) = &self.lines[self.line_index];
        println!("{}", line_text);
        self.line_index += 1;
    }

    pub fn next_line(&self) -> Option<usize> {
        // map() transforms the inner value of the option, or
        // returns the contained error (monadic operation)
        self.lines
            .get(self.line_index)
            .map(|&(line_number, _)| line_number)
    }
}
