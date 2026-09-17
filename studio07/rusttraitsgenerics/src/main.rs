use std::fmt;

#[derive(Copy,Clone,Debug)]
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

#[derive(Debug)]
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

fn main() {
    let mut car = Car::new();
    
    #[cfg(oldexercise)]
    {
        println!("Car values: {} {} {} {}", car.name, car.dir, car.x, car.y);

        car.advance();
        println!("Car values again: {} {} {} {}", car.name, car.dir, car.x, car.y);

        car.advance();
        println!("Car values again: {} {} {} {}", car.name, car.dir, car.x, car.y);

        car.turn(Rotation::Left);
        println!("Car values again: {} {} {} {}", car.name, car.dir, car.x, car.y);

        car.advance();
        println!("Car values again: {} {} {} {}", car.name, car.dir, car.x, car.y);

        car.turn(Rotation::Right);
        println!("Car values again: {} {} {} {}", car.name, car.dir, car.x, car.y);

        car.turn(Rotation::Right);
        println!("Car values again: {} {} {} {}", car.name, car.dir, car.x, car.y);
    }

    figure_eight(&mut car);
}
