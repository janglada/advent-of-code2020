use itertools::Itertools;
use regex::Regex;
use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::str::FromStr;

use lazy_static::lazy_static; // 1.3.0

lazy_static! {
    static ref byr_present: Regex = Regex::new(r"\bbyr\s*:").unwrap();
    static ref iyr_present: Regex = Regex::new(r"\biyr\s*:").unwrap();
    static ref eyr_present: Regex = Regex::new(r"\beyr\s*:").unwrap();
    static ref hgt_present: Regex = Regex::new(r"\bhgt\s*:").unwrap();
    static ref hcl_present: Regex = Regex::new(r"\bhcl\s*:").unwrap();
    static ref ecl_present: Regex = Regex::new(r"\becl\s*:").unwrap();
    static ref pid_present: Regex = Regex::new(r"\bpid\s*:").unwrap();
    static ref byr: Regex = Regex::new(r"\bbyr\s*:\s*(\d{4})\b").unwrap();
    static ref iyr: Regex = Regex::new(r"\biyr\s*:\s*(\d{4})\b").unwrap();
    static ref eyr: Regex = Regex::new(r"\beyr\s*:\s*(\d{4})\b").unwrap();
    static ref hgt: Regex = Regex::new(r"\bhgt\s*:\s*(\d+)(in|cm)\b").unwrap();
    static ref hcl: Regex = Regex::new(r"\bhcl\s*:\s*(#[0-9a-f]{6})\b").unwrap();
    static ref ecl: Regex = Regex::new(r"\becl\s*:\s*(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
    static ref pid: Regex = Regex::new(r"\bpid\s*:\s*(\d{9})\b").unwrap();
}

pub fn validate_byr(text: &String) -> bool {
    byr.is_match(text)
        && match byr.captures_iter(text).next() {
            Some(cap) => {
                let year: u32 = FromStr::from_str(&cap[1]).unwrap();
                year >= 1920 && year <= 2002
            }
            None => {
                println!(" byr false {}", text);
                false
            }
        }
}

pub fn validate_iyr(text: &String) -> bool {
    iyr.is_match(text)
        && match iyr.captures_iter(text).next() {
            Some(cap) => {
                let year: u32 = FromStr::from_str(&cap[1]).unwrap();
                year >= 2010 && year <= 2020
            }
            None => {
                println!(" iyr false {}", text);
                false
            }
        }
}

pub fn validate_eyr(text: &String) -> bool {
    eyr.is_match(text)
        && match eyr.captures_iter(text).next() {
            Some(cap) => {
                let year: u32 = FromStr::from_str(&cap[1]).unwrap();
                year >= 2020 && year <= 2030
            }
            None => {
                println!(" eyr false {}", text);
                false
            }
        }
}

pub fn validate_hgt(text: &String) -> bool {
    hgt.is_match(text)
        && match hgt.captures_iter(text).next() {
            Some(cap) => {
                let height: i32 = FromStr::from_str(&cap[1]).unwrap();
                let unit = &cap[2];
                match unit {
                    "in" => height >= 59 && height <= 76,
                    "cm" => height >= 150 && height <= 193,
                    _ => false,
                }
            }
            None => {
                println!(" hgt false {}", text);
                false
            }
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

pub fn validate_passport1(text: &String) -> bool {
    hcl_present.is_match(text)
        && pid_present.is_match(text)
        && ecl_present.is_match(text)
        && hgt_present.is_match(text)
        && eyr_present.is_match(text)
        && iyr_present.is_match(text)
        && byr_present.is_match(text)
}

pub fn validate_passport2(text: &String) -> bool {
    validate_byr(text)
        && validate_ecl(text)
        && validate_eyr(text)
        && validate_hcl(text)
        && validate_hgt(text)
        && validate_ecl(text)
        && validate_pid(text)
}

pub fn day_four() -> Result<(), Error> {
    let count = include_str!("../day4.txt")
        .split("\n\n")
        // .filter(|text| validate_passport1(&text.to_string()))
        .filter(|text| validate_passport2(&text.to_string()))
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
        assert_eq!(validate_byr(&"byr: 1920".to_owned()), true);
        assert_eq!(validate_byr(&"byr: 2003".to_owned()), false);
        assert_eq!(validate_byr(&"byr: 1915".to_owned()), false);
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
        assert_eq!(validate_ecl(&"ecl: amb".to_owned()), true);
        assert_eq!(validate_ecl(&"ecl: blu".to_owned()), true);
        assert_eq!(validate_ecl(&"ecl: brn".to_owned()), true);
        assert_eq!(validate_ecl(&"ecl: gry".to_owned()), true);
        assert_eq!(validate_ecl(&"ecl: grn".to_owned()), true);
        assert_eq!(validate_ecl(&"ecl: hzl".to_owned()), true);
        assert_eq!(validate_ecl(&"ecl: oth".to_owned()), true);
        assert_eq!(validate_ecl(&"ecl: otha".to_owned()), false);
        assert_eq!(validate_ecl(&"ecl: wat".to_owned()), false);
    }

    #[test]
    fn test_pid() {
        assert_eq!(validate_pid(&"pid: 000000001".to_owned()), true);
        assert_eq!(validate_pid(&"pid: 0123456789".to_owned()), false);
    }

    #[test]
    fn test_passport() {
        assert_eq!(
            validate_passport2(
                &"eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"
                    .to_owned()
            ),
            false
        );

        assert_eq!(
            validate_passport2(
                &"iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946"
                    .to_owned()
            ),
            false
        );

        assert_eq!(
            validate_passport2(
                &"hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"
                    .to_owned()
            ),
            false
        );

        assert_eq!(
            validate_passport2(
                &"hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007".to_owned()
            ),
            false
        );

        assert_eq!(
            validate_passport2(
                &"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f".to_owned()
            ),
            true
        );

        assert_eq!(
            validate_passport2(
                &"eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"
                    .to_owned()
            ),
            true
        );

        assert_eq!(
            validate_passport2(
                &"hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022"
                    .to_owned()
            ),
            true
        );

        assert_eq!(
            validate_passport2(
                &"iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
                    .to_owned()
            ),
            true
        );
    }
}
