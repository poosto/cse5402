### Studio 17

1) *As the answer to the first exercise, list the names of the people who worked together on this studio.* \
Preston Meek

2) *As the answer to this exercise please show the code for the main function and the command line and the output that
was produced by each run.*
```rust
use std::env::args;

fn main() {
    let arg_count = args().count();
    println!("Program argument count: {arg_count}");
}
```
```shell
CSE 5402/studio17/rustforeign on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run
   Compiling rustforeign v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio17/rustforeign)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/rustforeign`
Program argument count: 1

CSE 5402/studio17/rustforeign on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run hello there world
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rustforeign hello there world`
Program argument count: 4
```

3) *As the answer to this exercise please show the code for the main function and the command line and the output that
was produced by each run.*
```rust
use std::env::args;

fn main() {
    let arg_count = args().count();
    println!("Program argument count: {arg_count}");

    for arg in args() {
        println!("{arg}");
    }
}
```
```shell
CSE 5402/studio17/rustforeign on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run
   Compiling rustforeign v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio17/rustforeign)
    Finished dev [unoptimized + debuginfo] target(s) in 0.44s
     Running `target/debug/rustforeign`
Program argument count: 1
target/debug/rustforeign

CSE 5402/studio17/rustforeign on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run hello there world
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rustforeign hello there world`
Program argument count: 4
target/debug/rustforeign
hello
there
world
```

4) *As the answer to this exercise please show the code for the main function.*
```rust
use std::{env::args, os::raw::c_int};

fn main() {
    let arg_count: c_int = args().count() as c_int;
    println!("Program argument count: {arg_count}");

    for arg in args() {
        println!("{arg}");
    }
}
```

5) *As the answer to this exercise please show the code for the updated main function.*
```rust
use std::ffi::CString;
use std::{env::args, os::raw::c_int};

extern crate libc;

extern "C" {
    pub fn printf(format: *const libc::c_char, ...) -> libc::c_int;
}

fn main() {
    let arg_count: c_int = args().count() as c_int;
    let format = CString::new("Program argument count: %i\n").unwrap();
    unsafe {
        printf(format.as_ptr(), arg_count);
    }

    for arg in args() {
        println!("{arg}");
    }
}
```
```shell
CSE 5402/studio17/rustforeign on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rustforeign`
Program argument count: 1
target/debug/rustforeign

CSE 5402/studio17/rustforeign on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run hello there world
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rustforeign hello there world`
Program argument count: 4
target/debug/rustforeign
hello
there
world
```

6) *Try to compile your program and as the answer to this exercise please (1) show the error that occurred and (2)
explain briefly why it occurred.*
```shell
❯ cargo check
    Checking libc v0.1.12
    Checking rustforeign v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio17/rustforeign)
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
  --> src/main.rs:13:5
   |
13 |     printf(format.as_ptr(), arg_count);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

For more information about this error, try `rustc --explain E0133`.
error: could not compile `rustforeign` (bin "rustforeign") due to previous error
```
Foreign function calls are always considered unsafe, since the implementation of such code does not abide by Rust's
compiler rules. Even if the function is 'safe' and doesn't even use any pointers, it still must be wrapped in an unsafe
block since any foreign code is never fully trusted.

7) *As the answer to this exercise please show the code for the updated main function.*
```rust
use std::ffi::CString;
use std::{env::args, os::raw::c_int};

extern crate libc;

extern "C" {
    pub fn printf(format: *const libc::c_char, ...) -> libc::c_int;
}

fn main() {
    let arg_count: c_int = args().count() as c_int;
    let format = CString::new("Program argument count: %i\n").unwrap();
    unsafe {
        printf(format.as_ptr(), arg_count);

        let per_arg_format = CString::new("%s\n").unwrap();
        for arg in args() {
            let arg_cstr = CString::new(arg).unwrap();
            printf(per_arg_format.as_ptr(), arg_cstr.as_ptr());
        }
    }
}
```

8) *As the answer to this exercise please show the code for the updated main function.*
```rust
use std::ffi::CString;
use std::{env::args, os::raw::c_int};

extern crate libc;

extern "C" {
    pub fn printf(format: *const libc::c_char, ...) -> libc::c_int;
}

fn main() {
    let arg_count: c_int = args().count() as c_int;
    let format = CString::new("Program argument count: %i\n").unwrap();

    unsafe {
        printf(format.as_ptr(), arg_count);
    }

    let per_arg_format = CString::new("%s\n").unwrap();

    for arg in args() {
        let arg_cstr = CString::new(arg).unwrap();

        unsafe {
            printf(per_arg_format.as_ptr(), arg_cstr.as_ptr());
        }
    }
}
```
