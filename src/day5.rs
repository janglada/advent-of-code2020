use itertools::Itertools;
use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
pub fn day_five() -> Result<(), Error> {
    let br = BufReader::new(File::open("day5.txt")?);
    let rows = br.lines().map(|ticket| get_id(&ticket.unwrap())).max();

    let my_str = include_str!("../day5.txt");
    let count = my_str
        .split("\n")
        .map(|ticket| get_id(ticket))
        // .max()
        .sorted()
        .for_each(|id| {
            println!("{}", id);
        });

    for slice in my_str
        .lines()
        .map(|ticket| get_id(ticket))
        .sorted()
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|slice| slice[1] - slice[0] == 2)
    {
        println!("{:?}", slice);
    }

    // for (i, v) in my_str.lines().skip(2).enumerate() {}

    // match count {
    //     Some(max) => println!("{}", max),
    //     _ => println!("Error!!"),
    // }

    Ok(())
}
struct Range {
    pub min: i32,
    pub max: i32,
}

fn get_id(ticket: &str) -> i32 {
    let b = ticket.to_string();
    println!("b {}", b);
    let rowStr = &b[0..7];
    let colStr = &b[7..10];
    row(rowStr).max * 8 + column(colStr).max
}
fn lower_half(mut range: Range) -> Range {
    let r = (range.max - range.min) / 2;
    range.max = range.min + r;
    range
}

fn upper_half(mut range: Range) -> Range {
    let r = (range.max - range.min) / 2;
    range.min = range.min + r;
    range
}

fn row(str: &str) -> Range {
    str.chars().fold(Range { min: 0, max: 127 }, |acc, x| {
        if x == 'F' {
            lower_half(acc)
        } else {
            upper_half(acc)
        }
    })
}

fn column(str: &str) -> Range {
    str.chars().fold(Range { min: 0, max: 7 }, |acc, x| {
        if x == 'L' {
            lower_half(acc)
        } else {
            upper_half(acc)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(row("FBFBBFF").max, 44);
        assert_eq!(row("BFFFBBF").max, 70);
        assert_eq!(row("FFFBBBF").max, 14);
        assert_eq!(row("BBFFBBF").max, 102);

        assert_eq!(column("RLR").max, 5);
        assert_eq!(column("RRR").max, 7);
        assert_eq!(column("RRR").max, 7);
        assert_eq!(column("RLL").max, 4);

        assert_eq!(get_id("FBFBBFFRLR"), 357);
        assert_eq!(get_id("BFFFBBFRRR"), 567);
        assert_eq!(get_id("FFFBBBFRRR"), 119);
        assert_eq!(get_id("BBFFBBFRLL"), 820);
    }

    #[test]
    fn test_3() {
        println!("row {} id {}", row("FFFFFFFRRR").max, get_id("FFFFFFFRRR"));
        println!("row {} id {}", row("BBBBBBBLLL").max, get_id("BBBBBBBLLL"));
    }
}
