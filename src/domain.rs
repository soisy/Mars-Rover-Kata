pub struct Planet {
    pub h: usize,
    pub w: usize,
}

impl Planet {
    fn new_position(&self, position: &Position, direction: &Direction) -> Position {
        let (x, y) = match direction {
            Direction::North => (position.x, position.y + 1),
            Direction::East => (position.x + 1, position.y),
            Direction::South => (position.x, position.y.checked_sub(1).unwrap_or(self.h - 1)),
            Direction::West => (position.x.checked_sub(1).unwrap_or(self.w - 1), position.y),
        };

        let x = if (x > self.w - 1) { 0 } else { x };
        let y = if (y > self.h - 1) { 0 } else { y };

        Position { x, y }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(PartialEq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

pub enum Command {
    MoveForward,
    MoveBackward,
    TurnLeft,
    TurnRight,
}

#[derive(PartialEq, Debug)]
pub struct Rover {
    position: Position,
    direction: Direction
}

impl Rover {
    pub fn new(x: usize, y: usize, d: &str) -> Rover {
        let direction = match d {
            "N" => Direction::North,
            "E" => Direction::East,
            "S" => Direction::South,
            "W" => Direction::West,
            _ => panic!("Invalid direction")
        };
        Rover {
            position: Position { x, y },
            direction
        }
    }
}

pub fn execute(command: Command, planet: Planet, rover: Rover) -> Rover {
    match command {
        Command::TurnLeft => turn_left(rover),
        Command::TurnRight => turn_right(rover),
        Command::MoveForward => move_forward(planet, rover),
        Command::MoveBackward => move_backward(planet, rover),
    }
}

fn move_forward(planet: Planet, rover: Rover) -> Rover {
    let position = planet.new_position(&rover.position, &rover.direction);
    Rover { position, ..rover}
}

fn move_backward(planet: Planet, rover: Rover) -> Rover {
    let position = planet.new_position(&rover.position, &rover.direction.opposite());
    Rover { position, ..rover}
}

fn turn_left(rover: Rover) -> Rover {
    let direction = match rover.direction {
        Direction::North => Direction::West,
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South
    };

    Rover {
        direction,
        ..rover
    }
}

fn turn_right(rover: Rover) -> Rover {
    let direction = match rover.direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North
    };

    Rover {
        direction,
        ..rover
    }
}

