// Preston Meek -- meek@wustl.edu
// Lab 2 -- scene_fragment.rs
// Define the SceneFragment struct. A fragment reads a configuration file, sets up one player
// for each character, recites their lines in order, and prints who enters and exits.

use super::declarations::*;
use super::player::Player;
use std::collections::HashSet;
use std::sync::atomic::Ordering;

const LINE_IDX_CHARACTER_NAME: usize = 0;
const LINE_IDX_CHARACTER_FILE: usize = 1;
const LINE_TOKEN_COUNT: usize = 2;

type PlayConfig = Vec<(String, String)>;

pub struct SceneFragment {
    pub(crate) title: String,
    players: Vec<Player>,
}

impl SceneFragment {
    // Create a scene fragment with the given title and no players.
    pub fn new(title: &str) -> Self {
        let title = title.to_string();

        SceneFragment {
            title,
            players: Vec::new(),
        }
    }

    // Make a player for each character in the play config, read its part file, and sort
    // the players. Return an error if any part file cannot be read.
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

        self.players.sort();

        Ok(())
    }

    // Add one configuration line to the play config. The first two tokens are the character
    // name and part file. Lines with fewer than two tokens are skipped, and extra tokens are
    // ignored. In whinge mode, warn if a line does not have exactly two tokens.
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

    // Read the configuration file and add each non-blank line to the play config.
    // Return an error if the file cannot be read, has no non-blank lines,
    // or has no valid character lines.
    fn read_config(config_file: &str, play_config: &mut PlayConfig) -> Result<(), u8> {
        let mut lines = Vec::<String>::new();

        // a missing configuration file is fatal
        if grab_trimmed_file_lines(config_file, &mut lines).is_err() {
            eprintln!("ERROR: Failed to read configuration file {}", config_file);
            return Err(ERR_FAILED_TO_GENERATE_SCRIPT);
        }

        // Lines are already trimmed, but this also removes empty whitespace lines
        lines.retain(|line| !line.is_empty());

        // Check if a configuration file is empty
        if lines.is_empty() {
            eprintln!("ERROR: Configuration file {} cannot be empty", config_file);
            return Err(ERR_FAILED_TO_GENERATE_SCRIPT);
        }

        for line in &lines {
            Self::add_config(line, play_config);
        }

        // Check if a configuration file only had junk
        // i.e., instead of being actually empty, it only has semantically meaningless lines
        // TODO: document this decision
        if play_config.is_empty() {
            eprintln!(
                "ERROR: Configuration file {} must have at least one valid line",
                config_file
            );
            return Err(ERR_FAILED_TO_GENERATE_SCRIPT);
        }

        Ok(())
    }

    // Read the configuration file and set up all of the fragment's players.
    // Return an error if any step fails.
    pub fn prepare(&mut self, config_file: &str) -> Result<(), u8> {
        let mut config = PlayConfig::new();
        Self::read_config(config_file, &mut config)?;
        self.process_config(&config)
    }

    // Print every player's lines in line number order. In whinge mode, warn about
    // missing or duplicated line numbers.
    pub fn recite(&mut self) {
        let mut expected_line = 0;
        let mut most_recent_speaker = String::new();
        let whinge = WHINGE_MODE.load(Ordering::SeqCst);

        loop {
            // Run through each line
            // Use filter_map to remove None values when calling next_line()
            // Map Some values to include the index of the Player for later
            // (we can't hold a ref to it and call speak())
            // Take the min of the tuple
            //
            // If players share a line number, the player sorted earlier in the players vector
            // speaks first. Players are sorted by first line number, and players tied on their
            // first line keep their config file order. This matches how ties keep file order within a player.
            // TODO: document this
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

    // Helper function that takes A and not B
    // or, the Players in Self but not in Other based on the name field
    fn players_not_in<'a>(
        &'a self,
        other: &'a [Player],
    ) -> impl DoubleEndedIterator<Item = &'a Player> {
        let other = other
            .iter()
            .map(|player| &player.name)
            .collect::<HashSet<_>>();

        self.players
            .iter()
            .filter(move |&player| !other.contains(&player.name))
    }

    // Reusable across enter and enter_all
    // follows_fragment is true when an earlier fragment's output precedes this one
    fn enter_with(&self, other: &[Player], follows_fragment: bool) {
        let has_title = !self.title.trim().is_empty();
        let mut entering = self.players_not_in(other).peekable();

        if has_title {
            // a blank line separates a new scene's title from the previous fragment's output,
            // but the first fragment's title starts the script, so nothing comes before it
            if follows_fragment {
                println!();
            }

            println!("{}", self.title);

            // a blank line separates the title from the entrances (as in hamlet_act_ii_scene_1.txt);
            // without entrances, speak's leading newline already separates it from the first line
            if entering.peek().is_some() {
                println!();
            }
        }

        entering.for_each(|player| println!("[Enter {}.]", player.name));
    }

    // Reusable across exit and exit_all
    fn exit_with(&self, other: &[Player]) {
        let mut exiting = self.players_not_in(other).rev().peekable();

        // a blank line separates the last spoken line from the exits, but only if anyone exits
        if exiting.peek().is_some() {
            println!();
        }

        exiting.for_each(|player| println!("[Exit {}.]", player.name));
    }

    // Print the title, if there is one, and announce the players who were not in the
    // previous fragment.
    pub fn enter(&self, other: &Self) {
        self.enter_with(&other.players, true);
    }

    // Used for the first fragment. Print the title and announce that every player enters.
    pub fn enter_all(&self) {
        // Everyone enters, so no shared players, thus the slice is empty
        self.enter_with(&[], false);
    }

    // Announce, in reverse order, the players who are not in the next fragment.
    pub fn exit(&self, other: &Self) {
        self.exit_with(&other.players);
    }

    // Used for the last fragment. Announce, in reverse order, that every player exits.
    pub fn exit_all(&self) {
        // Everyone exits, so no shared players, thus the slice is empty
        self.exit_with(&[]);
    }
}

