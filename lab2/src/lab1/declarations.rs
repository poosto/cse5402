// TODO: add a file header comment explaining this module's purpose (lost 1 pt on Lab 1 for missing it).

use std::sync::atomic::AtomicBool;

pub type Play = Vec<(usize, String, String)>;

pub const MIN_COMMAND_LINE_ARGS: usize = 2;
pub const MAX_COMMAND_LINE_ARGS: usize = 3;

pub const ARGS_IDX_PROGRAM_NAME: usize = 0;
pub const ARGS_IDX_CONFIG_FILE: usize = 1;
pub const ARGS_IDX_WHINGE_MODE: usize = 2;

pub const ERR_BADLY_FORMED_COMMAND_LINE: u8 = 1;
pub const ERR_FAILED_TO_GENERATE_SCRIPT: u8 = 2;

pub static WHINGE_MODE: AtomicBool = AtomicBool::new(false);
