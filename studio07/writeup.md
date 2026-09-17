### Studio 07

1) *As the answer to the first exercise, list the names of the people who worked together on this studio.* \
Preston Meek

2) *Compile and run the code and as the answer to this exercise please show the code you wrote and the output the
program produced.*
```rust
#[derive(Debug)]
struct Car {
    name: String,
    x: isize,
    y: isize
}

impl Car {
    fn new() -> Self {
        Self {
            name: "Lightning".to_string(),
            x: 0,
            y: 0
        }
    }
}

fn main() {
    let car = Car::new();

    println!("{:?}", car);
}
```
```shell
Car { name: "Lightning", x: 0, y: 0 }
```

3) *Compile and run the code and as the answer to this exercise please show the code you wrote and the output the
program produced.*
```rust
use std::fmt;

enum Direction {
    North,
    South,
    East,
    West
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Direction::North => "North",
            Direction::South => "South",
            Direction::East => "East",
            Direction::West => "West"
        }) 
    }
}

struct Car {
    name: String,
    dir: Direction,
    x: isize,
    y: isize
}

impl Car {
    fn new() -> Self {
        Self {
            name: "Lightning".to_string(),
            dir: Direction::North,
            x: 0,
            y: 0
        }
    }
}

fn main() {
    let car = Car::new();

    println!("Car values: {} {} {} {}", car.name, car.dir, car.x, car.y);
}
```
```shell
Car values: Lightning North 0 0
```

4) *Compile and run your program and as the answer to this exercise please show the output the program produced.*
```shell
Car values: Lightning North 0 0
Car values again: Lightning North 0 1
```

5) *Compile and run your program and as the answer to this exercise please show the output the program produced.*
```shell
Car values: Lightning North 0 0
Car values again: Lightning North 0 1
Car values again: Lightning North 0 2
Car values again: Lightning West 0 2
Car values again: Lightning West -1 2
Car values again: Lightning North -1 2
Car values again: Lightning East -1 2
```

6) *Compile and run your program and as the answer to this exercise please show the code for the generic function and
the output the program produced.*
```rust
fn figure_eight<T: Mover + Turner + std::fmt::Debug>(c: &mut T) {
    // 1
    c.advance();
    println!("{:?}", c);

    // 2
    c.turn(Rotation::Left);
    c.advance();
    println!("{:?}", c);

    // 3
    c.turn(Rotation::Left);
    c.advance();
    println!("{:?}", c);

    // 4
    c.turn(Rotation::Left);
    c.advance();
    println!("{:?}", c);

    // 5
    c.advance();
    println!("{:?}", c);

    // 6
    c.turn(Rotation::Right);
    c.advance();
    println!("{:?}", c);

    // 7
    c.turn(Rotation::Right);
    c.advance();
    println!("{:?}", c);

    // 8
    c.turn(Rotation::Right);
    c.advance();
    println!("{:?}", c);
}
```
```shell
Car { name: "Lightning", dir: North, x: 0, y: 1 }
Car { name: "Lightning", dir: West, x: -1, y: 1 }
Car { name: "Lightning", dir: South, x: -1, y: 0 }
Car { name: "Lightning", dir: West, x: -2, y: 0 }
Car { name: "Lightning", dir: West, x: -3, y: 0 }
Car { name: "Lightning", dir: North, x: -3, y: 1 }
Car { name: "Lightning", dir: East, x: -2, y: 1 }
Car { name: "Lightning", dir: South, x: -2, y: 0 }
```

7) *Compile and run your program, and as the answer to this exercise please show the code you wrote and the output the
program produced.* 
```rust
#[derive(Debug)]
struct Foo {
    name: String,
}

impl Foo {
    fn new(s: &str) -> Self {
        Foo {
            name: s.to_string()
        }
    }
}

fn main() {
    let foo_one = Foo::new("hi");
    let foo_two = Foo::new("there");

    println!("{:?} {:?}", foo_one, foo_two);
}
```
```shell
Foo { name: "hi" } Foo { name: "there" }
```

8) *As the answer to this exercise please (1) show your code that implements the Drop trait, (2) show the output the
program produced, and (3) explain how the order in which the structs were dropped relates to the order in which they
were declared.* 
```rust
#[derive(Debug)]
struct Foo {
    name: String,
}

impl Foo {
    fn new(s: &str) -> Self {
        Foo {
            name: s.to_string()
        }
    }
}

// different from std::mem::drop,
// since that takes by ownership (intuitively)
impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping struct {}", self.name);
    }
}

fn main() {
    let foo_one = Foo::new("hi");
    let foo_two = Foo::new("there");

    println!("{:?} {:?}", foo_one, foo_two);
}
```
```shell
Foo { name: "hi" } Foo { name: "there" }
Dropping struct there
Dropping struct hi
```
The order in which the structs are dropped is the reverse of their order of declaration. This makes sense, as the second
instance might rely on the first instance for cleanup, so if the first instance is dropped before the second, there
could be UB. This is identical to the C++ approach.

