    pub struct Planet {
        pub h: usize,
        pub w: usize,
    }

    impl Planet {
        fn next_position(&self, rover: &Rover) -> Position {
            match rover.direction {
                Direction::North => {
                    if self.h == rover.position.y + 1 {
                        Position { y: 0, ..rover.position }
                    } else {
                        Position { y: rover.position.y + 1, ..rover.position }
                    }
                },
                Direction::South => {
                    if rover.position.y == 0 {
                        Position { y: self.h - 1, ..rover.position }
                    } else {
                        Position { y: rover.position.y - 1, ..rover.position }
                    }
                }
                Direction::East => {
                    if self.w == rover.position.x + 1 {
                        Position {x: 0, ..rover.position}
                    } else {
                        Position {x: rover.position.x + 1, ..rover.position}
                    }
                }
                Direction::West => {
                    if rover.position.x == 0 {
                        Position {x: self.w - 1, ..rover.position}
                    } else {
                        Position {x: rover.position.x - 1, ..rover.position}
                    }
                }
            }

        }

        fn prev_position(&self, rover: &Rover) -> Position {
            let opposite_direction = match rover.direction {
                Direction::North => Direction::South,
                Direction::South => Direction::North,
                Direction::West => Direction::East,
                Direction::East => Direction::West,
            };

            let new_rover = Rover {position: rover.position, direction: opposite_direction};
            self.next_position(&new_rover)
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
        let position = planet.next_position(&rover);
        Rover { position, ..rover}
    }

    fn move_backward(planet: Planet, rover: Rover) -> Rover {
        let position = planet.prev_position(&rover);
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

