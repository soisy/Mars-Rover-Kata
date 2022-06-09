#![allow(unused)]

fn main() {
    println!("Hello, world!");
}

struct Planet {
    h: usize,
    w: usize,
}

#[derive(PartialEq, Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

enum Command {
    MoveForward,
    MoveBackward,
    TurnLeft,
    TurnRight,
}

#[derive(PartialEq, Debug)]
struct Rover {
    position: Position,
    direction: Direction
}

impl Rover {
    fn new(x: usize, y: usize, d: &str) -> Rover {
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

fn execute(command: Command, planet: Planet, rover: Rover) -> Rover {
    match command {
        Command::TurnLeft => turn_left(rover),
        Command::TurnRight => turn_right(rover),
        Command::MoveForward => move_forward(planet, rover),
        Command::MoveBackward => move_backward(planet, rover),
    }
}

fn move_forward(planet: Planet, rover: Rover) -> Rover {
    let position = match rover.direction {
        Direction::North => Position { x: rover.position.x, y: rover.position.y + 1 },
        Direction::East => Position { x: rover.position.x + 1, y: rover.position.y },
        Direction::South => Position { x: rover.position.x, y: rover.position.y - 1 },
        Direction::West => Position { x: rover.position.x - 1, y: rover.position.y },
    };
    Rover { position, ..rover}
}

fn move_backward(planet: Planet, rover: Rover) -> Rover {
    let position = match rover.direction {
        Direction::North => Position { x: rover.position.x, y: rover.position.y - 1 },
        Direction::East => Position { x: rover.position.x - 1, y: rover.position.y },
        Direction::South => Position { x: rover.position.x, y: rover.position.y + 1 },
        Direction::West => Position { x: rover.position.x + 1, y: rover.position.y },
    };
    Rover { position, ..rover}
}

fn turn_left(rover: Rover) -> Rover {
    let new_direction = match rover.direction {
        Direction::North => Direction::West,
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South
    };

    Rover {
        direction: new_direction,
        ..rover
    }
}

fn turn_right(rover: Rover) -> Rover {
    let new_direction = match rover.direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North
    };

    Rover {
        direction: new_direction,
        ..rover
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn rotate_right_command() {
        /*
           Planet: 5 4
           Rover: 0 0 N
           Command: R
           --
           Rover: 0 0 E
           */
        let planet = Planet { h: 5, w: 4 };
        let rover = Rover::new(0, 0, "N");
        let rover = execute(Command::TurnRight, planet, rover);

        assert_eq!(rover, Rover::new(0, 0, "E"));
    }


    #[test]
    fn rotate_left_command() {
        /*
           Planet: 5 4
           Rover: 0 0 N
           Command: L
           --
           Rover: 0 0 W
           */
        let planet = Planet { h: 5, w: 4 };
        let rover = Rover::new(0, 0, "N");
        let rover = execute(Command::TurnLeft, planet, rover);

        assert_eq!(rover, Rover::new(0, 0, "W"));
    }

    #[test]
    fn move_forward_command() {
        /*
           Planet: 5 4
           Rover: 0 0 N
           Command: M
           --
           Rover: 0 1 N
           */
        let planet = Planet { h: 5, w: 4 };
        let rover = Rover::new(0, 0, "N");
        let rover = execute(Command::MoveForward, planet, rover);

        assert_eq!(rover, Rover::new(0, 1, "N"));
    }

    #[test]
    fn move_backward_command() {
        let planet = Planet { h: 5, w: 4 };
        let rover = Rover::new(0, 1, "N");
        let rover = execute(Command::MoveBackward, planet, rover);

        assert_eq!(rover, Rover::new(0, 0, "N"));
    }
}
