### Studio 02

1) *As the answer to the first exercise, list the names of the people who worked together on this studio.* \
Preston Meek

2) *Build and run the program to see what the largest value is that is printed out before overflow occurs and the program panics, and as the answer to this exercise please show the program's output.*
```shell
❯ cargo run
   Compiling rusttypes v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio02/rusttypes)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/rusttypes`
x: 1
x: 2
x: 4
x: 8
x: 16
x: 32
x: 64
x: 128
thread 'main' panicked at src/main.rs:6:9:
attempt to multiply with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

3) *Modify the copy that's outside the conditional compilation block so that it uses the `checked_mul` operation as the condition of a `while` expression to double the variable's value until overflow would occur the next time it was doubled. Build and run the program and as the answer to this exercise again show the program's output.*
```shell
x: 1
x: 2
x: 4
x: 8
x: 16
x: 32
x: 64
x: 128
```

4) *Inside the main function, declare an immutable variable of type `Point` whose fields are initialized with different floating point values, and print out its value using "{}" for each of its fields. Compile and run your program and as the answer to this exercise please show the code you wrote and the output of the program.*
```rust
struct Point(f64, f64);
fn main() {
    let p = Point(1.7, 0.8);
    println!("p is ({}, {})", p.0, p.1);
}
```
```shell
p is (1.7, 0.8)
```

5) *Modify the statement that prints out the value of the point variable so that it uses "{:?}" once (for the entire variable), instead of using "{}" twice (for each of the variable's fields). Compile and run your program and as the answer to this exercise please show the error that was produced.*
```shell
error[E0277]: `Point` doesn't implement `Debug`
 --> src/main.rs:6:27
  |
6 |     println!("p is {:?}", p);
  |                           ^ `Point` cannot be formatted using `{:?}`
  |
  = help: the trait `Debug` is not implemented for `Point`
  = note: add `#[derive(Debug)]` to `Point` or manually `impl Debug for Point`
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `Point` with `#[derive(Debug)]`
  |
1 + #[derive(Debug)]
2 | struct Point(f64, f64);
  |
For more information about this error, try `rustc --explain E0277`.
error: could not compile `ruststructs` (bin "ruststructs") due to 1 previous error
```

6) *Annotate the `Point` declaration with the attribute `#[derive(Debug)]`. Compile and run your program and as the answer to this exercise please show the output the program produced.*
```shell
p is Point(1.7, 0.8)
```

7) *Modify the declaration of the `Point` variable so that it is mutable, and then after the statement that prints it out add statements that modify the values of its fields and another statement that prints it out again. Compile and run your program and as the answer to this exercise please show the output the program produced.*
```shell
p is Point(1.7, 0.8)
p is now Point(4.5, 0.33)
```

8) *In between the struct declaration and the main function, add an impl block and in it declare an associated public `new` function that takes no parameters and returns an instance of the struct type that is initialized with value 0 in each of its fields. Modify your main function so that it uses that function to initialize its struct variable. Compile and run your program and as the answer to this exercise please show the output the program produced.*
```shell
p is Point(1.7, 0.8)
p is now Point(4.5, 0.33)
p2::new() is Point(0.0, 0.0)
```

9) *Modify your struct and impl block so that they take a generic type parameter `T`, and modify `new` so that it takes two arguments of the parameterized type. Modify your main function so that it uses the "turbofish" notation to specify that the struct variable is parameterized with type `f64`, and provides appropriate floating point values in its call to `new`. Compile and run your program and as the answer to this exercise please show the code you wrote and the output the program produced.*
```rust
#[derive(Debug)]
struct Point<T>(T, T);
impl<T> Point<T> {
    fn new(first: T, second: T) -> Self {
        Point(first, second)
    }
}
fn main() {
    let p = Point::<f64>::new(1.7, 0.8);
    println!("p is {:?}", p);
}
```
```shell
p is Point(1.7, 0.8)
```

10) *Declare another mutable variable in your main function, which is parameterized with type `usize` and is initialized by providing appropriate unsigned integer values to a call to the (generic) `new` function. Print out the value of that variable, change the values of its fields, and then print it out again. Compile and run your program and as the answer to this exercise please show the code you added for this exercise, and the output the program produced.*
```rust
#[derive(Debug)]
struct Point<T>(T, T);
impl<T> Point<T> {
    fn new(first: T, second: T) -> Self {
        Point(first, second)
    }
}
fn main() {
    let p = Point::<f64>::new(1.7, 0.8);
    println!("p is {:?}", p);
    let mut p2 = Point::<usize>::new(4, 6);
    println!("p2 is {:?}", p2);
    // p2 = Point(3, 2);
    p2.0 = 3;
    p2.1 = 2;
    println!("p2 is now {:?}", p2);
}
```
```shell
p is Point(1.7, 0.8)
p2 is Point(4, 6)
p2 is now Point(3, 2)
```

11) *In the generic implementation block for the struct, add an associated public `set` method that takes a mutable reference to the type-parameterized struct type as its first (self) argument, and instances of the parameterized type as its second and third arguments, and assigns them to the struct's fields. Modify your program's main function so that it uses the new `set` method to modify the structs' fields' values, instead of modifying the fields directly. As the answer to this exercise please show the code for the method you added, and explain briefly why the dead code error that you had seen earlier does not occur, i.e., is it necessary to access the fields directly, or is it sufficient to call a method that accesses them?*
```rust
#[derive(Debug)]
struct Point<T>(T, T);
impl<T> Point<T> {
    fn new(first: T, second: T) -> Self {
        Point(first, second)
    }
    fn set(&mut self, first: T, second: T) {
        self.0 = first;
        self.1 = second;
    }
}
fn main() {
    let mut p = Point::<f64>::new(1.3, 2.3);
    println!("p is {:?}", p);
    p.set(3.3, 4.6);
    println!("p is now {:?}", p);
}
```
```shell
p is Point(1.3, 2.3)
p is now Point(3.3, 4.6)
```
The warning is gone after using the `set` method. Therefore, this implies that it is sufficient to modify the fields of
a struct using a method to suppress usage warnings.

12) *As the answer to this exercise show the program's output.*
```shell
x = 1 -- # of doublings = 0
x = 2 -- # of doublings = 1
x = 4 -- # of doublings = 2
x = 8 -- # of doublings = 3
x = 16 -- # of doublings = 4
x = 32 -- # of doublings = 5
x = 64 -- # of doublings = 6
x = 128 -- # of doublings = 7
```

13) *As the answer to this exercise please show the output that was produced.*
```shell
x = 2
x = 4
x = 8
x = 16
x = 32
x = 64
x = 128
x = 255
```
