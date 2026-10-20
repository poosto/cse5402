// TODO: add a file header comment explaining this module's purpose (grader feedback from Lab 1).
// TODO: add a summary comment above every function (grader feedback from Lab 1).

use super::declarations::*;
use super::player::Player;
use std::sync::atomic::Ordering;

const IDX_PLAY_TITLE: usize = 0;
const IDX_CHARACTER_FRAGMENTS: usize = 1;
const MIN_CONFIG_LINES: usize = 2;

const LINE_IDX_CHARACTER_NAME: usize = 0;
const LINE_IDX_CHARACTER_FILE: usize = 1;
const LINE_TOKEN_COUNT: usize = 2;

type PlayConfig = Vec<(String, String)>;

pub struct Play {
    title: String,
    players: Vec<Player>,
}

impl Play {
    pub fn new() -> Self {
        Play {
            title: String::new(),
            players: Vec::new(),
        }
    }

    fn process_config(&mut self, play_config: &PlayConfig) -> Result<(), u8> {
        for (character_name, character_file_name) in play_config {
            let mut player = Player::new(character_name);

            // TODO: document in ReadMe.txt that an unreadable part file is now fatal (Lab 2 spec: process_config returns an error
            // if prepare fails) whereas Lab 1 skipped it with a warning (e.g. ../lab1/dir_part_config.txt); also that prepare
            // is called before the push rather than after, as the spec words it.
            if player.prepare(character_file_name).is_err() {
                eprintln!(
                    "ERROR: Failed to read part file {} for character {}",
                    character_file_name, character_name
                );
                return Err(ERR_FAILED_TO_GENERATE_SCRIPT);
            }

            self.players.push(player);
        }

        Ok(())
    }

    fn add_config(config_line: &str, play_config: &mut PlayConfig) {
        let words: Vec<&str> = config_line.split_whitespace().collect();

        if words.len() != LINE_TOKEN_COUNT && WHINGE_MODE.load(Ordering::SeqCst) {
            eprintln!(
                "WARNING: Configuration line \"{}\" has {} tokens, expected {}",
                config_line,
                words.len(),
                LINE_TOKEN_COUNT
            );
        }

        if words.len() >= LINE_TOKEN_COUNT {
            play_config.push((
                words[LINE_IDX_CHARACTER_NAME].to_string(),
                words[LINE_IDX_CHARACTER_FILE].to_string(),
            ));
        }
    }

    fn read_config(
        config_file: &str,
        play_title: &mut String,
        play_config: &mut PlayConfig,
    ) -> Result<(), u8> {
        let mut lines = Vec::<String>::new();

        // a missing configuration file is fatal
        if grab_trimmed_file_lines(config_file, &mut lines).is_err() {
            eprintln!("ERROR: Failed to read configuration file {}", config_file);
            return Err(ERR_FAILED_TO_GENERATE_SCRIPT);
        }

        // Lines are already trimmed, but this also removes empty whitespace lines
        lines.retain(|line| !line.is_empty());

        if lines.len() < MIN_CONFIG_LINES {
            eprintln!(
                "ERROR: Configuration file {} needs a title and at least one character definition",
                config_file
            );
            return Err(ERR_FAILED_TO_GENERATE_SCRIPT);
        }

        *play_title = lines[IDX_PLAY_TITLE].clone();

        for line in &lines[IDX_CHARACTER_FRAGMENTS..] {
            Self::add_config(line, play_config);
        }

        Ok(())
    }

    pub fn prepare(&mut self, config_file: &str) -> Result<(), u8> {
        let mut config = PlayConfig::new();
        Self::read_config(config_file, &mut self.title, &mut config)?;
        self.process_config(&config)
    }

    pub fn recite(&mut self) {
        println!("{}", self.title);

        let mut expected_line = 0;
        let mut most_recent_speaker = String::new();
        let whinge = WHINGE_MODE.load(Ordering::SeqCst);

        loop {
            // Run through each line
            // Use filter_map to remove None values when calling next_line()
            // Map Some values to include the index of the Player for later
            // (we can't hold a ref to it and call speak())
            // Take the min of the tuple
            let next_line = self
                .players
                .iter()
                .enumerate()
                .filter_map(|(idx, player)| player.next_line().map(|line| (line, idx)))
                .min();

            match next_line {
                None => break,
                Some((line, idx)) => {
                    if whinge {
                        // a gap is reported once as a range rather than line by line, since
                        // a huge line number could otherwise produce billions of warnings
                        if line > expected_line {
                            // line > expected_line >= 0, so line - 1 cannot underflow
                            let last_missing = line - 1;

                            if last_missing == expected_line {
                                eprintln!("WARNING: Line {} is missing", expected_line);
                            } else {
                                eprintln!(
                                    "WARNING: Lines {} to {} are missing",
                                    expected_line, last_missing
                                );
                            }
                        } else if line < expected_line {
                            eprintln!("WARNING: Line {} is duplicated", line);
                        }
                    }

                    self.players[idx].speak(&mut most_recent_speaker);
                    // saturate so a line numbered usize::MAX cannot overflow
                    expected_line = line.saturating_add(1);
                }
            }
        }
    }
}
