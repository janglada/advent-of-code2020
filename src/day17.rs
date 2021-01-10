use crate::day17::CubeState::Active;
use itertools::Itertools;

use std::collections::HashMap;

type Coordinates = (isize, isize, isize, isize);

#[derive(Clone, Copy, PartialEq)]
enum CubeState {
    Active,
    Inactive,
}

struct Cube {
    state: CubeState,
    prev_state: CubeState,
}

type Space = HashMap<Coordinates, Cube>;

fn neighbour_positions(coord: Coordinates) -> impl Iterator<Item = Coordinates> {
    let cx = coord.0;
    let cy = coord.1;
    let cz = coord.2;
    let cw = coord.3;
    iproduct!(-1..=1, -1..=1, -1..=1, -1..=1)
        .filter(|&x| !(x.0 == 0 && x.1 == 0 && x.2 == 0 && x.3 == 0))
        .map(move |(x, y, z, w)| (cx + x, cy + y, cz + z, cw + w))
}

fn read(txt: &str) -> Space {
    let mut space = Space::new();

    // at max

    txt.split("\n").enumerate().for_each(|(i, c)| {
        c.chars().enumerate().for_each(|(j, c)| {
            let initial_state = match c {
                '#' => CubeState::Active,
                '.' => CubeState::Inactive,
                _ => CubeState::Inactive,
            };
            let cloned = initial_state.clone();
            let position = (i as isize, j as isize, 0, 0);
            space.insert(
                (i as isize, j as isize, 0, 0),
                Cube {
                    state: initial_state,
                    prev_state: cloned,
                },
            );

            // and it's neighbours...
            neighbour_positions(position).for_each(|p| {
                match space.get(&p) {
                    None => {
                        space.insert(
                            p,
                            Cube {
                                state: CubeState::Inactive,
                                prev_state: CubeState::Inactive,
                            },
                        );
                    }
                    Some(_) => {
                        // dont override
                    }
                }
            });
        });
    });

    space
}

pub(crate) fn day_seventeen() {
    let mut space = read(include_str!("../day17.txt"));

    iproduct!(-20..=20, -20..=20, -20..=20, -20..=20).for_each(|p| match space.get(&p) {
        None => {
            space.insert(
                p,
                Cube {
                    state: CubeState::Inactive,
                    prev_state: CubeState::Inactive,
                },
            );
        }
        Some(_) => {}
    });
    for (i, idx) in (0..6).enumerate() {
        space = space
            .iter()
            .map(|(&coord, e)| {
                let num_active_neigh = neighbour_positions(coord)
                    .filter(|p| match space.get(&p) {
                        None => {
                            // init position
                            // println!("EMPTY?!? {:?}", p);
                            false
                        }
                        Some(cube) => cube.state == CubeState::Active,
                    })
                    .count();

                let new_state = match e.prev_state {
                    Active => {
                        if num_active_neigh == 2 || num_active_neigh == 3 {
                            CubeState::Active
                        } else {
                            CubeState::Inactive
                        }
                    }
                    CubeState::Inactive => {
                        if num_active_neigh == 3 {
                            CubeState::Active
                        } else {
                            CubeState::Inactive
                        }
                    }
                };

                (
                    coord,
                    Cube {
                        state: new_state,
                        prev_state: e.prev_state,
                    },
                )
            })
            .collect::<HashMap<Coordinates, Cube>>()
            .iter()
            .map(|(&coords, c)| {
                (
                    coords,
                    Cube {
                        state: c.state,
                        prev_state: c.state,
                    },
                )
            })
            .collect::<HashMap<Coordinates, Cube>>();
    }

    let count_active = space
        .iter()
        .filter(|(coords, cube)| cube.state == CubeState::Active)
        .count();

    println!("{}", count_active);

    // space.iter_mut().for_each(|(&coord, cube)| {
    //     let count = neighbour_positions(coord)
    //         .filter(|p| match space.get(&p) {
    //             None => {
    //                 // init position
    //                 println!("EMPTY?!?");
    //                 false
    //             }
    //             Some(cube) => cube.state == CubeState::Active,
    //         })
    //         .count();
    // });
}

#[test]
fn test_3() {
    (-1..=1)
        .combinations_with_replacement(3)
        .for_each(|x| println!("{:?}", x));

    println!("****************");
    (-1..=1).permutations(3).for_each(|x| println!("{:?}", x));

    let mut neighbours = Vec::new();
    for i in -1..=1 {
        for j in -1..=1 {
            for k in -1..=1 {
                if !(i == 0 && j == 0 && k == 0) {
                    neighbours.push((i, j, k))
                }
            }
        }
    }

    println!("****************");
    println!("{:?}", neighbours);
    println!("{}", neighbours.len());

    let a1 = [-1, 0, 1];
    let a2 = [-1, 0, 1];
    let a3 = [-1, 0, 1];

    let vec: Vec<(isize, isize, isize)> = iproduct!(-1..=1, -1..=1, -1..=1)
        .filter(|&x| !(x.0 == 0 && x.1 == 0 && x.2 == 0))
        .collect();
    println!("{}", vec.len());
}
