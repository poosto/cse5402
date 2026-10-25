use std::{
    env::args,
    sync::atomic::{AtomicBool, Ordering},
};

pub static FLAG: AtomicBool = AtomicBool::new(true);

macro_rules! check_cmd_line {
    () => {
        let mut program_args = args();
        let program_name = program_args.next().unwrap();

        if program_args.count() != 0 && FLAG.load(Ordering::Relaxed) {
            eprintln!("Usage: {program_name} (no args)");
        }
    };
    (true) => {
        FLAG.store(true, Ordering::Relaxed);
    };
    (false) => {
        FLAG.store(false, Ordering::Relaxed);
    };
}

fn main() {
    check_cmd_line!();
    check_cmd_line!(false);
    check_cmd_line!();
}
