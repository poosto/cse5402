// TODO: add a file header comment explaining this module's purpose (grader feedback from Lab 1).
// TODO: add a summary comment above every function (grader feedback from Lab 1).

use super::declarations::*;
use super::player::Player;

type PlayConfig = Vec<(String, String)>;

pub struct Play {
    scene_title: String,
    characters: Vec<Player>,
}

impl Play {
    fn new() -> Self {
        Play {
            scene_title: String::new(),
            characters: Vec::new(),
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

            self.characters.push(player);
        }

        Ok(())
    }

    fn add_config(&self, config_line: &str, play_config: &mut PlayConfig) {
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
}
