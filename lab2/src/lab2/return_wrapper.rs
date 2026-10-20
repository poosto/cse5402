// Preston Meek -- meek@wustl.edu
// Lab 2 -- return_wrapper.rs
// Define the ReturnWrapper struct. It wraps the program's result so main can return it,
// and turns that result into the program's exit code.

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
    // then return the code as the program's exit code.
    fn report(self) -> ExitCode {
        if self.error_code != Self::SUCCESS {
            eprintln!("Error: {}", self.error_code);
        }

        ExitCode::from(self.error_code)
    }
}
