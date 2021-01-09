use lazy_static::lazy_static;
use regex::Regex; // 1.3.0

lazy_static! {
    static ref regex_rule: Regex =
        Regex::new(r"(?m)(\b[\w\s]*\b):\s*(\d+-\d+)\sor\s(\d+-\d+)").unwrap();
    static ref regex_range: Regex = Regex::new(r"(?m)(\d+)-(\d+)").unwrap();
}

struct Program {
    rules: Vec<Rule>,
    my_ticket: Vec<u32>,
    nearby_ticket: Vec<Vec<u32>>,
}

impl Program {
    fn scanning_rate(&self) -> u32 {
        let c: u32 = self
            .nearby_ticket
            .iter()
            .map(|ticket| {
                ticket
                    .iter()
                    .filter(|&value| self.rules.iter().all(|r| !r.is_valid_value(*value)))
            })
            .flatten()
            .sum();

        println!("{}", c);
        println!("{}", self.nearby_ticket.len());

        c
    }
}

struct Rule {
    name: String,
    predicate: Vec<Range>,
}

impl Rule {
    fn fromString(text: &str) -> Rule {
        let cap = regex_rule.captures_iter(text).next().unwrap();
        let name: &str = &cap[1];
        let range_1: &str = &cap[2];
        let range_2: &str = &cap[3];

        Rule {
            name: name.to_string(),
            predicate: vec![Range::fromString(range_1), Range::fromString(range_2)],
        }
    }

    fn is_valid_value(&self, value: u32) -> bool {
        self.predicate.iter().any(|p| p.is_within(value))
    }

    fn is_rule_applied(&self, ticket: &Vec<u32>) -> bool {
        ticket.iter().filter(|&&v| self.is_valid_value(v)).count() != 0
    }
}

struct Range {
    min: u32,
    max: u32,
}

impl Range {
    fn is_within(&self, value: u32) -> bool {
        value >= self.min && value <= self.max
    }

    fn fromString(text: &str) -> Range {
        let cap = regex_range.captures_iter(text).next().unwrap();
        let range_1: &str = &cap[1];
        let range_2: &str = &cap[2];
        Range {
            min: range_1.parse::<u32>().ok().unwrap(),
            max: range_2.parse::<u32>().ok().unwrap(),
        }
        // let name: &str = &cap[1];
        // let range_1: &str = &cap[2];
        // let range_2: &str = &cap[3];
    }
}

fn read(txt: &str) -> Program {
    let mut parts = txt.split("\n\n");

    let fields = parts.next().unwrap();

    let rules: Vec<Rule> = fields.split("\n").map(|s| Rule::fromString(s)).collect();
    let mut iter = parts.next().unwrap().split("\n");
    iter.next();
    let my_ticket = parse_ticket(iter.next().unwrap());

    let mut nearby_ticket_iter = parts.next().unwrap().split("\n");
    nearby_ticket_iter.next();
    let nearby_ticket: Vec<Vec<u32>> = nearby_ticket_iter.map(|row| parse_ticket(row)).collect();

    Program {
        rules,
        my_ticket,
        nearby_ticket,
    }
}

fn parse_ticket(str: &str) -> Vec<u32> {
    str.split(",")
        .map(|c| c.parse::<u32>().ok().unwrap())
        .collect()
}

pub fn day_sixteen() {
    let program = read(include_str!("../day16.txt"));

    println!("{}", program.scanning_rate());
}
