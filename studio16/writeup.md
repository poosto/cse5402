### Studio 16

1) *As the answer to the first exercise, list the names of the people who worked together on this studio.* \
Preston Meek

2) *Compile and run your program, and as the answer to this exercise please show the code for the main function and the
output that was produced.*
```rust
fn main() {
    let x = 4;
    let y = 33;

    println!("x = {x}, y = {y}");
}
```
```shell
❯ cargo run
   Compiling rustunsafe v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio16/rustunsafe)
    Finished dev [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/rustunsafe`
x = 4, y = 33
```

3) *Compile and run your program, and as the answer to this exercise please show the code for both functions and the
output that was produced.*
```rust
fn swap<T: Copy>(x: &mut T, y: &mut T) {
    let z = *x;
    *x = *y;
    *y = z;
}

fn main() {
    let mut x = 4;
    let mut y = 33;

    println!("x = {x}, y = {y}");

    swap(&mut x, &mut y);

    println!("x = {x}, y = {y}");
}
```
```shell
❯ cargo run
   Compiling rustunsafe v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio16/rustunsafe)
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/rustunsafe`
x = 4, y = 33
x = 33, y = 4
```

4) *Compile your program, and as the answer to this exercise please show the error(s) that occurred.*
```shell
❯ cargo check
    Checking rustunsafe v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio16/rustunsafe)
error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
 --> src/main.rs:2:13
  |
2 |     let z = *x;
  |             ^^ dereference of raw pointer
  |
  = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior

error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
 --> src/main.rs:3:10
  |
3 |     *x = *y;
  |          ^^ dereference of raw pointer
  |
  = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior

error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
 --> src/main.rs:3:5
  |
3 |     *x = *y;
  |     ^^^^^^^ dereference of raw pointer
  |
  = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior

error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
 --> src/main.rs:4:5
  |
4 |     *y = z;
  |     ^^^^^^ dereference of raw pointer
  |
  = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior

For more information about this error, try `rustc --explain E0133`.
error: could not compile `rustunsafe` (bin "rustunsafe") due to 4 previous errors
```

5) *Compile your program, and as the answer to this exercise please show the error(s) that occurred.*
```shell
❯ cargo check
    Checking rustunsafe v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio16/rustunsafe)
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
  --> src/main.rs:15:5
   |
15 |     swap(x_ptr, y_ptr);
   |     ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

For more information about this error, try `rustc --explain E0133`.
error: could not compile `rustunsafe` (bin "rustunsafe") due to previous error
```

6) *Compile and run your program, and as the answer to this exercise please show the code for both functions and the
output that was produced.*
```rust
unsafe fn swap<T: Copy>(x: *mut T, y: *mut T) {
    let z = *x;
    *x = *y;
    *y = z;
}

fn main() {
    let mut x = 4;
    let mut y = 33;

    println!("x = {x}, y = {y}");

    let x_ptr: *mut i32 = &mut x;
    let y_ptr: *mut i32 = &mut y;
    unsafe {
        swap(x_ptr, y_ptr);
    }

    println!("x = {x}, y = {y}");
}
```
```shell
❯ cargo run
   Compiling rustunsafe v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio16/rustunsafe)
    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/rustunsafe`
x = 4, y = 33
x = 33, y = 4
```

7) *As the answer to this exercise please describe briefly how your code (including in the main function as well as in
the swapping function) ensures that both of those contracts are met in all cases.* \
The `main` function guarantees:
* The pointers come from `&mut x` and `&mut y`, references to live local variables. A pointer made from a reference is never `nullptr` or dangling.
* `x` and `y` stay in scope until the end of `main`, so the pointers point to alive data for the whole call. These two
bullets satisfy Contract 1.
* The variables are `i32`, so the compiler will lay them out with proper `i32` alignment. Since the pointer is obtained
  directly from referencing the variable, the pointers are guaranteed to be aligned. Additionally, any four-byte
sequence is a valid `i32`. Therefore, Contract 2 is also satisfied.

The `swap` function guarantees:
* It only dereferences the two pointers it was given. It doesn't do pointer arithmetic, so it never reaches memory the caller didn't provide directly.
* It doesn't store the pointers anywhere, so there are no lifetime issues and dangers of dangling pointers.
* It only writes `i32` values that were read from valid `i32`s, so the referents stay well-formed.
