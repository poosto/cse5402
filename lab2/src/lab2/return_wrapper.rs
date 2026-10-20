use std::process::{ExitCode, Termination};

pub struct ReturnWrapper {
    error_code: u8,
}

impl ReturnWrapper {
    pub fn new(result: Result<(), u8>) -> Self {
        match result {
            Ok(()) => ReturnWrapper { error_code: 0 },
            Err(error_code) => ReturnWrapper { error_code },
        }
    }
}

impl Termination for ReturnWrapper {
    fn report(self) -> ExitCode {
        if self.error_code != 0 {
            eprintln!("Error: {}", self.error_code);
        }

        ExitCode::from(self.error_code)
    }
}
