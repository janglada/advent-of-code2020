extern crate peg;

use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
pub enum MsgRule {
    Compound(Vec<Vec<usize>>),
    Symbol(char),
}

struct CompiledRule {
    rule: Vec<String>,
}

impl CompiledRule {
    pub fn validate(&self, s: String) -> bool {
        self.rule.iter().any(|rule_str| rule_str.to_string() == s)
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

    pub fn compile_rule(&self, id: &usize) -> CompiledRule {
        CompiledRule {
            rule: self.parse_rule(id),
        }
    }

    pub fn parse_rule(&self, id: &usize) -> Vec<String> {
        match self.rule_dict.get(id).unwrap() {
            MsgRule::Compound(comp) => comp
                .iter()
                .map(|v| v.iter().flat_map(|ci| self.parse_rule(ci)).join(""))
                .collect(),
            MsgRule::Symbol(c) => vec![c.to_string()],
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
    let mut rules_compiler = Rules::new(iter.next().unwrap());
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

        let vec = rules.parse_rule(&18);

        println!("{:?}", vec);
        assert_eq!("bb", vec.get(0).unwrap());
    }

    #[test]
    fn test_complex_validation() {
        let rules = Rules::new(
            r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b""#,
        );

        let compiled = rules.compile_rule(&0);
        println!("{:?}", compiled.rule);

        assert_eq!(true, compiled.validate("aaaabb".to_string()));
        assert_eq!(true, compiled.validate("aaabab".to_string()));
        assert_eq!(true, compiled.validate("abbabb".to_string()));
        assert_eq!(true, compiled.validate("abbbab".to_string()));
        assert_eq!(true, compiled.validate("aabaab".to_string()));
        assert_eq!(true, compiled.validate("aabbbb".to_string()));
        assert_eq!(true, compiled.validate("abaaab".to_string()));
        assert_eq!(true, compiled.validate("ababbb".to_string()));
    }
}
