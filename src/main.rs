#![allow(unused)]

mod domain;

use core::fmt;
use std::{error::Error, num::ParseIntError};
use thiserror::Error;
use multi_try::MultiTry;
use domain::*;

fn main() {
    println!("Hello, world!");
}

pub fn parse_commands(commands: &str) -> Result<Vec<char>, MissionError> {
    let allowed_commands = vec!['L', 'R', 'F', 'B'];
    commands.chars()
        .map(|c| allowed_commands.contains(&c).then_some(c))
        .collect::<Option<Vec<char>>>()
        .ok_or(MissionError::InvalidCommand(commands.to_string()))
}

fn parse_planet(dimensions: &str, obstacles: &str) -> Result<Planet, String> {
    let (w, h) = parse_dimensions(dimensions).map_err(|e| e.to_string())?;
    let obstacles = parse_obstacles(obstacles).map_err(|e| e.to_string())?;
    Ok(Planet::new(w, h, obstacles))
}

fn parse_rover(position: &str, direction: &str) -> Result<Rover, Vec<MissionError>> {
    parse_position(position)
        .and_try(parse_direction(direction))
        .map(|(position, direction)| Rover { position, direction })
}

fn parse_direction(direction: &str) -> Result<Direction, MissionError> {
    match direction {
        "N" => Ok(Direction::North),
        "E" => Ok(Direction::East),
        "S" => Ok(Direction::South),
        "W" => Ok(Direction::West),
        _ => Err(MissionError::InvalidDirection(direction.to_string())),
    }
}

fn parse_dimensions(dimensions: &str) -> Result<(usize, usize), MissionError> {
    dimensions
        .split("x")
        .map(|x| x.parse::<usize>().map_err(|_| MissionError::InvalidDimensions(dimensions.to_string())))
        .collect::<Result<Vec<usize>, MissionError>>()
        .and_then(|d| match d.len() {
            2 => Ok((d[0], d[1])),
            _ => Err(MissionError::InvalidDimensions(dimensions.to_string())),
        })
}

fn parse_obstacles(obstacles: &str) -> Result<Vec<Position>, MissionError> {
    obstacles
        .split(" ")
        .filter(|x| x.len() > 0)
        .map(parse_position)
        .collect::<Result<Vec<Position>, MissionError>>()
        .and_then(|v| match v.len() {
            0 => Ok(vec![]),
            _ => Ok(v),
        })
        .map_err(|_| MissionError::InvalidCoordinates(obstacles.to_string()))
}

fn parse_position(position: &str) -> Result<Position, MissionError> {
    position
        .split(",")
        .map(|x| x.parse::<usize>().map_err(|_| MissionError::InvalidCoordinates(position.to_string())))
        .collect::<Result<Vec<usize>, _>>()
        .and_then(|x| match x.len() {
            2 => Ok(Position::new(x[0], x[1])),
            _ => Err(MissionError::InvalidCoordinates(position.to_string())),
        })
}

fn execute_commands(commands: &str, planet: Planet, rover: Rover) -> Result<Rover, MissionError> {
    parse_commands(commands)
        .and_then(|commands| commands.iter()
            .try_fold(rover, |rover, &command| {
                execute(Command::new(command), &planet, rover)
            })
        )
}

#[cfg(test)]
mod tests {
    use std::{mem::discriminant, num::ParseIntError};
    use itertools::Itertools;

    use super::*;
    use domain::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn rotate_right_command() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(0, 0, "N");
        let rover = execute(Command::TurnRight, &planet, rover);

