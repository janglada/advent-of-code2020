use itertools::Itertools;
use multimap::MultiMap;
use regex::{Captures, Regex};
use std::fmt;

use lazy_static::lazy_static; // 1.3.0

type BagSpec<'a> = (&'a str, &'a str);
type Rules<'a> = MultiMap<BagSpec<'a>, (usize, BagSpec<'a>)>;
lazy_static! {
    static ref shiny_gold: Regex = Regex::new(r"(?m)(\w+\s\w+)\sbags\s*contain\s*(\d\s*\w+\s\w+\sbags?,?\s*)*(\d\s*(shiny gold)\sbags?,?\s*)+(\d\s*\w+\s\w+\sbags?,?\s*)").unwrap();
    static ref RULE: Regex = Regex::new(r"(?m)(\w+)\s(\w+)\sbags\s*contain\s*((\d)\s*(\w+\s\w+)\sbags?,?\s*\.?)+").unwrap();
}

fn parse_rules(input: &str) -> Rules<'_> {
    let mut rules: Rules = Default::default();

    peg::parser! {
        pub(crate) grammar parser() for str {
            pub(crate) rule root(r: &mut Rules<'input>)
                = (line(r) "." whitespace()*)* ![_]

            rule line(r: &mut Rules<'input>)
                = spec:bag_spec() " contain " rules:rules() {
                    if let Some(rules) = rules {
                        for rule in rules {
                            r.insert(spec, rule)
                        }
                    }
                }

            rule bag_spec() -> BagSpec<'input>
                = adjective:name() " " color:name() " bag" "s"? { (adjective, color) }

            rule rules() -> Option<Vec<(usize, BagSpec<'input>)>>
                = rules:rule1()+ { Some(rules) }
                / "no other bags" { None }

            /// Rule followed by an optional comma and space
            rule rule1() -> (usize, BagSpec<'input>)
                = r:rule0() ", "? { r }

            /// A single rule
            rule rule0() -> (usize, BagSpec<'input>)
                = quantity:number() " " spec:bag_spec() { (quantity, spec) }

            rule number() -> usize
                = e:$(['0'..='9']+) { e.parse().unwrap() }

            /// A sequence of non-whitespace characters
            rule name() -> &'input str
                = $((!whitespace()[_])*)

            /// Spaces, tabs, CR and LF
            rule whitespace()
                = [' ' | '\t' | '\r' | '\n']
        }
    }

    parser::root(input, &mut rules).unwrap();
    rules
}
fn extract_color(text: &str) -> Result<String, &'static str> {
    match shiny_gold.captures_iter(text).next() {
        Some(cap) => {
            // println!(" cap[2] {}", cap[1].to_owned());
            Ok(cap[1].to_owned())
        }
        _ => Err("color not found"),
    }
}

fn parse_rules2(input: &str) -> Rules<'_> {
    let mut rules: Rules = Default::default();
    input
        .split("\n")
        .map(|row| {
            println!(" --------------------  row {}", row);

            RULE.captures_iter(row).for_each(|cap| {
                println!("     cap {:?}", cap);

                println!(
                    "      colors {} {} --> {} {} ",
                    &cap[1], &cap[2], &cap[4], &cap[5],
                );
            });

            rules.insert(("light", "red"), (1, ("bright", "white")));
        })
        .for_each(|c| {});

    rules
}

struct FormattedRules<'a>(Rules<'a>);

impl fmt::Display for FormattedRules<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (k, vv) in &self.0 {
            write!(f, "{} {} bags can contain ", k.0, k.1)?;
            if vv.is_empty() {
                write!(f, "no other bags")?;
            } else {
                for (i, v) in vv.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(
                        f,
                        "{} {} {} {}",
                        v.0,
                        v.1 .0,
                        v.1 .1,
                        if v.0 == 1 { "bag" } else { "bags" }
                    )?;
                }
            }
            writeln!(f, ".")?;
        }
        Ok(())
    }
}

fn subgraph_contains(graph: &Rules<'_>, root: &(&str, &str), needle: &(&str, &str)) -> bool {
    graph
        .get_vec(root)
        .unwrap_or(&Default::default())
        .iter()
        .any(|(_, neighbor)| neighbor == needle || subgraph_contains(graph, neighbor, needle))
}

pub fn day_seven() {
    let rules = parse_rules(include_str!("../day7.txt"));
    let needle = &("shiny", "gold");
    let colors_that_contain_shiny_gold: Vec<_> = rules
        .keys()
        // shiny gold bags are already shiny god, we're not interested
        // in what they can contain (as per the example)
        .filter(|&k| k != needle)
        .filter(|&k| subgraph_contains(&rules, k, needle))
        .collect();
    println!(
        "{:?} {}",
        colors_that_contain_shiny_gold,
        colors_that_contain_shiny_gold.len()
    );
    // parse_rules(include_str!("../day7.txt"));

    // let count: Vec<String> = include_str!("../day7.txt")
    //     .split("\n")
    //     .map(|row| extract_color(row))
    //     .filter(Result::is_ok)
    //     .map(|r| r.unwrap())
    //     .inspect(|c| {
    //         println!("  {}", c);
    //     })
    //     .sorted()
    //     .dedup()
    //     .collect();

    //println!(" colors {:?}", count);
}
