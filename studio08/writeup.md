### Studio 08

1) *As the answer to the first exercise, list the names of the people who worked together on this studio.* \
Preston Meek

2) *Compile and run your program, and as the answer to this exercise please show the output that was produced.*
```shell
Car { name: "Lightning", dir: North, x: 0, y: 0 }
```

3) *Compile and run your program, and as the answer to this exercise please show the output that was produced.*
```shell
Car { name: "Lightning", dir: North, pos: Position(0, 0) }
```

4) *As the answer to this exercise please show your code that implements the AddAssign trait for the Position struct.*
```rust
impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Position) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
```

5) *As the answer to this exercise please (1) show the program's output and (2) explain why you think the car's movement
is as it should be, with the new implementation based on overloading the += operator for the car's position.*
```shell
Car { name: "Lightning", dir: North, pos: Position(0, 0) }
Car { name: "Lightning", dir: North, pos: Position(-1, 1) }
Car { name: "Lightning", dir: North, pos: Position(-1, 2) }
Car { name: "Lightning", dir: West, pos: Position(-2, 2) }
Car { name: "Lightning", dir: South, pos: Position(-2, 1) }
Car { name: "Lightning", dir: West, pos: Position(-3, 1) }
Car { name: "Lightning", dir: West, pos: Position(-4, 1) }
Car { name: "Lightning", dir: North, pos: Position(-4, 2) }
Car { name: "Lightning", dir: East, pos: Position(-3, 2) }
Car { name: "Lightning", dir: South, pos: Position(-3, 1) }
Car { name: "Lightning", dir: South, pos: Position(-3, 1) }
```
The car's movement is as-expected. It correctly turns left, advances, turns right, advances, and prints its position
again at (-1, 1). Then, it completes the figure-eight motion. The functionality of the code is identical to what we had
before, but the syntax is much clearer now because of the operator overloading.

6) *Compile and run your program and as the answer to this exercise please show your code that implements the SubAssign
trait for the Position struct and the output of the program that indicates that the home method is working correctly.*
```rust
impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}
```
```
```shell
Car { name: "Lightning", dir: South, pos: Position(-3, 1) }
Car { name: "Lightning", dir: South, pos: Position(0, 0) }
```
The car successfully returns to Position(0, 0). I derived the `Clone` trait for `Position` so that `self.pos -=
self.pos.clone()` is valid.

7) *Compile and run your program, and as the answer to this exercise please show your code and the output the program produced.*
```shell
x = 0.8414709848078965
```

8) *Compile and run your program, and confirm that the same output as in the previous exercise was produced. As the
answer to this exercise please show your modified code.* 
```rust
fn main() {
    let mut x: f64 = 0.0;

    let mut c = || {
        x = x.cos().sin();
    };

    c();

    println!("x = {}", x);
}
```
```shell
x = 0.8414709848078965
```

9) *As the answer to this exercise please show your modified code.*
```rust
struct Foo(f64, fn(f64) -> f64, fn(f64) -> f64);

fn main() {
    let mut x: f64 = 0.0;

    let mut s = Foo(x, f64::cos, f64::sin);

    let mut c = || s.0 = s.2(s.1(s.0));
    c();

    println!("x = {}", s.0);
}
```

10) *Compile and run your program, and confirm that the same output as in the previous exercise was produced. As the
answer to this exercise please show your modified code.* 
```rust
struct Foo<T>(T, fn(T) -> T, fn(T) -> T);

fn main() {
    let x: f64 = 0.0;
    let mut s = Foo(x, f64::cos, f64::sin);

    let mut c = || s.0 = s.2(s.1(s.0));
    c();

    println!("x = {}", s.0);
}
```

11) *Compile and run your program, as the answer to this exercise please show the code that you added for this exercise
and the output produced by that code.*
```shell
x = 0.8414709848078965
Before: Hello
After: Hello, world!
```
