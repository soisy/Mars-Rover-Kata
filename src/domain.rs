use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct Planet {
    pub h: usize,
    pub w: usize,
    pub obstacles: Vec<Position>,
}

impl Planet {
    pub fn without_obstacles(w: usize, h: usize) -> Planet {
        Planet { w, h, obstacles: vec![] }
    }

    pub fn new (w: usize, h: usize, obstacles: Vec<Position>) -> Planet {
        Planet { w, h, obstacles }
    }

    fn new_position(&self, position: &Position, direction: &Direction) -> Result<Position, MissionError> {
        let (x, y) = match direction {
            Direction::North => (position.x, position.y + 1),
            Direction::East => (position.x + 1, position.y),
            Direction::South => (position.x, position.y.checked_sub(1).unwrap_or(self.h - 1)),
            Direction::West => (position.x.checked_sub(1).unwrap_or(self.w - 1), position.y),
        };

        let x = if (x > self.w - 1) { 0 } else { x };
        let y = if (y > self.h - 1) { 0 } else { y };

        let new_position = Position { x, y };

        if self.obstacles.iter().any(|&p| p == new_position) {
            Err(MissionError::HitObstacle(*position))
        } else {
            Ok(new_position)
        }
    }

    fn is_obstacle(&self, position: &Position) -> bool {
        self.obstacles.iter().any(|p| p == position)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum MissionError {
    #[error("rover hit obstacle")]
    HitObstacle(Position),
    #[error("invalid coordinates `{0}`")]
    InvalidCoordinates(String),
    #[error("invalid dimensions `{0}`")]
    InvalidDimensions(String),
    #[error("invalid direction `{0}`")]
    InvalidDirection(String),
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

#[derive(PartialEq, Debug)]
pub enum Direction {
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

// dovrebbe resituire un Option<Command>
impl Command {
    pub fn new(c: char) -> Command {
        match c {
            'F' => Command::MoveForward,
            'B' => Command::MoveBackward,
            'L' => Command::TurnLeft,
            'R' => Command::TurnRight,
            _ => panic!("Invalid command"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Rover {
    pub position: Position,
    pub direction: Direction
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


pub fn execute(command: Command, planet: &Planet, rover: Rover) -> Result<Rover, MissionError> {
    match command {
        Command::TurnLeft => Ok(turn_left(rover)),
        Command::TurnRight => Ok(turn_right(rover)),
        Command::MoveForward => move_forward(planet, rover),
        Command::MoveBackward => move_backward(planet, rover),
    }
}

fn move_forward(planet: &Planet, rover: Rover) -> Result<Rover, MissionError> {
    planet.new_position(&rover.position, &rover.direction)
        .map(|p| Rover { position: p, direction: rover.direction })
}

fn move_backward(planet: &Planet, rover: Rover) -> Result<Rover, MissionError> {
    planet.new_position(&rover.position, &rover.direction.opposite())
        .map(|p| Rover { position: p, direction: rover.direction })
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