        assert_eq!(rover, Ok(Rover::new(0, 0, "E")));
    }


    #[test]
    fn rotate_left_command() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(0, 0, "N");
        let rover = execute(Command::TurnLeft, &planet, rover);

        assert_eq!(rover, Ok(Rover::new(0, 0, "W")));
    }

    #[test]
    fn move_forward_command() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(0, 0, "N");
        let rover = execute(Command::MoveForward, &planet, rover);

        assert_eq!(rover, Ok(Rover::new(0, 1, "N")));
    }

    #[test]
    fn move_backward_command() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(0, 1, "N");
        let rover = execute(Command::MoveBackward, &planet, rover);

        assert_eq!(rover, Ok(Rover::new(0, 0, "N")));
    }

    #[test]
    fn pacman_effect_north_forward() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(0, 4, "N");
        let rover = execute(Command::MoveForward, &planet, rover);

        assert_eq!(rover, Ok(Rover::new(0, 0, "N")));
    }

    #[test]
    fn pacman_effect_sud_forward() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(0, 0, "S");
        let rover = execute(Command::MoveForward, &planet, rover);

        assert_eq!(rover, Ok(Rover::new(0, 4, "S")));
    }

    #[test]
    fn pacman_effect_east_forward() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(3, 0, "E");
        let rover = execute(Command::MoveForward, &planet, rover);

        assert_eq!(rover, Ok(Rover::new(0, 0, "E")));
    }

    #[test]
    fn pacman_effect_west_forward() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(0, 0, "W");
        let rover = execute(Command::MoveForward, &planet, rover);

        assert_eq!(rover, Ok(Rover::new(3, 0, "W")));
    }

    #[test]
    fn pacman_effect_north_backward() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(0, 4, "S");
        let rover = execute(Command::MoveBackward, &planet, rover);

        assert_eq!(rover, Ok(Rover::new(0, 0, "S")));
    }

    #[test]
    fn pacman_effect_south_backward() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(0, 0, "N");
        let rover = execute(Command::MoveBackward, &planet, rover);

        assert_eq!(rover, Ok(Rover::new(0, 4, "N")));
    }

    #[test]
    fn pacman_effect_east_backward() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(3, 0, "W");
        let rover = execute(Command::MoveBackward, &planet, rover);

        assert_eq!(rover, Ok(Rover::new(0, 0, "W")));
    }

    #[test]
    fn pacman_effect_west_backward() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(0, 0, "E");
        let rover = execute(Command::MoveBackward, &planet, rover);

        assert_eq!(rover, Ok(Rover::new(3, 0, "E")));
    }

    // +-----+-----+-----+-----+-----+
    // | 0,3 |     |     |     | 4,3 |
    // +-----+-----+-----+-----+-----+
    // |     |     |     |     |     |
    // +-----+-----+-----+-----+-----+
    // |     |     |     |     |     |
    // +-----+-----+-----+-----+-----+
    // | 0,0 |     |     |     | 4,0 |
    // +-----+-----+-----+-----+-----+

    #[test]
    fn go_to_opposite_angle() {
        let planet = Planet::without_obstacles(5, 4);
        let rover = Rover::new(0, 0, "N");
        let rover = execute_commands("LFRB", planet, rover);

        assert_eq!(rover, Ok(Rover::new(4, 3, "N")));
    }

    #[test]
    fn empty_command_string() {
        let planet = Planet::without_obstacles(5, 4);
        let rover = Rover::new(0, 0, "N");
        let rover = execute_commands("", planet, rover);

        assert_eq!(rover, Ok(Rover::new(0, 0, "N")));
    }

    #[test]
    fn invalid_command_string() {
        let planet = Planet::without_obstacles(5, 4);
        let rover = Rover::new(0, 0, "N");
        let rover = execute_commands("RBXRF", planet, rover);

        assert_eq!(rover, Err(MissionError::InvalidCommand("RBXRF".to_string())));
    }

    #[test]
    fn hit_obstacle_during_command_executions() {
        // val planet   = ("5x4", "2,0 0,3 3,2")
        // val rover    = ("0,0", "N")
        // val commands = "RFF"

        // TODO: complete the test
        // invoke a function with: planet, obstacles, rover and commands

        // assert result, OK "O:1:0:E"

        let obstacles = vec![
            Position::new(2, 0),
            Position::new(0, 3),
            Position::new(3, 2),
        ];
        let planet = Planet::new(5, 4, obstacles);
        let rover = Rover::new(0, 0, "N");
        let rover = execute_commands("RFF", planet, rover);

        assert_eq!(rover, Err(MissionError::HitObstacle(Position { x: 1, y: 0 })));
    }

    #[test]
    fn parse_planet_with_valid_and_invalid_arguments() {
        assert_eq!(parse_planet("5x4", ""), Ok(Planet::new(5, 4, vec![])));
        assert_eq!(parse_planet("10x4000", ""), Ok(Planet::new(10, 4000, vec![])));
        assert_eq!(parse_planet("5x4x6", ""), Err(String::from("invalid dimensions `5x4x6`")));
        assert_eq!(parse_planet("AAAx4000", ""), Err(String::from("invalid dimensions `AAAx4000`")));
        assert_eq!(parse_planet("10xAAA", ""), Err(String::from("invalid dimensions `10xAAA`")));
        assert_eq!(parse_planet("x4000", ""), Err(String::from("invalid dimensions `x4000`")));
        assert_eq!(parse_planet("asdads", ""), Err(String::from("invalid dimensions `asdads`")));
        assert_eq!(parse_planet("10x", ""), Err(String::from("invalid dimensions `10x`")));
        assert_eq!(parse_planet("134", ""), Err(String::from("invalid dimensions `134`")));
    }

    #[test]
    fn parse_planet_with_obstacles() {
        assert_eq!(
            parse_planet("5x4", "2,1 0,2"),
            Ok(Planet::new(5, 4, vec![Position::new(2, 1), Position::new(0, 2)]))
        );
        assert_eq!(
            parse_planet("5x4", "turbofish"),
            Err(String::from("invalid coordinates `turbofish`"))
        );
        assert_eq!(
            parse_planet("5x4", "2,1,4"),
            Err(String::from("invalid coordinates `2,1,4`"))
        );
        assert_eq!(
            parse_planet("5x4", "2 1,4"),
            Err(String::from("invalid coordinates `2 1,4`"))
        );
    }

    #[test]
    fn parse_rover_with_valid_and_invalid_arguments() {
        assert_eq!(parse_rover("0,0", "N"), Ok(Rover::new(0, 0, "N")));
        assert_eq!(parse_rover("AAAA", "N"), Err(vec![MissionError::InvalidCoordinates(String::from("AAAA"))]));
        assert_eq!(parse_rover("1,1", "invalid"), Err(vec![MissionError::InvalidDirection(String::from("invalid"))]));
        assert_eq!(
            parse_rover("AAA", "invalid"),
            Err(vec![MissionError::InvalidCoordinates(String::from("AAA")), MissionError::InvalidDirection(String::from("invalid"))])
        );
    }
}
