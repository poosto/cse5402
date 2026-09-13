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

5) 
