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

fn main() -> Result<(), Error> {
    let mut v = read(File::open("numbers.txt")?)?;
    // use `vec` for whatever



    let it = v.iter().combinations(2);
    for (i, el1) in it.enumerate() {
        match el1.get(0) {
            Some(v1) => {
                match el1.get(1) {
                    Some(v2) => {
                        if v1.add(*v2) == 2020 {
                            println!("({}, {}, {})", v1, v2, v1.mul(*v2))
                        }

                    }

                    _ => {}
                }
            },
            _ => {}
        }

    }

    Ok(())
}

