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

struct StringCursor<'a> {
    str: &'a str,
    index: i32,
    peek_index: i32,
}

impl StringCursor<'_> {
    fn new(str: &str) -> StringCursor {
        StringCursor {
            index: -1,
            peek_index: -1,
            str,
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.value_at(self.index + 1)
    }

    fn next(&mut self) -> Option<char> {
        self.index += 1;
        self.value_at(self.index)
    }
    fn prev(&mut self) -> Option<char> {
        self.index -= 1;
        self.value_at(self.index)
    }
    fn current(&mut self) -> Option<char> {
        self.value_at(self.index)
    }

    fn value_at(&self, index: i32) -> Option<char> {
        let len = self.str.len() as i32;
        // println!("str len {}, index {}", len);
        if index >= 0 && len > index {
            Some(self.str.as_bytes()[index as usize] as char)
        } else {
            None
        }
    }

    fn reset(&mut self) {
        self.peek_index = self.index;
    }

    //
    //
    fn advance(&mut self) {
        self.index = self.peek_index;
    }

    fn set_index(&mut self, index: i32) {
        self.index = index;
    }

    fn all_chars_validated(&self) -> bool {
        println!("ALL {} {}", self.index, (self.str.len() as i32));
        self.index == (self.str.len() as i32 - 1)
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

    pub fn is_valid(&self, id: &usize, s: &str) -> bool {
        // println!("{}", s);
        let string_cursor = &mut StringCursor::new(s);
        self.validate_rule(self.rule_dict.get(id).unwrap(), string_cursor)
            && string_cursor.all_chars_validated()
    }

    pub fn validate_rule(&self, rule: &MsgRule, char_iter: &mut StringCursor) -> bool {
        match rule {
            MsgRule::Compound(comp) => comp.iter().any(|r| {
                let index = char_iter.index;
                let is_valid = r
                    .iter()
                    .all(|x| self.validate_rule(self.rule_dict.get(x).unwrap(), char_iter));

                println!(
                    "IS VALID {:?} {} {}",
                    r,
                    is_valid,
                    char_iter.value_at(char_iter.peek_index).unwrap_or('-')
                );

                if !is_valid {
                    char_iter.set_index(index)
                }

                is_valid
            }),

            MsgRule::Symbol(c) => {
                let next_char = char_iter.peek();

                match next_char {
                    None => {
                        println!("NO MORE CHARS");
                        panic!()
                    }
                    Some(a) => {
                        println!("  next_char {}, rule   must match  {} {}", a, *c, *c == a);
                        if *c == a {
                            char_iter.next();
                            true
                        } else {
                            char_iter.prev();
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
    let rules = Rules::new(iter.next().unwrap());
    let n = iter
        .next()
        .unwrap()
        .split('\n')
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
        assert_eq!(false, rules.is_valid(&18, "bba"));
    }

    #[test]
    fn test_complex_validation1() {
        let rules = Rules::new(
            r#"0: 1 2
1: "a"
2: 1 3 | 3 1
3: "b""#,
        );

        assert_eq!(true, rules.is_valid(&0, "aab"));
        assert_eq!(true, rules.is_valid(&0, "aba"));
        assert_eq!(false, rules.is_valid(&0, "aaa"));
        assert_eq!(false, rules.is_valid(&0, "bba"));
        assert_eq!(false, rules.is_valid(&0, "abb"));
        assert_eq!(false, rules.is_valid(&0, "baa"));
    }

    #[test]
    fn test_complex_validation1_1() {
        let rules = Rules::new(
            r#"0: 1 2
1: "a"
2: 1 3 | 3 1
3: "b""#,
        );

        assert_eq!(false, rules.is_valid(&0, "aca"));
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
        assert_eq!(false, rules.is_valid(&0, "abbb"));
        assert_eq!(false, rules.is_valid(&0, "abbbaba"));
        assert_eq!(false, rules.is_valid(&0, "abbbabb"));
        assert_eq!(false, rules.is_valid(&0, "bababa"));
        assert_eq!(false, rules.is_valid(&0, "aaabbb"));
        assert_eq!(false, rules.is_valid(&0, "aaaabbb"));
    }
}
