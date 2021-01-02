use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
struct Vec2 {
    x: f64,
    y: f64,
    rot: f64,
}

impl Vec2 {
    fn manhattan_distance(self) -> i64 {
        (self.x.abs() + self.y.abs()).round() as i64
    }

    fn applyCommand(&mut self, command: Command) {
        match command.action {
            Action::North => {
                self.y = self.y + command.amount;
            }
            Action::South => {
                self.y = self.y - command.amount;
            }
            Action::East => {
                self.x = self.x + command.amount;
            }
            Action::West => {
                self.x = self.x - command.amount;
            }
            Action::Left => {
                self.rot = self.rot + command.amount * std::f64::consts::PI / 180.;
            }
            Action::Right => {
                self.rot = self.rot - command.amount * std::f64::consts::PI / 180.;
            }
            Action::Forward => {
                self.x = self.x + self.rot.cos() * command.amount;
                self.y = self.y + self.rot.sin() * command.amount;
            }
        }
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
    let mut intial_position = Vec2 {
        x: 0.,
        y: 0.,
        rot: 0.,
    };
    let commands = read(include_str!("../day12.txt"));

    commands
        .into_iter()
        .for_each(|command| intial_position.applyCommand(command));

    println!("{}", intial_position.manhattan_distance())

    // commands.iter().filter_map(|x| x).for_each(|command| {
    //
    // } );
}
