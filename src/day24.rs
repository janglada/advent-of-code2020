extern crate peg;

use std::collections::HashMap;

pub enum Neighbour {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

pub type Hex = (i32, i32);

peg::parser! {
  grammar instructions_parser() for str {

    rule east() -> Hex
        = "e" { (1, 0) }

    rule south_east() -> Hex
        = "se" { (0, 1) }

    rule south_west() -> Hex
        = "sw" { (-1, 1)  }

    rule west() -> Hex
        = "w" { (-1, 0) }

    rule north_west() -> Hex
        = "nw" { (0, -1) }

    rule north_east() -> Hex
        = "ne" { (1, -1)}

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
    let mut iter = include_str!("../day24.txt").split("\n").for_each(|line| {
        let tile0: Hex = (0, 0);
        println!("parsing {}", line);
        let dst_tile = instructions_parser::row(line)
            .ok()
            .unwrap()
            .iter()
            .fold(tile0, |acc, step| (acc.0 + step.0, acc.1 + step.1));
        *tiles.entry(dst_tile).or_insert(0) += 1;
    });

    let count = tiles.values().filter(|&&v| v % 2 == 1).count();
    println!("{}", count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_() {
        let tile: Vec<Hex> = instructions_parser::row("e").ok().unwrap();

        assert_eq!(1, tile.len());
        assert_eq!(1, tile.get(0).unwrap().0);
        assert_eq!(0, tile.get(0).unwrap().1);

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
