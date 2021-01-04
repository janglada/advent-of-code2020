use itertools::Itertools;
use lazy_static::lazy_static; // 1.3.0
use regex::Regex;
use std::collections::HashMap;
use std::convert::TryInto;
use std::ops::BitAnd;
use std::str::FromStr;

lazy_static! {
    static ref mask_regex: Regex = Regex::new(r"(?m)mask\s*=\s*([0X1]{36})").unwrap();
    static ref mem_regex: Regex = Regex::new(r"(?m)mem\[(\d+)\]\s=\s(\d+)").unwrap();
}

enum BitMaskBit {
    X,
    One,
    Zero,
}
//
// impl BitAnd<i8> for BitMaskBit {
//     type Output = i8;
//
//     // rhs is the "right-hand side" of the expression `a & b`
//     fn bitand(self, rhs: i8) -> i8 {
//         match self {
//             BitMaskBit::X => rhs,
//             BitMaskBit::One => 1 as i8,
//             BitMaskBit::Zero => 0 as i8,
//         }
//     }
// }

struct BitMask {
    mask: [BitMaskBit; 36],
    mask_str: String,
}

impl BitMask {
    fn fromString(input: &str) -> BitMask {
        let cap = mask_regex.captures_iter(input).next().unwrap();

        let mask_str: &str = &cap[1];
        let bitmak_vec: Vec<BitMaskBit> = mask_str
            .chars()
            .map(|c| match c {
                'X' => BitMaskBit::X,
                '0' => BitMaskBit::One,
                '1' => BitMaskBit::Zero,
                _ => {
                    println!("FOUND {}", c);
                    panic!();
                    BitMaskBit::One
                }
            })
            .collect::<Vec<BitMaskBit>>();

        BitMask {
            mask: bitmak_vec.try_into().unwrap_or_else(|v: Vec<BitMaskBit>| {
                panic!("Expected a Vec of length {} but it was {}", 36, v.len())
            }),
            mask_str: mask_str.parse().unwrap(),
        }
    }

    fn maskAddress(&self, mem: MemoryAddress) -> MemoryAddress {
        let set_zeros = u64::from_str_radix(
            self.mask_str
                .chars()
                .map(|v| match v {
                    'X' => "1",
                    '1' => "1",
                    '0' => "0",
                    _ => {
                        panic!()
                    }
                })
                .collect::<String>()
                .as_str(),
            2,
        )
        .unwrap();
        let set_ones = u64::from_str_radix(
            self.mask_str
                .chars()
                .map(|v| match v {
                    'X' => "0",
                    '1' => "1",
                    '0' => "0",
                    _ => {
                        panic!()
                    }
                })
                .collect::<String>()
                .as_str(),
            2,
        )
        .unwrap();

        MemoryAddress {
            address: mem.address,
            value: (mem.value & set_zeros) | set_ones,
        }
    }
}

struct Memory(HashMap<usize, usize>);

struct MemoryAddress {
    address: u64,
    value: u64,
}
impl MemoryAddress {
    fn fromString(input: &str) -> MemoryAddress {
        let cap = mem_regex.captures_iter(input).next().unwrap();

        let address = &cap[1].parse::<u64>().ok().unwrap();
        let value = &cap[2].parse::<u64>().ok().unwrap();

        MemoryAddress {
            address: *address,
            value: *value,
        }
    }
}
// impl BitMask {
//     fn mask(n: i64) -> i64 {}
// }
// impl BitAnd<i64> for BitMask {
//     type Output = i64;
//
//     // rhs is the "right-hand side" of the expression `a & b`
//     fn bitand(self, rhs: i64) -> i64 {
//         Self(self.0 & rhs.0)
//     }
// }

pub fn day_fourteen() {
    let mut computer_mem = HashMap::new();

    for mut chunk in &include_str!("../day14.txt")
        .split("\n")
        .into_iter()
        .group_by(|row| mask_regex.is_match(*row))
        .into_iter()
        .chunks(2)
    {
        // let (_, mask)   = ;

        let mask = BitMask::fromString(chunk.next().unwrap().1.next().unwrap());

        chunk
            .next()
            .unwrap()
            .1
            .map(|mem_str| MemoryAddress::fromString(mem_str))
            .for_each(|mem: MemoryAddress| {
                let prev = mem.value;
                let newAddress = mask.maskAddress(mem);

                computer_mem.insert(newAddress.address, newAddress.value);

                println!("{} --> {} ", prev, newAddress.value);
            });

        // println!("{} ", bitmask);
        // chunk.for_each(|(isMask, mut groups)| println!("{} {}", isMask, groups.next().unwrap()))
    }

    let response: u64 = computer_mem.values().sum();
    println!("{}", response);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let mask = BitMask::fromString("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        let new_addr = mask.maskAddress(MemoryAddress::fromString("mem[41476] = 11"));
        assert_eq!(new_addr.value, 73);
        let new_addr = mask.maskAddress(MemoryAddress::fromString("mem[41476] = 101"));
        assert_eq!(new_addr.value, 101);
        let new_addr = mask.maskAddress(MemoryAddress::fromString("mem[41476] = 0"));
        assert_eq!(new_addr.value, 64);
    }
}
