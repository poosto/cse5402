// TODO: add a file header comment explaining this module's purpose (grader feedback from Lab 1).
// TODO: add a summary comment above every function (grader feedback from Lab 1).

use super::declarations::*;
use super::scene_fragment::SceneFragment;
use std::sync::atomic::Ordering;

type ScriptConfig = Vec<(bool, String)>;
type Fragments = Vec<SceneFragment>;

pub struct Play {
    fragments: Fragments,
}

impl Play {
    pub fn new() -> Self {
        Play {
            fragments: Fragments::new(),
        }
    }

    fn process_config(&mut self, script_config: &ScriptConfig) -> Result<(), u8> {
        let mut title = String::new();

        for (is_new_scene, config_line_text) in script_config {
            if *is_new_scene {
                title = config_line_text.clone();
            } else {
                let mut fragment = SceneFragment::new(&title);
                title.clear();

                match fragment.prepare(config_line_text) {
                    Err(_) => return Err(ERR_FAILED_TO_GENERATE_SCRIPT),
                    Ok(()) => self.fragments.push(fragment),
                }
            }
        }

        Ok(())
    }

    fn add_config(config_line: &str, script_config: &mut ScriptConfig) {
        let words: Vec<&str> = config_line.split_whitespace().collect();

        let whinge = WHINGE_MODE.load(Ordering::SeqCst);
        const SCENE_DIRECTIVE: &str = "[scene]";

        match words.as_slice() {
            [] => return,
            [SCENE_DIRECTIVE] => {
                if whinge {
                    eprintln!(
                        "WARNING: Skipping script line \"{}\" because its {} directive has no scene title",
                        config_line, SCENE_DIRECTIVE
                    );
                }
            }
            [SCENE_DIRECTIVE, title @ ..] => script_config.push((true, title.join(" "))),
            [config_file, extra @ ..] => {
                script_config.push((false, config_file.to_string()));

                if whinge && !extra.is_empty() {
                    eprintln!(
                        "WARNING: Ignoring extra tokens \"{}\" after configuration file {} in script line \"{}\"",
                        extra.join(" "),
                        config_file,
                        config_line
                    );
                }
            }
        }
    }

    fn read_config(script_file: &str, script_config: &mut ScriptConfig) -> Result<(), u8> {
        let mut lines = Vec::<String>::new();

        // a missing configuration file is fatal
        if grab_trimmed_file_lines(script_file, &mut lines).is_err() {
            eprintln!("ERROR: Failed to read script file {}", script_file);
            return Err(ERR_FAILED_TO_GENERATE_SCRIPT);
        }

        // Lines are already trimmed, but this also removes empty whitespace lines
        lines.retain(|line| !line.is_empty());

        if lines.is_empty() {
            eprintln!("ERROR: Script file {script_file} is empty");
            return Err(ERR_FAILED_TO_GENERATE_SCRIPT);
        }

        for line in &lines {
            Self::add_config(line, script_config);
        }

        Ok(())
    }

    pub fn prepare(&mut self, script_file: &str) -> Result<(), u8> {
        let mut config = ScriptConfig::new();
        Self::read_config(script_file, &mut config)?;
        self.process_config(&config)?;

        match self.fragments.first() {
            None => {
                eprintln!(
                    "ERROR: Script file {} does not name any configuration files",
                    script_file
                );
                Err(ERR_FAILED_TO_GENERATE_SCRIPT)
            }
            Some(first) if first.title.is_empty() => {
                eprintln!(
                    "ERROR: Script file {} must give a [scene] title before its first configuration file",
                    script_file
                );
                Err(ERR_FAILED_TO_GENERATE_SCRIPT)
            }
            Some(_) => Ok(()),
        }
    }

    pub fn recite(&mut self) {
        // Can't cleanly do a for_each because a &self.fragments[i - 1] is another borrow,
        // since the iterator borrows for its entire duration.
        for idx in 0..self.fragments.len() {
            if idx == 0 {
                self.fragments[idx].enter_all();
            } else {
                self.fragments[idx].enter(&self.fragments[idx - 1]);
            }

            self.fragments[idx].recite();

            if idx == self.fragments.len() - 1 {
                self.fragments[idx].exit_all();
            } else {
                self.fragments[idx].exit(&self.fragments[idx + 1]);
            }
        }
    }
}
