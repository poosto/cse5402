### Studio 15

1) *As the answer to the first exercise, list the names of the people who worked together on this studio.* \
Preston Meek

2) 
```rust
use std::env::args;

fn main() {
    let mut program_args = args();
    let program_name = program_args.next().unwrap();

    if program_args.count() != 0 {
        eprintln!("Usage: {program_name} (no args)");
    }
}
```
```shell
CSE 5402/studio15/rustmacros on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run
   Compiling rustmacros v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio15/rustmacros)
    Finished dev [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/rustmacros`

CSE 5402/studio15/rustmacros on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run hello world
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rustmacros hello world`
Usage: target/debug/rustmacros (no args)
```

3) *As the answer to this exercise please show the code for the main function and the macro.*
```rust
use std::env::args;

macro_rules! check_cmd_line {
    () => {
        let mut program_args = args();
        let program_name = program_args.next().unwrap();

        if program_args.count() != 0 {
            eprintln!("Usage: {program_name} (no args)");
        }
    };
}

fn main() {
    check_cmd_line!();
}
```

4) *As the answer to this exercise please show the code for the declaration and initialization of the public static
atomic variable, and the macro.*
```rust
use std::{
    env::args,
    sync::atomic::{AtomicBool, Ordering},
};

static FLAG: AtomicBool = AtomicBool::new(true);

macro_rules! check_cmd_line {
    () => {
        let mut program_args = args();
        let program_name = program_args.next().unwrap();

        if program_args.count() != 0 && FLAG.load(Ordering::Relaxed) {
            eprintln!("Usage: {program_name} (no args)");
        }
    };
}

fn main() {
    check_cmd_line!();
}
```
```shell
CSE 5402/studio15/rustmacros on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run
   Compiling rustmacros v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio15/rustmacros)
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/rustmacros`

CSE 5402/studio15/rustmacros on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run bob
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rustmacros bob`
Usage: target/debug/rustmacros (no args)
```

5) *As the answer to this exercise, please show the output that was produced.*
```shell
❯ cargo run
   Compiling rustmacros v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio15/rustmacros)
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/rustmacros`

CSE 5402/studio15/rustmacros on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run bob
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rustmacros bob`
Usage: target/debug/rustmacros (no args)
Usage: target/debug/rustmacros (no args)
```

6) *As the answer to this exercise, please show the main function and the definition of the check_cmd_line macro.*
```rust
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
```
```shell
CSE 5402/studio15/rustmacros on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run
   Compiling rustmacros v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio15/rustmacros)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/rustmacros`

CSE 5402/studio15/rustmacros on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run bob
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rustmacros bob`
Usage: target/debug/rustmacros (no args)
```
