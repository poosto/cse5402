// Preston Meek -- meek@wustl.edu
// Lab 2 -- player.rs
// Define the Player struct. A player reads its part file, keeps its lines sorted by line number,
// and speaks them one at a time. Players can also be compared and sorted by their first line.

use super::declarations::*;
use std::cmp;
use std::sync::atomic;

type PlayLines = Vec<(usize, String)>;

pub struct Player {
    pub(crate) name: String,
    lines: PlayLines,
    line_index: usize,
}

impl Player {
    // Create a player with the given name and no lines.
    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            lines: Vec::new(),
            line_index: 0,
        }
    }

    // Read the player's part file, add each valid line, and sort the lines by line number.
    // Return an error if the file cannot be read.
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

    // Add one line from the part file. The first token is the line number and the rest is
    // the text. Blank lines are skipped. In whinge mode, warn about lines that have only
    // one token or do not start with a valid line number.
    fn add_script_line(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }

        if let Some((first_token, rest)) = line.split_once(char::is_whitespace) {
            match first_token.parse::<usize>() {
                Err(_) => {
                    if WHINGE_MODE.load(atomic::Ordering::SeqCst) {
                        eprintln!("WARNING: Skipping {}'s line because its first token \"{}\" does not represent a valid usize value", self.name, first_token);
                    }
                }
                Ok(line_number) => {
                    self.lines.push((line_number, rest.trim().to_string()));
                }
            }
        } else if WHINGE_MODE.load(atomic::Ordering::SeqCst) {
            // no whitespace at all means the line holds a single token, so it cannot
            // carry both a line number and the text the character speaks
            eprintln!(
                "WARNING: Skipping {}'s line \"{}\" because it has only one token",
                self.name, line
            );
        }
    }

    // Print the player's next line and move on to the one after it. If a different player
    // spoke last, print this player's name first. Do nothing if no lines are left.
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

    // Return the line number of the player's next line, or None if no lines are left.
    pub fn next_line(&self) -> Option<usize> {
        // map() transforms the inner value of the option, or
        // returns the contained error (monadic operation)
        self.lines
            .get(self.line_index)
            .map(|&(line_number, _)| line_number)
    }
}

impl PartialEq for Player {
    // Two players are equal if both are silent, or if both have lines and their
    // first line numbers match.
    fn eq(&self, other: &Self) -> bool {
        (self.lines.is_empty() && other.lines.is_empty())
            || (!self.lines.is_empty()
                && !other.lines.is_empty()
                && self.lines[0].0 == other.lines[0].0)
    }
}

// Marker trait
// A promise about how PartialEq behaves (reflexivity)
// There is nothing new to actually implement, and there'd be no other clear return type for
// PartialEq. Also, this lets generic code require Eq.
impl Eq for Player {}

impl PartialOrd for Player {
    // Players can always be compared, so this just wraps cmp's result in Some.
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Ord isn't a marker trait because it carries new info
// It means something can *always* be ordered, versus sometimes (Option)
// Whereas PartialEq can always compare two things, but reflexivity doesn't hold.
// Incomparable or not equal is basically the same thing for equivalence,
// but for ordering there is a 4th option (not orderable).
impl Ord for Player {
    // Silent players come before players with lines. Players with lines are ordered
    // by their first line number.
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self == other {
            cmp::Ordering::Equal
        } else if (self.lines.is_empty() && !other.lines.is_empty())
            || (!self.lines.is_empty()
                && !other.lines.is_empty()
                && self.lines[0].0 < other.lines[0].0)
        {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Greater
        }
    }
}

// Unit tests for Player's PartialEq/Eq/PartialOrd/Ord implementations, checking each
// case of the spec's ordering rules (silent vs. speaking players, first line numbers)
// and that sorting a vector of players puts them in the expected order.
// Generated with Claude
#[cfg(test)]
mod tests {
    use super::*;

    // Builds a player with the given name and one line per given line number,
    // without reading a part file.
    fn player_with_lines(name: &str, line_numbers: &[usize]) -> Player {
        let mut player = Player::new(name);

        for &line_number in line_numbers {
            player
                .lines
                .push((line_number, format!("line {}", line_number)));
        }

        player
    }

