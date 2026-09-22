use std::fmt;
use std::ops::AddAssign;
use std::ops::SubAssign;

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Direction::North => "North",
            Direction::South => "South",
            Direction::East => "East",
            Direction::West => "West",
        })
    }
}

trait Mover {
    fn advance(&mut self);
}

enum Rotation {
    Left,
    Right,
}

trait Turner {
    fn turn(&mut self, r: Rotation);
}

#[derive(Debug)]
struct Car {
    name: String,
    dir: Direction,
    pos: Position,
}

impl Car {
    fn new() -> Self {
        Self {
            name: "Lightning".to_string(),
            dir: Direction::North,
            pos: Position::new(0, 0),
        }
    }

    fn home(&mut self) {
        self.pos -= self.pos.clone();
    }
}

impl Mover for Car {
    fn advance(&mut self) {
        match self.dir {
            Direction::North => self.pos += Position::new(0, 1),
            Direction::South => self.pos += Position::new(0, -1),
            Direction::East => self.pos += Position::new(1, 0),
            Direction::West => self.pos += Position::new(-1, 0),
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

#[derive(Debug, Clone)]
struct Position(isize, isize);

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self(x, y)
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Position) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

fn main() {
    let mut car = Car::new();
    println!("{:?}", car);

    car.turn(Rotation::Left);
    car.advance();
    car.turn(Rotation::Right);
    car.advance();
    println!("{:?}", car);

    figure_eight(&mut car);
    println!("{:?}", car);

    car.home();
    println!("{:?}", car);
}
