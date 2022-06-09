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
        let planet = Planet { h: 5, w: 4 };
        let rover = Rover::new(0, 0, "N");
        let rover = execute(Command::TurnRight, planet, rover);

        assert_eq!(rover, Rover::new(0, 0, "E"));
    }


    #[test]
    fn rotate_left_command() {
        let planet = Planet { h: 5, w: 4 };
        let rover = Rover::new(0, 0, "N");
        let rover = execute(Command::TurnLeft, planet, rover);

        assert_eq!(rover, Rover::new(0, 0, "W"));
    }

    #[test]
    fn move_forward_command() {
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
