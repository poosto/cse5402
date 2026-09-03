### Studio 03

1) *As the answer to the first exercise, list the names of the people who worked together on this studio.* \
Preston Meek

2) *Compile and run your program and as the answer to this exercise please show the output that was produced.*
```rust
❯ cargo run
   Compiling rustownership v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio03/rustownership)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/rustownership`
Values: 4 6 6
```

3) *Compile and run your program and as the answer to this exercise please describe the error that occurred and explain
briefly why it is happening.*
```shell
error[E0382]: borrow of moved value: `b`
  --> src/main.rs:15:37
   |
12 |     let b: String = "there".to_string();
   |         - move occurs because `b` has type `String`, which does not implement the `Copy` trait
13 |     let c: String = b;
   |                     - value moved here
14 |
15 |     println!("Values: {} {} {}", a, b, c);
   |                                     ^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
   |
13 |     let c: String = b.clone();
   |                      ++++++++
```
`String` is not a trivial type, therefore it is not implicitly copied unlike `u8` (which has the `Copy` trait). Therefore, a move occurs and there is an ownership error.

4) *Compile and run your program and as the answer to this exercise please show the output that was produced.*
```shell
Values: hello world there
```

5) *Compile and run your program and as the answer to this exercise please describe the error that occurred and explain
briefly why it is happening.*
```shell
error[E0382]: borrow of moved value: `c`
  --> src/main.rs:19:40
   |
13 |     let c: String = b;
   |         - move occurs because `c` has type `String`, which does not implement the `Copy` trait
...
18 |     b = c;
   |         - value moved here
19 |     println!("Values: {} {} {}", a, b, c);
   |                                        ^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
   |
18 |     b = c.clone();
   |          ++++++++
```
This is a very similar error to what occurred in (3). The `c` variable was moved into `b`, and therefore `c` cannot be
borrowed anymore by `println!`.

6) *Compile and run your program and as the answer to this exercise please show the output that was produced.*
```shell
Values: hello world there
Values: hello there buddy
```

7) *Compile and run your program and as the answer to this exercise please show the output that was produced.*
```shell
Values: hello world there
Values: hello there buddy
Vec values: ["hello", "there", "buddy"]
```

8) *Compile and run your program and as the answer to this exercise please describe the error that occurred and explain
briefly why it is happening.*
```rust
error[E0507]: cannot move out of index of `Vec<String>`
  --> src/main.rs:28:21
   |
28 |     let d: String = vec[2];
   |                     ^^^^^^ move occurs because value has type `String`, which does not implement the `Copy` trait
   |
help: consider borrowing here
   |
28 |     let d: String = &vec[2];
   |                     +
help: consider cloning the value if the performance cost is acceptable
   |
28 |     let d: String = vec[2].clone();
   |                           ++++++++
```
This error occurs because `d` initiates a move out of the `Vec<String>` by index. This is not allowed by the compiler and leaves `d` in an error state.

9) *Compile and run your program and as the answer to this exercise please show the output that was produced.*
```shell
Values: hello world there
Values: hello there buddy
Vec values: ["hello", "there", "buddy"]
Vec + str: ["hello", "there"] buddy
```
