use std::fmt;

#[derive(Copy,Clone)]
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

trait Mover {
    fn advance(&mut self);
}

enum Rotation {
    Left,
    Right
}

trait Turner {
    fn turn(&mut self, r: Rotation);
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

impl Mover for Car {
    fn advance(&mut self) {
        match self.dir {
            Direction::North => self.y += 1,
            Direction::South => self.y -= 1,
            Direction::East => self.x += 1,
            Direction::West => self.x -= 1
        };
    }
}

impl Turner for Car {
    fn turn(&mut self, r: Rotation) {
        match (self.dir, r) {
            (Direction::North, Rotation::Left) => self.dir = Direction::West,
            (Direction::North, Rotation::Right) => self.dir = Direction::East,
            (Direction::South, Rotation::Left) => self.dir = Direction::West,
            (Direction::South, Rotation::Right) => self.dir = Direction::East,
            (Direction::East, Rotation::Left) => self.dir = Direction::North,
            (Direction::East, Rotation::Right) => self.dir = Direction::South,
            (Direction::West, Rotation::Left) => self.dir = Direction::South,
            (Direction::West, Rotation::Right) => self.dir = Direction::North,
        }
    }
}

fn main() {
    let mut car = Car::new();

    println!("Car values: {} {} {} {}", car.name, car.dir, car.x, car.y);

    car.advance();

    println!("Car values again: {} {} {} {}", car.name, car.dir, car.x, car.y);
}
