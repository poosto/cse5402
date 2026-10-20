// TODO: add a file header comment explaining this file's purpose (grader feedback from Lab 1).
// TODO: add a summary comment above every function (new, report) (grader feedback from Lab 1).

use std::process::{ExitCode, Termination};

pub struct ReturnWrapper {
    error_code: u8,
}

impl ReturnWrapper {
    const SUCCESS: u8 = 0;

    pub fn new(result: Result<(), u8>) -> Self {
        match result {
            Ok(()) => ReturnWrapper {
                error_code: Self::SUCCESS,
            },
            Err(error_code) => ReturnWrapper { error_code },
        }
    }
}

impl Termination for ReturnWrapper {
    fn report(self) -> ExitCode {
        if self.error_code != Self::SUCCESS {
            eprintln!("Error: {}", self.error_code);
        }

        ExitCode::from(self.error_code)
    }
}
