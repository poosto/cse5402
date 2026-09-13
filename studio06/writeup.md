### Studio 06

1) *As the answer to the first exercise, list the names of the people who worked together on this studio.* \
Preston Meek

2) *Compile your program and as the answer to this exercise please show the error that occurred.*
```shell
error[E0004]: non-exhaustive patterns: `0_u32..=44_u32` and `46_u32..=u32::MAX` not covered
 --> src/main.rs:4:11
  |
4 |     match x {
  |           ^ patterns `0_u32..=44_u32` and `46_u32..=u32::MAX` not covered
  |
  = note: the matched value is of type `u32`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern, a match arm with multiple or-patterns as shown, or multiple match arms
  |
5 ~         45 => { println!("Matched it!") },
6 +         0_u32..=44_u32 | 46_u32..=u32::MAX => todo!()
  |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `rustenumspatterns` (bin "rustenumspatterns") due to 1 previous error
```
Very interesting that even though the variable is immutable, all possible `u32` values must be matched against.

3) *As the answer to this exercise please explain which arm was matched in each case.*
```shell
Matched it!
```
versus:
```shell
Catch-all case
```

When the specific case is first, that `println` gets called. However, if the wildcard line is first, that will run
before the specific case and print instead. There is even a compiler warning covering this:
```shell
warning: unreachable pattern
 --> src/main.rs:6:9
  |
5 |         _ => { println!("Catch-all case"); }
  |         - matches any value
6 |         45 => { println!("Matched it!"); }
  |         ^^ no value can reach this
  |
  = note: `#[warn(unreachable_patterns)]` on by default
```

4) *Compile and run your program and as the answer to this exercise please show the output the program produced.*
```rust
fn main() {
    let x = ("hello", 45u32);

    match x {
        ("hello", 45u32) => { println!("Matched tuple!"); }
        _ => { println!("Catch-all case"); }
    };
}
```
```shell
❯ cargo run
   Compiling rustenumspatterns v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio06/rustenumspatterns)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/rustenumspatterns`
Matched tuple!
```

5) *Compile and run your program and as the answer to this exercise please show the output the program produced and
explain briefly why a different output than in the previous exercise was produced.* 
```shell
Catch-all case
```
This occurs because the first `match` branch no longer matches the value of the tuple, so the wildcard case runs instead.

6) *Compile and run your program and as the answer to this exercise please show the output the program produced and
explain briefly why having a single branch didn't produce an error or warning like the case in the previous exercise
where having a single branch did.* 
```shell
Matched: hello 45
```
This did not produce an error or warning because by using destructured variable names in the `match` statement, you are
covering every possible case since any inputs will be bound to said variables.

7) *Compile and run your program with the variable initialized to "hello" and then again with the variable initialized
to "31" and as the answer to this exercise please show the output the program produced in each case.* \
When input is `"hello"`:
```shell
Could not convert string literal to u8
```
versus `"31"`:
```shell
Got value: 31
```

```rust
use std::str::FromStr;

let x = "31";

match u8::from_str(x) {
  Ok(number) => println!("Got value: {}", number),
  Err(_) => println!("Could not convert string literal to u8")
};
```

8) *As the answer to this exercise please show the code for that if let expression.*
```rust

```
