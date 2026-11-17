// Preston Meek -- meek@wustl.edu
// Lab 3 -- return_wrapper.rs
// Define the ReturnWrapper struct. It wraps the program's result so main can return it,
// and turns that result into the program's exit code.

use super::declarations::{write_error, ERR_FAILED_TO_WRITE_OUTPUT};
use std::process::{ExitCode, Termination};

pub struct ReturnWrapper {
    error_code: u8,
}

impl ReturnWrapper {
    const SUCCESS: u8 = 0;

    // Wrap the program's result. Ok becomes the success code 0, and Err keeps its error code.
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
    // Called when main returns. Print the error code to stderr if the program failed,
    // then return the code as the program's exit code. If the error code cannot be
    // written, return the write error code instead, like the rest of the program does.
    fn report(self) -> ExitCode {
        if self.error_code != Self::SUCCESS && write_error!("Error: {}", self.error_code).is_err() {
            return ExitCode::from(ERR_FAILED_TO_WRITE_OUTPUT);
        }

        ExitCode::from(self.error_code)
    }
}
