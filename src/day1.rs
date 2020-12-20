

use std::fs::File;
use std::path::Path;
use itertools::Itertools;

use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::ops::{Add, Mul};

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

pub fn day_one() -> Result<(), Error> {
    let mut v = read(File::open("day1.txt")?)?;
    // use `vec` for whatever

    let it = v.iter().combinations(3);
    for (i, el1) in it.enumerate() {
        if  el1.iter().map(|x| **x).sum::<i64>() == 2020 {
            println!("({})", el1.iter().map(|x| **x).product::<i64>())
        }
    }

    Ok(())
}

