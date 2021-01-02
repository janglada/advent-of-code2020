use itertools::Itertools;
use std::cmp;
use std::fmt;
use std::path::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: i64,
    y: i64,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum State {
    Floor,
    Empty,
    Occupied,
}
#[derive(Clone, Copy, PartialEq)]
struct Tile {
    prev_state: State,
    new_state: State,
    pos: Vec2,
}

impl Tile {
    fn reset(&mut self) {
        self.prev_state = self.new_state;
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::Floor => write!(f, "."),
            State::Empty => write!(f, "L"),
            State::Occupied => write!(f, "#"),
        }
    }
}

struct FloorGrid {
    tiles: Vec<Tile>,
    size: Vec2,
}

impl FloorGrid {
    fn iter(&self) -> impl Iterator<Item = Tile> + '_ {
        self.tiles.iter().copied()
    }
}

impl FloorGrid {
    fn index(&self, pos: Vec2) -> Option<usize> {
        if (0..self.size.x).contains(&pos.x) && (0..self.size.y).contains(&pos.y) {
            Some((pos.y + pos.x * self.size.y) as _)
        } else {
            None
        }
    }

    fn set(&mut self, pos: Vec2, state: State) {
        // println!("SETTING {} {} {}", pos.x, pos.y, state);
        if let Some(index) = self.index(pos) {
            // self.tiles[index].prev_state = self.tiles[index].new_state;
            self.tiles[index].new_state = state;
        }
    }

    fn reset_state(&mut self, pos: Vec2, _state: State) {
        if let Some(index) = self.index(pos) {
            self.tiles[index].prev_state = self.tiles[index].new_state;
        }
    }

    fn neighbor_positions(pos: Vec2) -> impl Iterator<Item = Vec2> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|&(x, y)| !(x == 0 && y == 0))
            .map(move |(dx, dy)| Vec2 {
                x: pos.x + dx,
                y: pos.y + dy,
            })
    }

    fn get(&self, pos: Vec2) -> Option<Tile> {
        self.index(pos).map(|index| self.tiles[index])
    }

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn applyRules(&mut self) -> (usize, usize) {
        let mut occuppied: usize = 0;
        let copied_vec = self.tiles.to_vec();
        let num_changes = copied_vec
            .iter()
            .map(|tile| {
                let occupied_neighbours = FloorGrid::neighbor_positions(tile.pos)
                    .map(|pos| self.get(pos))
                    .filter_map(|t| t)
                    .filter(|t| t.prev_state == State::Occupied)
                    .count();

                let after = match tile.prev_state {
                    State::Empty => {
                        if occupied_neighbours == 0 {
                            occuppied = occuppied + 1;
                            State::Occupied
                        } else {
                            State::Empty
                        }
                    }
                    State::Occupied => {
                        if occupied_neighbours >= 4 {
                            State::Empty
                        } else {
                            occuppied = occuppied + 1;
                            State::Occupied
                        }
                    }
                    State::Floor => State::Floor,
                };

                let changed = if after != tile.prev_state { 1 } else { 0 };

                self.set(tile.pos, after);
                // println!(
                //     "{} {} {} --> {}",
                //     tile.pos.x, tile.pos.y, tile.prev_state, after
                // );

                changed
            })
            .sum();

        self.tiles.iter_mut().for_each(|mut t| {
            t.reset();
        });

        println!("num_changes {}", num_changes);

        (num_changes, occuppied)
    }
}

impl fmt::Display for FloorGrid {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.size.x {
            // println!("{} ", i);
            for j in 0..self.size.y {
                let tile = self.get(Vec2 { x: i, y: j }).unwrap();

                match tile.prev_state {
                    State::Floor => write!(f, "."),
                    State::Occupied => write!(f, "#"),
                    State::Empty => write!(f, "L"),
                };
            }
            write!(f, "\n");
        }
        write!(f, "")
    }
}

