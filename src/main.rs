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

fn execute(command: Command, planet: Planet, rover: Rover) -> Rover {
    match command {
        Command::TurnLeft => Rover { direction: Direction::West, ..rover },
        Command::TurnRight => Rover { direction: Direction::East, ..rover },
        _ => rover
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
        let planet = Planet {
            h: 5,
            w: 4,
        };
        let rover = Rover {
            position: Position {
                x: 0,
                y: 0,
            },
            direction: Direction::North,
        };
        let rover = execute(Command::TurnRight, planet, rover);

        assert_eq!(rover, Rover {
            position: Position {
                x: 0,
                y: 0,
            },
            direction: Direction::East,
        });
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
        let planet = Planet {
            h: 5,
            w: 4,
        };
        let rover = Rover {
            position: Position {
                x: 0,
                y: 0,
            },
            direction: Direction::North,
        };
        let rover = execute(Command::TurnLeft, planet, rover);

        assert_eq!(rover, Rover {
            position: Position {
                x: 0,
                y: 0,
            },
            direction: Direction::West,
        });
    }


}
