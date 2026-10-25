### Studio 14

1) *As the answer to the first exercise, list the names of the people who worked together on this studio.* \
Preston Meek

2) *Compile and run your program, and as the answer to this exercise please show the output that was produced.*
```shell
❯ cargo run
   Compiling rustio v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio14/rustio)
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
     Running `target/debug/rustio`
x = hello world, y = 45, z = true
```

3) *Compile and run your code and show the output that was produced.*
```shell
❯ cargo run
   Compiling rustio v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio14/rustio)
warning: unused `Result` that must be used
 --> src/main.rs:8:5
  |
8 |     writeln!(std::io::stdout().lock(), "x = {x}, y = {y}, z = {z}");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: this `Result` may be an `Err` variant, which should be handled
  = note: `#[warn(unused_must_use)]` on by default
  = note: this warning originates in the macro `writeln` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: `rustio` (bin "rustio") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/rustio`
x = hello world, y = 45, z = true
```

4) *Compile and run your code and show (1) the output that was produced and (2) the code that you wrote for this.*
```shell
❯ cargo run
   Compiling rustio v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio14/rustio)
    Finished dev [unoptimized + debuginfo] target(s) in 0.63s
     Running `target/debug/rustio`
x = hello world, y = 45, z = true
```
```rust
use std::{io::Write, process::ExitCode};

fn main() -> ExitCode {
    let x = "hello world";
    let y = 45;
    let z = true;

    let mut guard = std::io::stdout().lock();
    let result = writeln!(guard, "x = {x}, y = {y}, z = {z}");

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error writing to stdout: {e}");
            ExitCode::FAILURE
        }
    }
}
```

5) *As the answer to this exercise please show the output your program produced for each of those runs.*
```shell
CSE 5402/studio14/rustio on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run Cargo.toml
   Compiling rustio v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio14/rustio)
    Finished dev [unoptimized + debuginfo] target(s) in 0.58s
     Running `target/debug/rustio Cargo.toml`
File name: Cargo.toml
[package]
name = "rustio"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

CSE 5402/studio14/rustio on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0 took 2s
❯ cargo run ../Cargo.toml
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rustio ../Cargo.toml`
File name: ../Cargo.toml
Unable to open file "../Cargo.toml" (No such file or directory (os error 2))
```

6) *As the answer to this exercise please show (1) the output the program produced, and (2) the code you wrote for this
exercise and the previous one.* \
Previous code:
```rust
use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
    process::ExitCode,
};

fn main() -> ExitCode {
    let Some(file_name) = args().nth(1) else {
        eprintln!("usage: rustio <file_name>");
        return ExitCode::FAILURE;
    };
    println!("File name: {file_name}");

    let file = match File::open(&file_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Unable to open file \"{file_name}\" ({e})");
            return ExitCode::FAILURE;
        }
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => println!("{line}"),
            Err(e) => {
                eprintln!("Failed to read line: {e}");
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}
```
Current code:
```rust
use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
    process::ExitCode,
};

fn main() -> ExitCode {
    let Some(file_name) = args().nth(1) else {
        eprintln!("usage: rustio <file_name>");
        return ExitCode::FAILURE;
    };
    println!("File name: {file_name}");

    let file = match File::open(&file_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Unable to open file \"{file_name}\" ({e})");
            return ExitCode::FAILURE;
        }
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) if !line.trim().is_empty() => println!("{line}"),
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to read line: {e}");
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}
```
Output:
```shell
❯ cargo run Cargo.toml
   Compiling rustio v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio14/rustio)
    Finished dev [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/rustio Cargo.toml`
File name: Cargo.toml
[package]
name = "rustio"
version = "0.1.0"
edition = "2021"
# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[dependencies]
```
The blank lines in Cargo.toml are now skipped.