fn read(txt: &str) -> FloorGrid {
    // let mut grid: Vec<State> = Vec::new();
    let mut columns: i64 = 0;
    let mut rows: i64 = 0;

    let mut d: Vec<Tile> = Vec::new();
    let tiles = txt
        .chars()
        .filter_map(|c| {
            if c == '\n' {
                rows += 1;
                columns = 0;
                None
            } else {
                let state = match c {
                    '.' => State::Floor,
                    'L' => State::Empty,
                    '#' => State::Occupied,
                    _ => State::Empty,
                };

                let tile = Some(Tile {
                    prev_state: state,
                    new_state: state,
                    pos: Vec2 {
                        x: rows,
                        y: columns,
                    },
                });
                // println!("{} {}", rows, columns);
                columns += 1;
                tile
            }
        })
        .collect();
    // txt.split("\n").for_each(|row| {
    //     rows = rows + 1;
    //     cols = 0;
    //     d.push(
    //         row.chars()
    //             .map(|c| {
    //                 cols = cols + 1;
    //                 match c {
    //                     '.' => State::Floor,
    //                     'L' => State::Empty,
    //                     '#' => State::Occupied,
    //                     _ => State::Empty,
    //                 }
    //             })
    //             .collect(),
    //     )
    // });

    println!("rows {}", rows);
    println!("cols {}", columns);

    FloorGrid {
        size: Vec2 {
            x: rows + 1,
            y: columns,
        },
        tiles,
    }
}

pub fn day_eleven() {
    let mut grid = read(include_str!("../day11.txt"));
    // println!("{}", grid.into_iter());
    //
    loop {
        let (num_changes, occupied) = grid.applyRules();
        println!("num_changes {}  occupied {} ", num_changes, occupied);
        if num_changes == 0 {
            break;
        }
    }

    // for (i, state) in grid
    //     .into_iter()
    //     .map(|pos| match pos.0 {
    //         State::Empty => {
    //             if pos.1 == 0 {
    //                 State::Occupied
    //             } else {
    //                 State::Empty
    //             }
    //         }
    //         State::Occupied => {
    //             if pos.1 >= 4 {
    //                 State::Empty
    //             } else {
    //                 State::Occupied
    //             }
    //         }
    //         State::Floor => State::Floor,
    //     })
    //     .enumerate()
    // {
    //     if i % rows == 0 {
    //         println!();
    //     } else {
    //         print!("{}", state);
    //     }
    // }
    // print!("{}", a);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0() {
        println!("AAA");
        let mut grid = read(include_str!("../day11_test.txt"));

        let tile = grid.get(Vec2 { x: 0, y: 0 }).unwrap();

        assert_eq!(tile.pos.x, 0);
        assert_eq!(tile.pos.y, 0);
        assert_eq!(tile.prev_state, State::Empty);

        println!("{}", grid);
        // // println!("{}", grid.into_iter().next().unwrap());
        let (num_changes, occupied) = grid.applyRules();
        println!("{}", grid);
        assert_eq!(
            grid.to_string(),
            include_str!("../day11_test_expected_1.txt").to_owned()
        );
    }
    #[test]
    fn test_1() {
        println!("AAA");
        let mut grid = read(include_str!("../day11_test.txt"));
        println!("{}", grid);
        // println!("BBBB");
        // // println!("{}", grid.into_iter().next().unwrap());
        let (num_changes, occupied) = grid.applyRules();
        grid.applyRules();
        assert_eq!(
            grid.to_string(),
            include_str!("../day11_test_expected_2.txt").to_owned()
        );
    }

    // #[test]
    // fn test_3() {
    //     vec![1i32, 0, -1i32]
    //         .iter()
    //         .tuple_combinations()
    //         .for_each(|pos| println!("{:?}", pos))
    // }

    // #[test]
    // fn test_3() {
    //     vec![1i32, 0, -1i32]
    //         .iter()
    //         .tuple_combinations()
    //         .for_each(|pos| println!("{:?}", pos))
    // }
}