// Unit tests for SceneFragment::players_not_in, which decides who enters and exits
// between fragments: players must be matched by name (not by Player's ordering-based
// equality) and returned in the fragment's order, reversible for exits.
// Generated with Claude
#[cfg(test)]
mod tests {
    use super::*;

    // Builds an untitled fragment holding silent players with the given names, in order.
    fn fragment_with(names: &[&str]) -> SceneFragment {
        let mut fragment = SceneFragment::new("");

        for name in names {
            fragment.players.push(Player::new(name));
        }

        fragment
    }

    // Names of the players in self but not in other, in fragment order.
    fn names_not_in(fragment: &SceneFragment, other: &SceneFragment) -> Vec<String> {
        fragment
            .players_not_in(&other.players)
            .map(|player| player.name.clone())
            .collect()
    }

    // Names of the players in self but not in other, in reverse fragment order (as exit uses).
    fn names_not_in_rev(fragment: &SceneFragment, other: &SceneFragment) -> Vec<String> {
        fragment
            .players_not_in(&other.players)
            .rev()
            .map(|player| player.name.clone())
            .collect()
    }

    // Hamlet II.1a -> II.1b: Polonius stays on stage, Reynaldo exits, Ophelia enters.
    #[test]
    fn hamlet_ii_1a_to_1b() {
        let ii_1a = fragment_with(&["Polonius", "Reynaldo"]);
        let ii_1b = fragment_with(&["Polonius", "Ophelia"]);

        assert_eq!(names_not_in(&ii_1b, &ii_1a), vec!["Ophelia"]);
        assert_eq!(names_not_in_rev(&ii_1a, &ii_1b), vec!["Reynaldo"]);
    }

    // With no other players (as enter_all/exit_all pass), everyone is returned,
    // forward for entrances and reversed for exits.
    #[test]
    fn empty_other_returns_everyone() {
        let fragment = fragment_with(&["King", "Queen", "Rosencrantz"]);
        let empty = fragment_with(&[]);

        assert_eq!(
            names_not_in(&fragment, &empty),
            vec!["King", "Queen", "Rosencrantz"]
        );
        assert_eq!(
            names_not_in_rev(&fragment, &empty),
            vec!["Rosencrantz", "Queen", "King"]
        );
    }

    // The enter_all/exit_all path, which passes an empty slice rather than a fragment.
    #[test]
    fn empty_slice_returns_everyone() {
        let fragment = fragment_with(&["King", "Queen"]);
        let names: Vec<&str> = fragment
            .players_not_in(&[])
            .map(|player| player.name.as_str())
            .collect();

        assert_eq!(names, vec!["King", "Queen"]);
    }

    // Identical casts produce no entrances or exits.
    #[test]
    fn same_players_returns_no_one() {
        let a = fragment_with(&["Polonius", "Ophelia"]);
        let b = fragment_with(&["Ophelia", "Polonius"]);

        assert!(names_not_in(&a, &b).is_empty());
        assert!(names_not_in(&b, &a).is_empty());
    }

    // Disjoint casts mean everyone enters and everyone exits.
    #[test]
    fn disjoint_players_returns_everyone() {
        let scene_1 = fragment_with(&["Polonius", "Ophelia"]);
        let scene_2 = fragment_with(&["King", "Queen"]);

        assert_eq!(names_not_in(&scene_2, &scene_1), vec!["King", "Queen"]);
        assert_eq!(
            names_not_in_rev(&scene_1, &scene_2),
            vec!["Ophelia", "Polonius"]
        );
    }

    // Players that remain keep the fragment's order, with shared players skipped
    // wherever they appear.
    #[test]
    fn order_is_preserved_around_shared_players() {
        let fragment = fragment_with(&["A", "B", "C", "D", "E"]);
        let other = fragment_with(&["D", "B"]);

        assert_eq!(names_not_in(&fragment, &other), vec!["A", "C", "E"]);
        assert_eq!(names_not_in_rev(&fragment, &other), vec!["E", "C", "A"]);
    }

    // Silent players all compare equal under Player's PartialEq, so this checks that
    // matching is by name: a differently named silent player still enters.
    #[test]
    fn matches_by_name_not_player_equality() {
        let fragment = fragment_with(&["Ophelia"]);
        let other = fragment_with(&["Reynaldo"]);

        assert!(fragment.players[0] == other.players[0]);
        assert_eq!(names_not_in(&fragment, &other), vec!["Ophelia"]);
    }

    // An empty fragment has no one to enter or exit.
    #[test]
    fn empty_self_returns_no_one() {
        let empty = fragment_with(&[]);
        let other = fragment_with(&["Polonius"]);

        assert!(names_not_in(&empty, &other).is_empty());
    }
}
