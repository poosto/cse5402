### Studio 01
1) *As the answer to the first exercise, list the names of the people who worked together on this studio.* \
Preston Meek

2) *As the answer to this exercise, write down the hostname of the linuxlab machine you're using, which you can find out
at any time by issuing the command hostname if you didn't write it down immediately when you logged in.* \
`iht32-1502.engr.wustl.edu`

3) *As the answer to this exercise please show the output that each of them produced, and confirm that they each output
the correct version number.*
```shell
[meek@iht32-1502.sif ~]$ rustc --version
rustc 1.71.0 (8ede3aae2 2023-07-12)
[meek@iht32-1502.sif ~]$ cargo --version
cargo 1.71.0 (cfd3bbd8f 2023-06-08)
[meek@iht32-1502.sif ~]$ rustdoc --version
rustdoc 1.71.0 (8ede3aae2 2023-07-12)
```

4) *As the answer to this excercise, please declare which options you selected and if you selected any of the AI tools, please specify which ones you are using in APA-style citations.* \
I will (for now) be using my personal Claude Chat and Claude CLI. \
Note that Claude CLI was not used at all for this studio, so I included a dummy citation as per the instructions of this
question (since I will be using it in the future).

a) Anthropic. (2026, August 27). AI use policy citation formatting [Generative AI chat]. Claude Opus 5. https://claude.ai/share/0f880f94-fe02-4130-aeec-f496899e7d03 \
b) Anthropic. (2026). Claude Code (command-line interface) [Generative AI coding assistant]. Claude Opus 5. https://claude.com/claude-code

5) *Issue the commands cat `hello/Cargo.toml` and then `cat hello/src/main.rs` to print out the metadata and source code for the package respectively, and as the answer to this exercise please show the output those commands produced.*
```shell
[meek@iht32-1502.sif studio01]$ cargo new hello
     Created binary (application) `hello` package
[meek@iht32-1502.sif studio01]$ cat hello/Cargo.toml
[package]
name = "hello"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
[meek@iht32-1502.sif studio01]$ cat hello/src/main.rs
fn main() {
    println!("Hello, world!");
}
```

6) *As the answer to this exercise please show the output that was produced.*
```shell
[meek@iht32-1502.sif studio01]$ cd hello && cargo run
   Compiling hello v0.1.0 (/home/compute/meek/cse5402/studio01/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 2.99s
     Running `target/debug/hello`
Hello, world!
```

7) *As the answer to this exercise please show the code you wrote and the output the program produced when run with vs.
without command line arguments.*
```rust
use std::env;

fn main() {
    println!("Hello, world!");

    let mut arg_vec: std::vec::Vec<String> = Vec::new();

    for arg in env::args() {
        arg_vec.push(arg);
    }

    if arg_vec.len() == 1 {
        println!("usage: ./{} <arg1> [<arg2> ...]", arg_vec[0]);
    } else {
        println!(
            "arguments passed to ./{} were {:?}",
            arg_vec[0],
            &arg_vec[1..]
        );
    }
}
```

```shell
[meek@iht32-1501.sif cmdargs]$ cargo run
   Compiling cmdargs v0.1.0 (/home/compute/meek/cse5402/studio01/cmdargs)
    Finished dev [unoptimized + debuginfo] target(s) in 1.14s
     Running `target/debug/cmdargs`
Hello, world!
usage: ./target/debug/cmdargs <arg1> [<arg2> ...]
[meek@iht32-1501.sif cmdargs]$ cargo run hi there
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/cmdargs hi there`
Hello, world!
arguments passed to ./target/debug/cmdargs were ["hi", "there"]
```

8) *As the answer to this exercise please show all of those outputs.*
```shell
prestonmeek:cmdargs$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/cmdargs`
usage: ./target/debug/cmdargs <arg1> [<arg2> ...]
prestonmeek:cmdargs$ echo $?
1
prestonmeek:cmdargs$ cargo run hello world && echo $?
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/cmdargs hello world`
arguments passed to ./target/debug/cmdargs were ["hello", "world"]
0
```

9) *As the answer to this exercise please say briefly whether the outputs you saw from this exercise differed from the
ones from the previous exercise, and if so how they differed.*
```shell
prestonmeek:cmdargs$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/cmdargs`
usage: ./target/debug/cmdargs <arg1> [<arg2> ...]
Error: 1
prestonmeek:cmdargs$ echo $?
1
prestonmeek:cmdargs$ cargo run hello world
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/cmdargs hello world`
arguments passed to ./target/debug/cmdargs were ["hello", "world"]
prestonmeek:cmdargs$ echo $?
0
```
The outputs were identical, except the `Result<(), u8>` version included an `Error: 1` print statement when returning
from `main()`.

10) *As the answer to this exercise please (1) show code you wrote and the output that was produced by the program, and
(2) explain briefly how Rust handled the division by zero.*
```rust
fn main() {
    println!("Hello, world!");

    let x = 45;
    let y = 23;
    let z = x - y;

    #[allow(unconditional_panic)]
    let res = x / (x - y - z);

    println!("Result: {}", res);
}
```

```
prestonmeek:rusterrors$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/rusterrors`
Hello, world!

thread 'main' panicked at src/main.rs:9:15:
attempt to divide by zero
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Rust was able to detect the divide-by-zero error at compile time (since compile-time literals were used for the
arithmetic rather than runtime values). Thus, the program did not compile without the use of the 
`#[allow(unconditional_panic)]` attribute. After that was added, the program had a runtime panic when it reached the
divide-by-zero instruction.

