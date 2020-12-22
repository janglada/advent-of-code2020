use itertools::Itertools;
use std::fs::File;
use std::path::Path;

use regex::{Captures, Regex};
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::ops::{Add, Mul};

pub fn day_two() -> Result<(), Error> {
    let re = Regex::new(r"^(\d+)-(\d+)\s*(\w):\s(\w+)$").unwrap();

    // use `vec` for whatever
    let br = BufReader::new(File::open("day2.txt")?);
    let rows = br
        .lines()
        .map(|l| l.unwrap())
        .filter(|row| {
            let mut m = re.captures_iter(&*row);
            match m.next() {
                Some(cap) => {
                    let min = (&cap[1]).parse::<usize>().unwrap();
                    let max = (&cap[2]).parse::<usize>().unwrap();
                    let letter = &cap[3];
                    let text = &cap[4];

                    // second
                    let c1 = text.chars().nth(min - 1).unwrap();
                    let c2 = text.chars().nth(max - 1).unwrap();
                    (c1 == letter.chars().next().unwrap()) ^ (c2 == letter.chars().next().unwrap())

                    // first problem
                    // let count = text.matches(letter).count();
                    //count >= min && count <= max
                }
                _ => false,
            }
        })
        .count();

    dbg!(rows);

    // for (i, row) in rows.enumerate() {
    //
    //        // println!("({})", el)
    //     for cap in re.captures_iter(&*row) {
    //         let min = (&cap[1]).parse::<usize>().unwrap();
    //         let max = (&cap[2]).parse::<usize>().unwrap();
    //         let letter =&cap[3];
    //         let text =&cap[4];
    //
    //         let count = text.matches(letter).count();
    //         if count >= min && count <= max {
    //
    //         }
    //
    //         println!(": {} Day: {} Year: {} {} {}", &cap[1], &cap[2], &cap[3], &cap[4], count);
    //     }
    // }

    Ok(())
}
