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

struct BitMask {
    mask: [BitMaskBit; 36],
    mask_str: String,
    other_addresses: Vec<String>,
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
            other_addresses: Vec::new(),
        }
    }

    fn maskAddress(&self, mem: MemoryAddress) {
        let s = format!("{:036b}", mem.address);

        let mut addr: String = s
            .chars()
            .zip(self.mask_str.chars())
            .map(|(bit, mask)| match mask {
                '0' => bit,
                '1' => '1',
                'X' => 'X',
                _ => {
                    panic!("found {}", bit)
                }
            })
            .collect();

        let addresses: Vec<String> = addr
            .chars()
            .enumerate()
            .filter_map(|(idx, char)| if char == 'X' { Some(idx) } else { None })
            .for_each(|idx| {})
            .map(|idx| {
                vec![
                    addr.chars()
                        .enumerate()
                        .map(|(i, c)| if i == idx { '0' } else { c })
                        .collect(),
                    addr.chars()
                        .enumerate()
                        .map(|(i, c)| if i == idx { '1' } else { c })
                        .collect(),
                ]
            })
            .flatten()
            .collect();

        let addresses: Vec<String> = addr
            .chars()
            .enumerate()
            .filter_map(|(idx, char)| if char == 'X' { Some(idx) } else { None })
            .map(|idx| {
                vec![
                    addr.chars()
                        .enumerate()
                        .map(|(i, c)| if i == idx { '0' } else { c })
                        .collect(),
                    addr.chars()
                        .enumerate()
                        .map(|(i, c)| if i == idx { '1' } else { c })
                        .collect(),
                ]
            })
            .flatten()
            .collect();

        println!("{:?}", addresses);
    }

    fn maskMemoryValue(&self, mem: MemoryAddress) -> MemoryAddress {
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
            value: mem.value,
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
                let newAddress = mask.maskMemoryValue(mem);

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
        let new_addr = mask.maskMemoryValue(MemoryAddress::fromString("mem[41476] = 11"));
        assert_eq!(new_addr.value, 73);
        let new_addr = mask.maskMemoryValue(MemoryAddress::fromString("mem[41476] = 101"));
        assert_eq!(new_addr.value, 101);
        let new_addr = mask.maskMemoryValue(MemoryAddress::fromString("mem[41476] = 0"));
        assert_eq!(new_addr.value, 64);
    }

    #[test]
    fn test_1() {
        let s = format!("{:b}", 1236);
        println!("{}", s);
    }

    #[test]
    fn test_2() {
        let mask = BitMask::fromString("mask = 000000000000000000000000000000X1001X");
        mask.maskAddress(MemoryAddress::fromString("mem[42] = 100"));
    }
}
