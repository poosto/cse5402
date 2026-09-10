### Studio 05

1) *As the answer to the first exercise, list the names of the people who worked together on this studio.* \
Preston Meek

2) *Compile and run your program and as the answer to this exercise please (1) show code you wrote and the output that
was produced by the program, and (2) explain briefly which of the arithmetic operators has higher precedence and why
that explanation is consistent with the value that was produced.*
```rust
fn main() {
    println!("Compute: {}", 3 + 4 * 8);
}
```
```shell
❯ cargo run
   Compiling rustexpressions v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio05/rustexpressions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/rustexpressions`
Compute: 35
```
If addition had higher precedence, the result would be (3+4) * 8 = 56. \
Since multiplication precedes addition, the result was 3 + (4*8) = 35.

3) *Compile and run your program and as the answer to this exercise please show the code you wrote for it and the output
of the program.* 
```rust
let x = 35;
let y = 63;

let result = if x > y {
x
} else {
y
};

println!("Result: {}", result);
```
```shell
Result: 63
```

4) *Compile and run your program and as the answer to this exercise please describe the error that occurred and explain
briefly why it is happening.*
```shell
error[E0308]: `if` and `else` have incompatible types
  --> src/main.rs:11:9
   |
8  |       let result = if x > y {
   |  __________________-
9  | |         x;
   | |         --
   | |         ||
   | |         |help: consider removing this semicolon
   | |         expected because of this
10 | |     } else {
11 | |         y
   | |         ^ expected `()`, found integer
12 | |     };
   | |_____- `if` and `else` have incompatible types

For more information about this error, try `rustc --explain E0308`.
warning: `rustexpressions` (bin "rustexpressions") generated 1 warning
error: could not compile `rustexpressions` (bin "rustexpressions") due to 1 previous error; 1 warning emitted
```
This error is happening because the semicolon after `x` prevents it from being a return type. Changing that statement to
`return x;` would fix it, or simply removing the semicolon.

5) *Compile and run your program and as the answer to this exercise please show the code you wrote for it and the output
of the program when it ran.*
```rust
let tup = (35, 63);

// let result = match tup {
//     (x, y) => {
//         if x > y {
//             x
//         } else {
//             y
//         }
//     }
// };

// Simplified match destructuring
// (match is just a let in disguise, but let requires an irrefutable pattern)
let (x, y) = tup;
let result = if x > y {
x
} else {
y
};

println!("Result: {}", result);
```
```shell
Result: 63
```

6) *Compile and run your program and as the answer to this exercise please show the code you wrote for it and the output
of the program when it ran.* 
```rust
let mut vec = Vec::<usize>::new();

for i in 0..10 {
  vec.push(i);
}

println!("Vec: {:?}", vec); 
```
```shell
Vec: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
```

7) *Compile and run your program and as the answer to this exercise please describe the error that occurred and explain
briefly why it is happening.* 
```shell
error[E0382]: borrow of moved value: `vec`
   --> src/main.rs:57:33
    |
45  |     let mut vec = Vec::<usize>::new();
    |         ------- move occurs because `vec` has type `Vec<usize>`, which does not implement the `Copy` trait
...
53  |     for j in vec {
    |              --- `vec` moved due to this implicit call to `.into_iter()`
...
57  |     println!("Vec again: {:?}", vec);
    |                                 ^^^ value borrowed here after move
    |
note: `into_iter` takes ownership of the receiver `self`, which moves `vec`
   --> /Users/prestonmeek/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/iter/traits/collect.rs:313:18
    |
313 |     fn into_iter(self) -> Self::IntoIter;
    |                  ^^^^
    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider iterating over a slice of the `Vec<usize>`'s content to avoid moving into the `for` loop
    |
53  |     for j in &vec {
    |              +
```
This error is happening because the for loop consumes each value of the vector by casting it to a consuming iterator.
Thus, `vec` no longer owns the underlying data and cannot be referenced.

8) *Compile and run your program and as the answer to this exercise please show the code you wrote for it and the output
of the program when it ran.*
```rust
let mut vec = Vec::<usize>::new();

for i in 0..10 {
  vec.push(i);
}

println!("Vec: {:?}", vec); 

for j in &vec {
  println!("Value: {}", j);
}

println!("Vec again: {:?}", vec);
```
```shell
Vec: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
Value: 0
Value: 1
Value: 2
Value: 3
Value: 4
Value: 5
Value: 6
Value: 7
Value: 8
Value: 9
Vec again: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
```
