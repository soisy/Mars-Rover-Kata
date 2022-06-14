#![allow(unused)]

mod domain;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn rotate_right_command() {
        let planet = Planet { w: 4, h: 5};
        let rover = Rover::new(0, 0, "N");
        let rover = execute(Command::TurnRight, planet, rover);

        assert_eq!(rover, Rover::new(0, 0, "E"));
    }


    #[test]
    fn rotate_left_command() {
        let planet = Planet { w: 4, h: 5};
        let rover = Rover::new(0, 0, "N");
        let rover = execute(Command::TurnLeft, planet, rover);

        assert_eq!(rover, Rover::new(0, 0, "W"));
    }

    #[test]
    fn move_forward_command() {
        let planet = Planet { w: 4, h: 5};
        let rover = Rover::new(0, 0, "N");
        let rover = execute(Command::MoveForward, planet, rover);

        assert_eq!(rover, Rover::new(0, 1, "N"));
    }

    #[test]
    fn move_backward_command() {
        let planet = Planet { w: 4, h: 5};
        let rover = Rover::new(0, 1, "N");
        let rover = execute(Command::MoveBackward, planet, rover);

        assert_eq!(rover, Rover::new(0, 0, "N"));
    }

    #[test]
    fn pacman_effect_north_forward() {
        let planet = Planet { w: 4, h: 5};
        let rover = Rover::new(0, 4, "N");
        let rover = execute(Command::MoveForward, planet, rover);

        assert_eq!(rover, Rover::new(0, 0, "N"));
    }

    #[test]
    fn pacman_effect_sud_forward() {
        let planet = Planet { w: 4, h: 5};
        let rover = Rover::new(0, 0, "S");
        let rover = execute(Command::MoveForward, planet, rover);

        assert_eq!(rover, Rover::new(0, 4, "S"));
    }

    #[test]
    fn pacman_effect_east_forward() {
        let planet = Planet { w: 4, h: 5};
        let rover = Rover::new(3, 0, "E");
        let rover = execute(Command::MoveForward, planet, rover);

        assert_eq!(rover, Rover::new(0, 0, "E"));
    }

    #[test]
    fn pacman_effect_west_forward() {
        let planet = Planet { w: 4, h: 5};
        let rover = Rover::new(0, 0, "W");
        let rover = execute(Command::MoveForward, planet, rover);

        assert_eq!(rover, Rover::new(3, 0, "W"));
    }

    #[test]
    fn pacman_effect_north_backward() {
        let planet = Planet { w: 4, h: 5};
        let rover = Rover::new(0, 4, "S");
        let rover = execute(Command::MoveBackward, planet, rover);

        assert_eq!(rover, Rover::new(0, 0, "S"));
    }

    #[test]
    fn pacman_effect_south_backward() {
        let planet = Planet { w: 4, h: 5};
        let rover = Rover::new(0, 0, "N");
        let rover = execute(Command::MoveBackward, planet, rover);

        assert_eq!(rover, Rover::new(0, 4, "N"));
    }

    #[test]
    fn pacman_effect_east_backward() {
        let planet = Planet { w: 4, h: 5};
        let rover = Rover::new(3, 0, "W");
        let rover = execute(Command::MoveBackward, planet, rover);

        assert_eq!(rover, Rover::new(0, 0, "W"));
    }

    #[test]
    fn pacman_effect_west_backward() {
        let planet = Planet { w: 4, h: 5};
        let rover = Rover::new(0, 0, "E");
        let rover = execute(Command::MoveBackward, planet, rover);

        assert_eq!(rover, Rover::new(3, 0, "E"));
    }
}
