extern crate peg;

use itertools::Itertools;

use itertools::__std_iter::Flatten;
use std::collections::HashMap;
#[derive(Debug)]
enum BorderId {
    North,
    East,
    West,
    South,
}
#[derive(Debug)]
pub struct Border {
    border_id: BorderId,
    data: [u8; 10],
    used: bool,
}

impl PartialEq for Border {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data || self.data.iter().eq(other.data.iter().rev())
    }
}

pub struct Tile {
    id: usize,
    borders: [Border; 4],
}

impl Tile {
    pub fn new(id: usize, data: Vec<Vec<u8>>) -> Tile {
        let mut north_border_data = [0; 10];
        let mut south_border_data = [0; 10];
        let mut east_border_data = [0; 10];
        let mut west_border_data = [0; 10];

        for (i, row) in data.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                if i == 0 {
                    north_border_data[j] = *value;
                }
                if i == 9 {
                    south_border_data[j] = *value;
                }

                if j == 0 {
                    east_border_data[i] = *value;
                }
                if j == 9 {
                    west_border_data[i] = *value;
                }
            }
        }

        Tile {
            id,
            borders: [
                Border {
                    border_id: BorderId::North,
                    data: north_border_data,
                    used: false,
                },
                Border {
                    border_id: BorderId::East,
                    data: east_border_data,
                    used: false,
                },
                Border {
                    border_id: BorderId::South,
                    data: south_border_data,
                    used: false,
                },
                Border {
                    border_id: BorderId::West,
                    data: west_border_data,
                    used: false,
                },
            ],
        }
    }
}

peg::parser! {
  grammar tiles_parser() for str {
    rule number() -> usize
      = n:$(['0'..='9']+) { n.parse().unwrap() }

    pub rule tile_id() -> usize
      = "Tile " r:number() ":" {r}

    rule tile_data() -> Vec<Vec<u8>>
      = d:tile_data_row() ** "\n" {
            d
      }

    rule tile_data_row() -> Vec<u8>
      = v:position() * <10>{ v }

    rule position() -> u8
        = occupied() / empty()

    rule occupied() -> u8
        = "#" {
            1
        }
    rule empty() -> u8
        = "." {
            0
        }


    pub rule tile() -> Tile
      = id:tile_id()  "\n" data:tile_data() {
        Tile::new(id, data)
      }



    pub rule tiles() -> Vec<Tile>
      = t:tile() ** "\n"  {
        t
      }


    }

}

pub fn day_twenty() {
    let mut used_tiles: HashMap<usize, usize> = HashMap::new();
    let tiles1: Vec<Tile> = include_str!("../day20.txt")
        .split("\n\n")
        .map(|s| tiles_parser::tile(s).ok().unwrap())
        .collect();

    for vpair in tiles1.iter().combinations(2) {
        let first = vpair.first().unwrap();
        let last = vpair.last().unwrap();
        let first_id = first.id;
        let last_id = last.id;
        first.borders.iter().for_each(|b| {
            last.borders
                .iter()
                .filter(|&other| other == b)
                .for_each(|mut other| {
                    *used_tiles.entry(first_id).or_insert(0) += 1;
                    *used_tiles.entry(last_id).or_insert(0) += 1;
                })
        })
    }

    let s: usize = used_tiles
        .iter()
        .filter(|(&key, &v)| v == 2)
        .map(|(&key, &v)| key)
        .product();
    println!("{}", s)
    //
    // let mut reader = Reader::new(Cursor::new(t.data))
    //     .with_guessed_format()
    //     .expect("Cursor io never fails");
    // assert_eq!(reader.format(), Some(ImageFormat::Pnm));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_title() {
        let r: Result<usize, _> = tiles_parser::tile_id("Tile 3673:");
        assert_eq!(true, r.is_ok());
        let tid = r.unwrap();
        assert_eq!(tid, 3673);
        println!("{}", tid);
    }

    #[test]
    fn test_tile() {
        let r: Result<Tile, _> = tiles_parser::tile(
            r#"Tile 3779:
....###..#
#..#......
...##.....
#.#..#...#
..##.#....
##.##....#
#.........
.....#...#
#.###...#.
.#####.#.#"#,
        );
        assert_eq!(true, r.is_ok());
        let tid = r.unwrap();

        // println!("{:?}", tid.data);
    }

    #[test]
    fn test_border_eq() {
        let b1 = Border {
            used: false,
            border_id: BorderId::North,
            data: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        let b2 = Border {
            used: false,
            border_id: BorderId::North,
            data: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        let b3 = Border {
            used: false,
            border_id: BorderId::North,
            data: [0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        let b4 = Border {
            used: false,
            border_id: BorderId::North,
            data: [0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        };
        assert_eq!(b2, b1);
        assert_ne!(b1, b3);
        assert_ne!(b2, b3);
        assert_eq!(b1, b4);
        assert_eq!(b2, b4);
    }
}
