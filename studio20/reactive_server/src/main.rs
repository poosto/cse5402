use std::env;

const ERR_INVALID_ARGS: u8 = 1;
const ERR_CANNOT_PARSE_ARGS: u8 = 2;

const ARG_IDX_PORT_NUMBER: usize = 0;

fn main() -> Result<(), u8> {
    let program_name = env::args().take(1).collect::<String>();
    let program_args = env::args().skip(1).collect::<Vec<_>>();

    if program_args.is_empty() || program_args.len() > 1 {
        eprintln!("Usage: {program_name} port_number");
        return Err(ERR_INVALID_ARGS);
    }

    let port = program_args[ARG_IDX_PORT_NUMBER]
        .as_str()
        .parse::<usize>()
        .map_err(|_| {
            eprintln!("Unable to parse port number");
            ERR_CANNOT_PARSE_ARGS
        })?;

    println!("{}", port);

    Ok(())
}
