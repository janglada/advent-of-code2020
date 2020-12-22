use itertools::Itertools;
use regex::Regex;
use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::str::FromStr;

use lazy_static::lazy_static; // 1.3.0

lazy_static! {
    static ref byr: Regex = Regex::new(r"byr\s*:\s*(\d{4})\b").unwrap();
    static ref iyr: Regex = Regex::new(r"iyr\s*:\s*(\d{4})\b").unwrap();
    static ref eyr: Regex = Regex::new(r"eyr\s*:\s*(\d{4})\b").unwrap();
    static ref hgt: Regex = Regex::new(r"hgt\s*:\s*(\d+)(\w+)\b").unwrap();
    static ref hcl: Regex = Regex::new(r"hcl\s*:\s*(#[0-9a-f]{6})\b").unwrap();
    static ref ecl: Regex = Regex::new(r"ecl\s*:\s*(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
    static ref pid: Regex = Regex::new(r"pid\s*:\s*([0-9]{9})\b").unwrap();
}

pub fn validate_byr(text: &String) -> bool {
    byr.is_match(text)
        && match byr.captures_iter(text).next() {
            Some(cap) => {
                let year: u32 = FromStr::from_str(&cap[1]).unwrap();
                year >= 1920 && year <= 2002
            }
            None => false,
        }
}

pub fn validate_iyr(text: &String) -> bool {
    iyr.is_match(text)
        && match iyr.captures_iter(text).next() {
            Some(cap) => {
                let year: u32 = FromStr::from_str(&cap[1]).unwrap();
                year >= 2010 && year <= 2020
            }
            None => false,
        }
}

pub fn validate_eyr(text: &String) -> bool {
    eyr.is_match(text)
        && match eyr.captures_iter(text).next() {
            Some(cap) => {
                let year: u32 = FromStr::from_str(&cap[1]).unwrap();
                year >= 2020 && year <= 2030
            }
            None => false,
        }
}
pub fn validate_hgt(text: &String) -> bool {
    hgt.is_match(text)
        && match hgt.captures_iter(text).next() {
            Some(cap) => {
                let height: i32 = FromStr::from_str(&cap[1]).unwrap();
                let unit = &cap[2];
                println!("-{}{}-", height, unit);
                match unit {
                    "in" => height >= 59 && height <= 76,
                    "cm" => height >= 150 && height <= 193,
                    _ => false,
                }
            }
            None => false,
        }
}

pub fn validate_hcl(text: &String) -> bool {
    hcl.is_match(text)
}

pub fn validate_ecl(text: &String) -> bool {
    ecl.is_match(text)
}

pub fn validate_pid(text: &String) -> bool {
    pid.is_match(text)
}

pub fn day_four() -> Result<(), Error> {
    let br = BufReader::new(File::open("day4.txt")?);

    let s = br.lines().group_by(|l| match l {
        Ok(lo) => (*lo).len() == 0,
        Err(e) => false,
    });

    let mut data_grouped: Vec<String> = Vec::new();

    for (key, group) in &s {
        let v: Vec<String> = group.flatten().collect();
        let str = v.join(" ");
        //println!(">> {}", str);
        if !key {
            data_grouped.push(str)
        }
    }

    let count = data_grouped
        .iter()
        .filter(|text| {
            validate_byr(text)
                && validate_ecl(text)
                && validate_eyr(text)
                && validate_hcl(text)
                && validate_hgt(text)
                && validate_ecl(text)
                && validate_pid(text)
        })
        .count();

    dbg!(count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byr() {
        assert_eq!(validate_byr(&"byr: 2002".to_owned()), true);
        assert_eq!(validate_byr(&"byr: 2003".to_owned()), false);
    }

    #[test]
    fn test_hgt() {
        assert_eq!(validate_hgt(&"hgt: 60in".to_owned()), true);
        assert_eq!(validate_hgt(&"hgt: 190cm".to_owned()), true);
        assert_eq!(validate_hgt(&"hgt: 190in".to_owned()), false);
        assert_eq!(validate_hgt(&"hgt: 190".to_owned()), false);
    }

    #[test]
    fn test_hcl() {
        assert_eq!(validate_hcl(&"hcl: #123abc".to_owned()), true);
        assert_eq!(validate_hcl(&"hcl: #123abz".to_owned()), false);
        assert_eq!(validate_hcl(&"hcl: 123abc".to_owned()), false);
    }

    #[test]
    fn test_ecl() {
        assert_eq!(validate_ecl(&"ecl: brn".to_owned()), true);
        assert_eq!(validate_ecl(&"ecl: wat".to_owned()), false);
    }

    #[test]
    fn test_pid() {
        assert_eq!(validate_pid(&"pid: 000000001".to_owned()), true);
        assert_eq!(validate_pid(&"pid: 0123456789".to_owned()), false);
    }
}