11) *As the answer to this exercise please show the output produced from the run where the program did not divide by
zero, and from the run where it did.*
```shell
prestonmeek:rusterrors$ RUST_BACKTRACE=1 cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/rusterrors`
Hello, world!

thread 'main' panicked at src/main.rs:9:15:
attempt to divide by zero
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/17067e9ac6d7ecb70e50f92c1944e545188d2359/library/std/src/panicking.rs:697:5
   1: core::panicking::panic_fmt
             at /rustc/17067e9ac6d7ecb70e50f92c1944e545188d2359/library/core/src/panicking.rs:75:14
   2: core::panicking::panic_const::panic_const_div_by_zero
             at /rustc/17067e9ac6d7ecb70e50f92c1944e545188d2359/library/core/src/panicking.rs:178:21
   3: rusterrors::main
             at ./src/main.rs:9:15
   4: core::ops::function::FnOnce::call_once
             at /Users/prestonmeek/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

prestonmeek:rusterrors$ RUST_BACKTRACE=1 cargo run
   Compiling rusterrors v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio01/rusterrors)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/rusterrors`
Hello, world!
Result: 22
```

12) *As the answer to this exercise please show the code you wrote and the output of the program.*
```rust
pub const VAL: usize = 42;

fn main() {
    println!("Hello, world! {}", VAL);
}
```
```shell

prestonmeek:rustmodules$ cargo run
   Compiling rustmodules v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio01/rustmodules)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/rustmodules`
Hello, world! 42
```

13) *As the answer to this exercise please show the code you wrote and the output of the program.*
```rust
#[cfg(oldexercise)] pub const VAL: usize = 42;
pub static VAL: usize = 42;

fn main() {
    println!("Hello, world! {}", VAL);
}
```
```shell
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/rustmodules`
Hello, world! 42
```

14) *Compile and run your program and as the answer to this exercise please show the error that was produced.*
```rust
#[cfg(oldexercise)] pub const VAL: usize = 42;
#[cfg(oldexercise)] pub static VAL: usize = 42;
pub static mut VAL: usize = 42;

fn main() {
    println!("Hello, world! {}", VAL);
}
```
```shell
error[E0133]: use of mutable static is unsafe and requires unsafe block
 --> src/main.rs:6:34
  |
6 |     println!("Hello, world! {}", VAL);
  |                                  ^^^ use of mutable static
  |
  = note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined behavior

For more information about this error, try `rustc --explain E0133`.
warning: `rustmodules` (bin "rustmodules") generated 2 warnings
error: could not compile `rustmodules` (bin "rustmodules") due to 1 previous error; 2 warnings emitted
```

15) *Compile and run your program and as the answer to this exercise please show the code you wrote and the output of
the program.*
```rust
#[cfg(oldexercise)] pub const VAL: usize = 42;
#[cfg(oldexercise)] pub static VAL: usize = 42;
pub static mut VAL: usize = 42;

fn main() {
    unsafe {
        println!("Hello, world! {}", VAL);
    }
}
```
```shell
warning: creating a shared reference to mutable static is discouraged
 --> src/main.rs:7:34
  |
7 |     println!("Hello, world! {}", VAL);
  |                                  ^^^ shared reference to mutable static
  |
  = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2024/static-mut-references.html>
  = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference is created for it while the shared reference lives
  = note: `#[warn(static_mut_refs)]` on by default

warning: `rustmodules` (bin "rustmodules") generated 3 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rustmodules`
Hello, world! 42
```

16) *Compile and run your program and as the answer to this exercise please show the code you wrote and the output of
the program.*
```rust
#[cfg(oldexercise)] pub const VAL: usize = 42;
#[cfg(oldexercise)] pub static VAL: usize = 42;
#[cfg(oldexercise)] pub static mut VAL: usize = 42;

use std::sync::atomic::{AtomicUsize, Ordering};

static VAL: AtomicUsize = AtomicUsize::new(42);

fn main() {
    #[cfg(oldexercise)] unsafe {
        println!("Hello, world! {}", VAL);
    }

    println!("Value: {}", VAL.load(Ordering::SeqCst));
    VAL.store(33, Ordering::SeqCst);
    println!("New value: {}", VAL.load(Ordering::SeqCst));
}
```
```shell
Value: 42
New value: 33
```

