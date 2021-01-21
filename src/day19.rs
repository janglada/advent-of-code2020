extern crate peg;

use itertools::__std_iter::Peekable;
use itertools::{Itertools, MultiPeek};
use std::collections::HashMap;
use std::slice::Iter;
use std::str::Chars;

struct Multizip<T>(Vec<T>);

impl<T> Iterator for Multizip<T>
where
    T: Iterator,
{
    type Item = Vec<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter_mut().map(Iterator::next).collect()
    }
}

#[derive(Debug)]
pub enum MsgRule {
    Compound(Vec<Vec<usize>>),
    Symbol(char),
}

struct CompiledRule {
    rule: Vec<Vec<char>>,
}

impl CompiledRule {
    pub fn validate(&self, s: String) -> bool {
        false
        // self.rule.iter().any(|rule_str| rule_str.to_string() == s)
    }
}

struct Rules {
    rule_dict: HashMap<usize, MsgRule>,
}

impl Rules {
    pub fn new(s: &str) -> Rules {
        let mut rules_compiler = Rules {
            rule_dict: HashMap::new(),
        };
        s.split('\n')
            .map(|s| rules_parser::line(s).ok().unwrap())
            .for_each(|(id, msg)| {
                rules_compiler.rule_dict.insert(id, msg);
            });

        rules_compiler
    }

    // pub fn compile(&self, id: &usize) -> CompiledRule {
    //     CompiledRule {
    //         rule: self.compile_rule(id),
    //     }
    // }
    // pub fn compile_rule(&self, id: &usize) -> Vec<Vec<char>> {
    //     let mut v: Vec<char> = Vec::new();
    //     match self.rule_dict.get(id).unwrap() {
    //         MsgRule::Compound(_) => {}
    //         MsgRule::Symbol(c) => {
    //             v.push(*c);
    //         }
    //     }
    //     v
    // }
    // pub fn validate(&self, id: &usize, s: String) -> bool {
    //     let mut char_iter = s.chars();
    //     match self.rule_dict.get(id).unwrap() {
    //         MsgRule::Compound(comp) => comp
    //             .iter()
    //             .map(|v| v.iter().map(|ci| self.parse_rule(ci).join("").to_string()))
    //             .collect(),
    //         MsgRule::Symbol(c) => char_iter.next().unwrap() == 'c',
    //     }
    // }

    pub fn is_valid(&self, id: &usize, s: &str) -> bool {
        // println!("{}", s);
        self.validate_rule(self.rule_dict.get(id).unwrap(), &mut s.chars().peekable())
    }

    pub fn validate_rule(&self, rule: &MsgRule, char_iter: &mut Peekable<Chars<'_>>) -> bool {
        match rule {
            MsgRule::Compound(comp) => Multizip(comp.iter().map(|v| v.iter()).collect()).all(|x| {
                x.iter().any(|r| {
                    println!("checking rule to {}", r);
                    self.validate_rule(self.rule_dict.get(r).unwrap(), char_iter)
                })
            }),
            MsgRule::Symbol(c) => {
                let next_char = char_iter.peek();

                match next_char {
                    None => false,
                    Some(a) => {
                        println!("  next_char {}, rule   must match  {}", *a, *c);
                        if *c == *a {
                            char_iter.next();
                            true
                        } else {
                            false
                        }
                    }
                }
                // println!("  {} matches {} {}", next_char, c, next_char == c);
            }
        }
    }
}

peg::parser! {
  grammar rules_parser() for str {
    rule number() -> usize
      = n:$(['0'..='9']+) { n.parse().unwrap() }

    rule char() -> char
      = s:$(['a'|'b']) { s.chars().next().unwrap() }


    rule rule_sequence() -> Vec<usize>
      = r:number()  ** " " {r}

    rule rule_sequences() -> Vec<Vec<usize>>
      = r:rule_sequence()  ** " | " {r}

    rule id() -> usize
      = n:$(['0'..='9']+) { n.parse().unwrap() }


    rule multirule() -> (usize,MsgRule)
        = id:number() ":" " " r:rule_sequences()  {
            (id,MsgRule::Compound(r))
        }


    pub rule singlecharrule() -> (usize,MsgRule)
        = id:number() ":" " " "\"" r:char()  "\"" {
             (id, MsgRule::Symbol(r))
        }


    pub rule line() ->(usize,MsgRule)
      = singlecharrule() / multirule()

    }
}

