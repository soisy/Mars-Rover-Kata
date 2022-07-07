#![allow(unused)]

mod domain;

use domain::*;

fn main() {
    println!("Hello, world!");
}

pub fn parse_commands(commands: &str) -> Vec<char> {
    let allowed_commands = vec!['L', 'R', 'F', 'B'];
    commands.chars()
        .filter(|c| allowed_commands.contains(c))
        .collect()
}

fn parse_planet(dimensions: &str, obstacles: &str) -> Result<Planet, String> {
    let (w, h) = parse_dimensions(dimensions)?;
    let obstacles = parse_obstacles(obstacles)?;
    Ok(Planet::new(w, h, obstacles))
}

fn parse_dimensions(dimensions: &str) -> Result<(usize, usize), String> {
    dimensions
        .split("x")
        .map(|x| x.parse::<usize>().map_err(|e| e.to_string()))
        .collect::<Result<Vec<usize>, String>>()
        .and_then(|dimensions| match dimensions.len() {
            2 => Ok((dimensions[0], dimensions[1])),
            _ => Err("invalid number of dimensions".to_string()),
        })
}

fn parse_obstacles(obstacles: &str) -> Result<Vec<Position>, String> {
    obstacles
        .split(" ") // -> "2,1"
        .filter(|x| x.len() > 0)
        .map(parse_position)
        .collect::<Result<Vec<Position>, String>>()
        .and_then(|v| match v.len() {
            0 => Ok(vec![]),
            _ => Ok(v),
        })
}

fn parse_position(position: &str) -> Result<Position, String> {
    position
        .split(",")
        .map(|x| x.parse::<usize>().map_err(|e| e.to_string()))
        .collect::<Result<Vec<usize>, String>>()
        .and_then(|position| match position.len() {
            2 => Ok(Position::new(position[0], position[1])),
            _ => Err("invalid number of coordinates".to_string()),
        })
}

pub fn execute_commands(commands: &str, planet: Planet, rover: Rover) -> Result<Rover, MissionError> {
    parse_commands(commands).iter()
        .try_fold(rover, |rover, &command| {
            execute(Command::new(command), &planet, rover)
        })
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
        assert_eq!(parse_planet("5x4", ""), Ok(Planet::new(5,4, vec![])));
        assert_eq!(parse_planet("10x4000", ""), Ok(Planet::new(10,4000, vec![])));
        assert_eq!(parse_planet("5x4x6", ""), Err(String::from("invalid number of dimensions")));
        assert_eq!(parse_planet("AAAx4000", ""), Err(String::from("invalid digit found in string")));
        assert_eq!(parse_planet("10xAAA", ""), Err(String::from("invalid digit found in string")));
        assert_eq!(parse_planet("x4000", ""), Err(String::from("cannot parse integer from empty string")));
        assert_eq!(parse_planet("asdads", ""), Err(String::from("invalid digit found in string")));
        assert_eq!(parse_planet("10x", ""), Err(String::from("cannot parse integer from empty string")));
        assert_eq!(parse_planet("134", ""), Err(String::from("invalid number of dimensions")));
    }

    #[test]
    fn parse_planet_with_obstacles() {
        assert_eq!(
            parse_planet("5x4", "2,1 0,2"),
            Ok(Planet::new(5,4, vec![Position::new(2,1), Position::new(0,2)]))
        );
        assert_eq!(
            parse_planet("5x4", "turbofish"),
            Err(String::from("invalid digit found in string"))
        );
        assert_eq!(
            parse_planet("5x4", "2,1,4"),
            Err(String::from("invalid number of coordinates"))
        );
        assert_eq!(
            parse_planet("5x4", "2 1,4"),
            Err(String::from("invalid number of coordinates"))
        );
    }
}