17) *Compile your program, and as the answer to this exercise please show the error(s) that occurred.*
```shell
error[E0601]: `main` function not found in crate `rustmodules`
  --> src/main.rs:19:2
   |
19 | }
   |  ^ the main function must be defined at the crate level (in `src/main.rs`)
   |
note: here is a function named `main`
  --> src/main.rs:10:5
   |
10 |     fn main() {
   |     ^^^^^^^^^
   = note: you have one or more functions named `main` not defined at the crate level
   = help: consider moving the `main` function definitions

For more information about this error, try `rustc --explain E0601`.
warning: `rustmodules` (bin "rustmodules") generated 4 warnings
error: could not compile `rustmodules` (bin "rustmodules") due to 1 previous error; 4 warnings emitted
```

18) *As the answer to this exercise please show the error(s) that occurred.*
```shell
error[E0425]: cannot find value `VAL` in this scope
  --> src/main.rs:16:27
   |
16 |     println!("Value: {}", VAL.load(Ordering::SeqCst));
   |                           ^^^ not found in this scope
   |
note: static `crate::foo::VAL` exists but is inaccessible
  --> src/main.rs:8:5
   |
8  |     static VAL: AtomicUsize = AtomicUsize::new(42);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible

error[E0433]: failed to resolve: use of undeclared type `Ordering`
  --> src/main.rs:16:36
   |
16 |     println!("Value: {}", VAL.load(Ordering::SeqCst));
   |                                    ^^^^^^^^ use of undeclared type `Ordering`
   |
help: consider importing one of these enums
   |
1  + use std::cmp::Ordering;
   |
1  + use std::sync::atomic::Ordering;
   |

error[E0425]: cannot find value `VAL` in this scope
  --> src/main.rs:17:5
   |
17 |     VAL.store(33, Ordering::SeqCst);
   |     ^^^ not found in this scope
   |
note: static `crate::foo::VAL` exists but is inaccessible
  --> src/main.rs:8:5
   |
8  |     static VAL: AtomicUsize = AtomicUsize::new(42);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible

error[E0433]: failed to resolve: use of undeclared type `Ordering`
  --> src/main.rs:17:19
   |
17 |     VAL.store(33, Ordering::SeqCst);
   |                   ^^^^^^^^ use of undeclared type `Ordering`
   |
help: consider importing one of these enums
   |
1  + use std::cmp::Ordering;
   |
1  + use std::sync::atomic::Ordering;
   |
```

19) 
```rust
#[cfg(oldexercise)] pub const VAL: usize = 42;
#[cfg(oldexercise)] pub static VAL: usize = 42;
#[cfg(oldexercise)] pub static mut VAL: usize = 42;

mod foo {
    use std::sync::atomic::AtomicUsize;
    pub static VAL: AtomicUsize = AtomicUsize::new(42);
}

use foo::VAL;
use std::sync::atomic::Ordering;

fn main() {
    #[cfg(oldexercise)] unsafe {
        println!("Hello, world! {}", VAL);
    }

    println!("Value: {}", VAL.load(Ordering::SeqCst));
    VAL.store(33, Ordering::SeqCst);
    println!("New value: {}", VAL.load(Ordering::SeqCst));
}
```
```shell
Value: 42
New value: 33
```

22) *Try that within your `cmdargs` program, and as the answer to this exercise please show the error message that was
issued.*
```shell
error: negative integers cannot be used to index on a `Vec<String>`
  --> src/main.rs:12:13
   |
12 |     arg_vec[-1];
   |             ^^ cannot use a negative integer for indexing on `Vec<String>`
   |
help: to access an element starting from the end of the `Vec<String>`, compute the index
   |
12 |     arg_vec[arg_vec.len() -1];
   |             +++++++++++++

error: could not compile `cmdargs` (bin "cmdargs") due to 1 previous error
```

23) *As the answer to this exercise please show the error message that was issued.*
```shell
prestonmeek:cmdargs$ cargo run
   Compiling cmdargs v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio01/cmdargs)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/cmdargs`

thread 'main' panicked at src/main.rs:12:27:
index out of bounds: the len is 1 but the index is 1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

24) *As the answer to this exercise please show the output the program produced in each of the runs, and also a copy of
the code for one of them.*
```rust
mod my {
    #[derive(Debug)]
    pub enum Err {
        DivByZero = 1
    }
}

fn main() -> Result<(), my::Err> {
    println!("Hello, world!");

    let x = 45;
    let y = 23;
    let z = x - y + 2;

    let numerator = x;
    let denominator = x - y - z;

    match denominator {
        0 => Err(my::Err::DivByZero),
        _ => {
            println!("Result: {}", numerator / denominator);
            Ok(())
        }
    }
}
```
```shell
prestonmeek:rusterrors$ cargo run
   Compiling rusterrors v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio01/rusterrors)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/rusterrors`
Hello, world!
Error: 1
prestonmeek:rusterrors$ cargo run
   Compiling rusterrors v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio01/rusterrors)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/rusterrors`
Hello, world!
Result: -22
```

### Citations
Anthropic. (2026, August 27). Rust command-line argument parsing guidance [Generative AI chat]. Claude Code v2.1.247 (Claude Opus 5, medium reasoning effort). Appendix A.

I used Claude Code for some help with syntax and explaining concepts (such as using references for slices). I also used it as a starting point for questions without being given the answer directly. Besides that, I primarily used the BOT textbook for help with syntax.
