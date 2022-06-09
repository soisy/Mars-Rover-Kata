    pub struct Planet {
        pub h: usize,
        pub w: usize,
    }

    #[derive(PartialEq, Debug)]
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