    // Two silent players are equal.
    #[test]
    fn both_silent_are_equal() {
        let a = player_with_lines("A", &[]);
        let b = player_with_lines("B", &[]);

        assert_eq!(a.cmp(&b), cmp::Ordering::Equal);
        assert!(a == b);
    }

    // A silent player is less than a player with lines, and the speaking
    // player is greater than the silent one.
    #[test]
    fn silent_is_less_than_speaking() {
        let silent = player_with_lines("Silent", &[]);
        let speaking = player_with_lines("Speaking", &[0]);

        assert_eq!(silent.cmp(&speaking), cmp::Ordering::Less);
        assert_eq!(speaking.cmp(&silent), cmp::Ordering::Greater);
        assert!(silent < speaking);
        assert!(speaking > silent);
        assert!(silent != speaking);
    }

    // Players whose first line numbers match are equal, even if their names
    // and later lines differ.
    #[test]
    fn same_first_line_is_equal() {
        let a = player_with_lines("A", &[3, 4, 9]);
        let b = player_with_lines("B", &[3]);

        assert_eq!(a.cmp(&b), cmp::Ordering::Equal);
        assert_eq!(b.cmp(&a), cmp::Ordering::Equal);
        assert!(a == b);
    }

    // A player with a lower first line number is less than one with a higher
    // first line number, regardless of how many lines each has.
    #[test]
    fn lower_first_line_is_less() {
        let early = player_with_lines("Early", &[2, 50, 51]);
        let late = player_with_lines("Late", &[7]);

        assert_eq!(early.cmp(&late), cmp::Ordering::Less);
        assert_eq!(late.cmp(&early), cmp::Ordering::Greater);
        assert!(early < late);
        assert!(late > early);
        assert!(early <= late);
        assert!(late >= early);
    }

    // Comparisons use the first line in the vector, not the next line to be
    // spoken, so a player compares the same before and after speaking.
    #[test]
    fn comparison_ignores_line_index() {
        let mut a = player_with_lines("A", &[1, 10]);
        let b = player_with_lines("B", &[5]);

        a.line_index = 1;

        assert_eq!(a.cmp(&b), cmp::Ordering::Less);
    }

    // Line numbers at the extremes of usize compare correctly.
    #[test]
    fn extreme_line_numbers() {
        let zero = player_with_lines("Zero", &[0]);
        let max = player_with_lines("Max", &[usize::MAX]);
        let other_max = player_with_lines("OtherMax", &[usize::MAX]);

        assert!(zero < max);
        assert!(max == other_max);
    }

    // partial_cmp always agrees with cmp, and eq agrees with cmp returning
    // Equal, for every pair drawn from a mix of silent and speaking players.
    #[test]
    fn traits_are_consistent() {
        let players = vec![
            player_with_lines("Silent1", &[]),
            player_with_lines("Silent2", &[]),
            player_with_lines("Zero", &[0]),
            player_with_lines("Three", &[3, 8]),
            player_with_lines("AlsoThree", &[3]),
            player_with_lines("Ten", &[10]),
        ];

        for a in &players {
            for b in &players {
                assert_eq!(a.partial_cmp(b), Some(a.cmp(b)));
                assert_eq!(a == b, a.cmp(b) == cmp::Ordering::Equal);
                assert_eq!(a.cmp(b), b.cmp(a).reverse());
            }
        }
    }

    // Sorting a mixed vector puts silent players first, then speaking players
    // in order of their first line numbers.
    #[test]
    fn sort_orders_players() {
        let mut players = vec![
            player_with_lines("Ten", &[10]),
            player_with_lines("Silent1", &[]),
            player_with_lines("Two", &[2, 3]),
            player_with_lines("Zero", &[0]),
            player_with_lines("Silent2", &[]),
            player_with_lines("Five", &[5]),
        ];

        players.sort();

        let names: Vec<&str> = players.iter().map(|player| player.name.as_str()).collect();

        // sort() is stable, so the equal silent players keep their relative order
        assert_eq!(
            names,
            vec!["Silent1", "Silent2", "Zero", "Two", "Five", "Ten"]
        );
    }
}
