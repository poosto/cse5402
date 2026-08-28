use std::env;

const BAD_ARGS: u8 = 1;

fn main() -> Result<(), u8> {
    let mut arg_vec: std::vec::Vec<String> = Vec::new();

    for arg in env::args() {
        arg_vec.push(arg);
    }

    println!("{}", arg_vec[arg_vec.len()]);

    if arg_vec.len() == 1 {
        println!("usage: ./{} <arg1> [<arg2> ...]", arg_vec[0]);
        Err(BAD_ARGS)
    } else {
        println!(
            "arguments passed to ./{} were {:?}",
            arg_vec[0],
            &arg_vec[1..]
        );
        Ok(())
    }
}
