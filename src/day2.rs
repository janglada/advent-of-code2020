use std::fs::File;
use std::path::Path;
use itertools::Itertools;


use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::ops::{Add, Mul};
use regex::{Regex, Captures};


pub fn day_two() -> Result<(), Error> {


    // let (a, b) = include_str!("day2.txt")
    //     .split('\n')
    //     .map(str::parse::<i64>)
    //     .collect::<Result<Vec<_>, _>>()?
    //     .into_iter()


    let re = Regex::new(r"^(\d+)-(\d+)\s*(\w):\s(\w+)$").unwrap();



    // use `vec` for whatever
    let br = BufReader::new(File::open("day2.txt")?);
    let rows = br.lines()
        .map(|l| l.unwrap())
        .filter(|row| {
            let mut m = re.captures_iter(&*row);
            match m.next() {
                Some(cap) =>{
                    let min = (&cap[1]).parse::<usize>().unwrap();
                    let max = (&cap[2]).parse::<usize>().unwrap();
                    let letter =&cap[3];
                    let text =&cap[4];
                    let count = text.matches(letter).count();
                    count >= min && count <= max
                }

                _ => {false}
            }
        }).count();

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

