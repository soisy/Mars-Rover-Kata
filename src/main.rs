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

pub fn execute_commands(commands: &str, planet: Planet, rover: Rover) -> Option<Rover> {
    Some(
        parse_commands(commands).iter()
        .fold(rover, |rover, &command| {
            execute(Command::new(command), &planet, rover)
        })
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn rotate_right_command() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(0, 0, "N");
        let rover = execute(Command::TurnRight, &planet, rover);

        assert_eq!(rover, Rover::new(0, 0, "E"));
    }


    #[test]
    fn rotate_left_command() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(0, 0, "N");
        let rover = execute(Command::TurnLeft, &planet, rover);

        assert_eq!(rover, Rover::new(0, 0, "W"));
    }

    #[test]
    fn move_forward_command() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(0, 0, "N");
        let rover = execute(Command::MoveForward, &planet, rover);

        assert_eq!(rover, Rover::new(0, 1, "N"));
    }

    #[test]
    fn move_backward_command() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(0, 1, "N");
        let rover = execute(Command::MoveBackward, &planet, rover);

        assert_eq!(rover, Rover::new(0, 0, "N"));
    }

    #[test]
    fn pacman_effect_north_forward() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(0, 4, "N");
        let rover = execute(Command::MoveForward, &planet, rover);

        assert_eq!(rover, Rover::new(0, 0, "N"));
    }

    #[test]
    fn pacman_effect_sud_forward() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(0, 0, "S");
        let rover = execute(Command::MoveForward, &planet, rover);

        assert_eq!(rover, Rover::new(0, 4, "S"));
    }

    #[test]
    fn pacman_effect_east_forward() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(3, 0, "E");
        let rover = execute(Command::MoveForward, &planet, rover);

        assert_eq!(rover, Rover::new(0, 0, "E"));
    }

    #[test]
    fn pacman_effect_west_forward() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(0, 0, "W");
        let rover = execute(Command::MoveForward, &planet, rover);

        assert_eq!(rover, Rover::new(3, 0, "W"));
    }

    #[test]
    fn pacman_effect_north_backward() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(0, 4, "S");
        let rover = execute(Command::MoveBackward, &planet, rover);

        assert_eq!(rover, Rover::new(0, 0, "S"));
    }

    #[test]
    fn pacman_effect_south_backward() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(0, 0, "N");
        let rover = execute(Command::MoveBackward, &planet, rover);

        assert_eq!(rover, Rover::new(0, 4, "N"));
    }

    #[test]
    fn pacman_effect_east_backward() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(3, 0, "W");
        let rover = execute(Command::MoveBackward, &planet, rover);

        assert_eq!(rover, Rover::new(0, 0, "W"));
    }

    #[test]
    fn pacman_effect_west_backward() {
        let planet = Planet::without_obstacles(4, 5);
        let rover = Rover::new(0, 0, "E");
        let rover = execute(Command::MoveBackward, &planet, rover);

        assert_eq!(rover, Rover::new(3, 0, "E"));
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

        assert_eq!(rover, Some(Rover::new(4, 3, "N")));
    }

    #[test]
    fn empty_command_string() {
        let planet = Planet::without_obstacles(5, 4);
        let rover = Rover::new(0, 0, "N");
        let rover = execute_commands("", planet, rover);

        assert_eq!(rover, Some(Rover::new(0, 0, "N")));
    }

    #[test]
    fn hit_obstacle() {
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

        assert_eq!(rover, Some(Rover::new(1, 0, "E")));
    }
}
