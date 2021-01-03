use std::{fmt, ops};

#[derive(Copy, Clone, PartialEq, Debug)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn norm(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn unitary(self) -> Vec2 {
        let norm = self.norm();
        Vec2 {
            x: self.x / norm,
            y: self.y / norm,
        }
    }
}

struct Rotator {
    beta: f64,
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, _rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl ops::Mul<isize> for Vec2 {
    type Output = Vec2;

    fn mul(self, _rhs: isize) -> Vec2 {
        Vec2 {
            x: self.x * _rhs as f64,
            y: self.y * _rhs as f64,
        }
    }
}
impl ops::Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, _rhs: f64) -> Vec2 {
        Vec2 {
            x: self.x * _rhs,
            y: self.y * _rhs,
        }
    }
}

impl ops::Mul<Rotator> for Vec2 {
    type Output = Vec2;

    fn mul(self, _rhs: Rotator) -> Vec2 {
        let cos = _rhs.beta.to_radians().cos();
        let sin = _rhs.beta.to_radians().sin();
        Vec2 {
            x: self.x * cos - sin * self.y,
            y: self.x * sin + cos * self.y,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct ShipState {
    pos: Vec2,
    rot: f64,
    waypoint: Vec2,
}

impl ShipState {
    fn manhattan_distance(self) -> i64 {
        (self.pos.x.abs() + self.pos.y.abs()).round() as i64
    }

    fn applyCommand(self, command: Command) -> ShipState {
        let mut result = self.clone();
        match command.action {
            Action::North => {
                result.waypoint = result.waypoint + Vec2 { x: 0., y: 1. } * command.amount;
            }
            Action::South => {
                result.waypoint = result.waypoint + Vec2 { x: 0., y: -1. } * command.amount;
            }
            Action::East => {
                result.waypoint = result.waypoint + Vec2 { x: 1., y: 0. } * command.amount;
            }
            Action::West => {
                result.waypoint = result.waypoint + Vec2 { x: -1., y: 0. } * command.amount;
            }
            Action::Left => {
                result.waypoint = result.waypoint
                    * Rotator {
                        beta: command.amount,
                    };
            }
            Action::Right => {
                result.waypoint = result.waypoint
                    * Rotator {
                        beta: -command.amount,
                    };
            }
            Action::Forward => {
                result.pos = result.pos + result.waypoint * command.amount;

                // result.pos.x = self.pos.x + self.rot.cos() * command.amount;
                // result.pos.y = self.pos.y + self.rot.sin() * command.amount;
            }
        }
        result
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct Command {
    action: Action,
    amount: f64,
}

fn read(txt: &str) -> Vec<Command> {
    txt.split("\n")
        .map(|row| {
            let mut chars = row.chars();

            let action = match chars.next().unwrap() {
                'N' => Action::North,
                'S' => Action::South,
                'E' => Action::East,
                'W' => Action::West,
                'L' => Action::Left,
                'R' => Action::Right,
                'F' => Action::Forward,
                _ => panic!("Unknown "),
            };
            let amount = chars.as_str().parse::<f64>().unwrap();
            println!("{} {}", action, amount);
            Command { action, amount }
        })
        .collect()
}

pub fn day_twelve() {
    let mut intial_position = ShipState {
        pos: Vec2 { x: 0., y: 0. },
        rot: 0.,
        waypoint: Vec2 { x: 10., y: 1. },
    };
    let commands = read(include_str!("../day12.txt"));

    let d = commands
        .into_iter()
        .fold(intial_position, |p, command| p.applyCommand(command))
        .manhattan_distance();

    println!("{}", d)

    // commands.iter().filter_map(|x| x).for_each(|command| {
    //
    // } );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let a = Vec2 { x: 1., y: 0. } + Vec2 { x: 0., y: 1. };
        assert_eq!(a.x, 1.);
        assert_eq!(a.y, 1.);
        let b = Vec2 { x: 1., y: 1. } * 10;
        assert_eq!(b.x, 10.);
        assert_eq!(b.y, 10.);
    }

    #[test]
    fn test_11() {
        let a = Vec2 { x: 1., y: 0. };
        assert_eq!(a * Rotator { beta: 90. }, Vec2 { x: 0., y: 1. });
    }

    #[test]
    fn test_12() {
        let mut intial_position = ShipState {
            pos: Vec2 { x: 0., y: 0. },
            rot: 0.,
            waypoint: Vec2 { x: 10., y: 1. },
        };

        let mut state: ShipState = intial_position.applyCommand(Command {
            action: Action::Forward,
            amount: 10.,
        });

        assert_eq!(state.pos.x, 100.0);
        assert_eq!(state.pos.y, 10.);

        state = state.applyCommand(Command {
            action: Action::North,
            amount: 3.,
        });
        assert_eq!(state.pos.x, 100.0);
        assert_eq!(state.pos.y, 10.);

        state = state.applyCommand(Command {
            action: Action::Forward,
            amount: 7.,
        });
        assert_eq!(state.pos.x, 170.0);
        assert_eq!(state.pos.y, 38.);

        state = state.applyCommand(Command {
            action: Action::Right,
            amount: 90.,
        });
        assert_eq!(state.pos.x, 170.0);
        assert_eq!(state.pos.y, 38.);

        state = state.applyCommand(Command {
            action: Action::Forward,
            amount: 11.,
        });
        assert_eq!(state.pos.x, 214.0);
        assert_eq!(state.pos.y, -72.);
    }
}