9) *As the answer to this exercise please (1) show your code and (2) explain what changes (if any) you had to make in
addition to adding the generic parameters, in order to make this work.*
```rust
use std::fmt::Display;

#[derive(Debug)]
struct Foo<T: Display> {
    name: T,
}

impl<T: Display> Foo<T> {
    fn new(arg: impl Into<T>) -> Self {
        Foo {
            name: arg.into()
        }
    }
}

// different from std::mem::drop,
// since that takes by ownership (intuitively)
impl<T: Display> Drop for Foo<T> {
    fn drop(&mut self) {
        println!("Dropping struct {}", self.name);
    }
}

fn main() {
    let foo_one = Foo::<String>::new("hi");
    let foo_two = Foo::<usize>::new(45_usize);

    println!("{:?} {:?}", foo_one, foo_two);
}
```
```shell
Foo { name: "hi" } Foo { name: 45 }
Dropping struct 45
Dropping struct hi
```
To make the generic parameter work, I bounded `T` to the `Display` trait. Additionally, I wanted a `T` of `String` to
accept a `&str`, so I had the argument be bound by `Into<T>`. This means the argument to `new` doesn't have to strictly
be a `T` as long as it can be converted to a `T`.

10) *Compile and run your program, and as the answer to this exercise please show the output it produced.*
```rust
use std::fmt::Display;

#[derive(Debug)]
// usually, you shouldn't have trait bounds on the struct itself, only impl blocks
// but, any trait bounds on Drop must exist on the struct itself, so...
struct Foo<T: Display> {
    one: T,
    two: T
}

impl<T: Display> Foo<T> {
    fn new<U: Into<T>>(one: U, two: U) -> Self {
        Foo {
            one: one.into(), two: two.into()
        }
    }
}

// different from std::mem::drop,
// since that takes by ownership (intuitively)
impl<T: Display> Drop for Foo<T> {
    fn drop(&mut self) {
        println!("Dropping struct {} {}", self.one, self.two);
    }
}

fn main() {
    let foo_one = Foo::<String>::new("hi", "there");
    let foo_two = Foo::<usize>::new(45_usize, 33_usize);

    println!("{:?} {:?}", foo_one, foo_two);
}
```
```shell
Foo { one: "hi", two: "there" } Foo { one: 45, two: 33 }
Dropping struct 45 33
Dropping struct hi there
```

11) *Compile and run your program, and as the answer to this exercise please show the output the program produced.*
```shell
Foo { one: "hi", two: "there" } Foo { one: 45, two: 33 }
Foo { one: "hi", two: "" } Foo { one: 45, two: 0 }
```

12) *Remove the conditional compilation block from the code that implements the Drop trait. Compile and run your
program, and as the answer to this exercise please say whether an error occurred and if so why that happened.*
```rust
use std::fmt::Display;

#[derive(Debug, Default)]
// usually, you shouldn't have trait bounds on the struct itself, only impl blocks
// but, any trait bounds on Drop must exist on the struct itself, so...
struct Foo<T: Display> {
    one: T,
    two: T
}

impl<T: Display> Foo<T> {
    fn new<U: Into<T>>(one: U, two: U) -> Self {
        Foo {
            one: one.into(), two: two.into()
        }
    }
}

// different from std::mem::drop,
// since that takes by ownership (intuitively)
impl<T: Display> Drop for Foo<T> {
    fn drop(&mut self) {
        println!("Dropping struct {} {}", self.one, self.two);
    }
}

fn main() {
    let foo_one = Foo::<String>::new("hi", "there");
    let foo_two = Foo::<usize>::new(45_usize, 33_usize);

    let foo_one_prime = Foo::<String>::new("hi", Default::default());
    let foo_two_prime = Foo::<usize>::new(45_usize, Default::default());

    let foo_three = Foo::<String>{ one: "hi".to_string(), ..Foo::<String>::default() };

    println!("{:?} {:?}\n{:?} {:?}", foo_one, foo_two, foo_one_prime, foo_two_prime);
    println!("{:?}", foo_three);
}
```
```shell
error[E0509]: cannot move out of type `Foo<String>`, which implements the `Drop` trait
  --> src/main.rs:34:21
   |
34 | ... = Foo::<String>{ one: "hi".to_string(), ..Foo::<String>::default() };
   |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |       |
   |       cannot move out of here
   |       move occurs because value has type `String`, which does not implement the `Copy` trait

For more information about this error, try `rustc --explain E0509`.
error: could not compile `utilitytraits` (bin "utilitytraits") due to 1 previous error
```
This example uses struct update syntax, which basically uses some other struct to complete the remaining values of the
current struct. For `foo_three`, the first field is specified and the second field is obtained from a default instance of `Foo`. However, this creates a temporary `Foo` and partially moves it (only the `two` field). Thus, this will not compile since the `Drop` trait prevents partial moves of fields (similar issue to `Vec`). This would be fine for values that copy by default (i.e., implement `Copy`).