pub fn day_nineteen() {
    let mut iter = include_str!("../day19.txt").split("\n\n");
    let mut rules = Rules::new(iter.next().unwrap());
    let n = iter
        .next()
        .unwrap()
        .split("\n")
        .filter(|s| rules.is_valid(&0, s))
        .count();

    println!("{}", n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compund_rule() {
        let r = rules_parser::line("18: 48 40").ok().unwrap();

        assert_eq!(r.0, 18);
        let rule: MsgRule = r.1;
        match rule {
            MsgRule::Compound(c) => {
                assert_eq!(c.len(), 1);
                assert_eq!(c.get(0).unwrap().len(), 2);
                let mut iter = c.iter().next().unwrap().iter();
                assert_eq!(iter.next().unwrap(), &48);
                assert_eq!(iter.next().unwrap(), &40);
            }
            MsgRule::Symbol(_) => {
                panic!("Rule is compound");
            }
        }
    }
    #[test]
    fn test_compund_rule2() {
        let r = rules_parser::line("31: 48 133 | 41 127").ok().unwrap();

        assert_eq!(r.0, 31);
        let rule: MsgRule = r.1;
        match rule {
            MsgRule::Compound(c) => {
                assert_eq!(c.len(), 2);
                let mut compound = c.iter();
                let mut cond1 = compound.next();
                assert_eq!(cond1.unwrap().len(), 2);
                let mut iter = cond1.unwrap().iter();
                assert_eq!(iter.next().unwrap(), &48);
                assert_eq!(iter.next().unwrap(), &133);

                let mut cond1 = compound.next();
                assert_eq!(cond1.unwrap().len(), 2);
                let mut iter = cond1.unwrap().iter();
                assert_eq!(iter.next().unwrap(), &41);
                assert_eq!(iter.next().unwrap(), &127);
            }
            MsgRule::Symbol(_) => {
                panic!("Rule is compound");
            }
        }
    }
    #[test]
    fn test_single_rule1() {
        let r = rules_parser::singlecharrule("41: \"a\"").ok().unwrap();

        assert_eq!(r.0, 41);
        let rule: MsgRule = r.1;
        match rule {
            MsgRule::Compound(c) => {
                panic!("Rule is single char");
            }
            MsgRule::Symbol(c) => {
                assert_eq!(c, 'a');
            }
        }
    }
    #[test]
    fn test_single_rule2() {
        let r = rules_parser::line("41: \"a\"").ok().unwrap();

        assert_eq!(r.0, 41);
        let rule: MsgRule = r.1;
        match rule {
            MsgRule::Compound(c) => {
                panic!("Rule is single char");
            }
            MsgRule::Symbol(c) => {
                assert_eq!(c, 'a');
            }
        }
    }

    #[test]
    fn test_simple_validation() {
        let rules = Rules::new("18: 48 48\n48: \"b\"");

        assert_eq!(true, rules.is_valid(&18, "bb"));
        assert_eq!(false, rules.is_valid(&18, "ab"));
    }

    #[test]
    fn test_complex_validation1() {
        let rules = Rules::new(
            r#"0: 1 2
1: "a"
2: 1 3 | 3 1
3: "b""#,
        );

        // assert_eq!(true, rules.is_valid(&0, "aab"));
        // assert_eq!(true, rules.is_valid(&0, "aba"));
        // assert_eq!(false, rules.is_valid(&0, "aaa"));
        // assert_eq!(false, rules.is_valid(&0, "bba"));
        assert_eq!(false, rules.is_valid(&0, "abb"));
        // assert_eq!(false, rules.is_valid(&0, "baa"));
    }

    #[test]
    fn test_complex_validation2() {
        let rules = Rules::new(
            r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b""#,
        );

        assert_eq!(true, rules.is_valid(&0, "ababbb"));
        assert_eq!(true, rules.is_valid(&0, "abbbab"));
        assert_eq!(false, rules.is_valid(&0, "bababa"));
        assert_eq!(false, rules.is_valid(&0, "aaabbb"));
        assert_eq!(false, rules.is_valid(&0, "aaaabbb"));
    }
}
