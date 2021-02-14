extern crate peg;

use itertools::Itertools;
use std::collections::HashMap;
use std::ops;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct Hex {
    x: i32,
    y: i32,
}

impl Hex {
    pub fn new(x: i32, y: i32) -> Hex {
        Hex { x, y }
    }

    pub fn neighbours(&self) -> Vec<Hex> {
        vec![
            *self + Hex::new(1, 0),
            *self + Hex::new(0, 1),
            *self + Hex::new(-1, 1),
            *self + Hex::new(-1, 0),
            *self + Hex::new(0, -1),
            *self + Hex::new(1, -1),
        ]
    }
}

impl ops::Add<Hex> for Hex {
    type Output = Hex;

    fn add(self, _rhs: Hex) -> Hex {
        Hex::new(self.x + _rhs.x, self.y + _rhs.y)
    }
}
impl ops::Add<&Hex> for Hex {
    type Output = Hex;
    fn add(self, _rhs: &Hex) -> Hex {
        Hex::new(self.x + _rhs.x, self.y + _rhs.y)
    }
}
// pub type Hex = (i32, i32);

peg::parser! {
  grammar instructions_parser() for str {

    rule east() -> Hex
        = "e" { Hex::new(1, 0) }

    rule south_east() -> Hex
        = "se" { Hex::new(0, 1) }

    rule south_west() -> Hex
        = "sw" { Hex::new(-1, 1)  }

    rule west() -> Hex
        = "w" { Hex::new(-1, 0) }

    rule north_west() -> Hex
        = "nw" { Hex::new(0, -1) }

    rule north_east() -> Hex
        = "ne" { Hex::new(1, -1)}

    pub rule item() -> Hex
      = n: (east() / south_east() / south_west() /  west() / north_west() / north_east()) {
       n
      }



    pub rule row() -> Vec<Hex>
      = row:item() *  {
        row
      }


    }

}

pub fn run() {
    let mut tiles: HashMap<Hex, u32> = HashMap::new();
    include_str!("../day24.txt").split("\n").for_each(|line| {
        let tile0: Hex = Hex::new(0, 0);
        let dst_tile = instructions_parser::row(line)
            .ok()
            .unwrap()
            .iter()
            .fold(tile0, |acc, step| acc + step);
        *tiles.entry(dst_tile).or_insert(0) += 1;
    });

    let count = tiles.values().filter(|&&v| v % 2 == 1).count();
    println!("{}", count);

    for i in 1..101 {
        let mut new_tiles: HashMap<Hex, u32> = HashMap::new();
        // println!("size {}", tiles.len());
        tiles
            .keys()
            .flat_map(|&v| v.neighbours().into_iter())
            .unique()
            .for_each(|t| {
                let is_black = tiles.get(&t).unwrap_or(&0) % 2 == 1;
                let black_neighbours = t
                    .neighbours()
                    .iter()
                    .filter(|&t| tiles.get(t).unwrap_or(&0) % 2 == 1)
                    .count();
                // println!("BLACK {}  {}", is_black, black_neighbours);
                if is_black && (black_neighbours == 0 || black_neighbours > 2) {
                    new_tiles.insert(t, 0);
                } else if !is_black && black_neighbours == 2 {
                    new_tiles.insert(t, 1);
                } else {
                    let v = if is_black { 1 } else { 0 };
                    new_tiles.insert(t, v);
                }
            });
        tiles = new_tiles;
    }

    let count = tiles.values().filter(|&&v| v % 2 == 1).count();
    println!("{}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_() {
        let tile: Vec<Hex> = instructions_parser::row("e").ok().unwrap();

        assert_eq!(1, tile.len());
        assert_eq!(1, tile.get(0).unwrap().x);
        assert_eq!(0, tile.get(0).unwrap().y);

        println!("{:?}", tile);
    }

    #[test]
    fn test_tile2() {
        let tile: Vec<Hex> = instructions_parser::row("sweswneswswswswwswswswseneswswnwswwne")
            .ok()
            .unwrap();

        // assert_eq!(1, tile.len());
        // assert_eq!(1, tile.get(0).unwrap().0);
        // assert_eq!(0, tile.get(0).unwrap().1);

        println!("{:?}", tile);
    }
}
