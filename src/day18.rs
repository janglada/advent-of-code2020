extern crate peg;

peg::parser!( grammar arithmetic() for str {
    rule number() -> i64
        = n:$(['0'..='9']+) { n.parse().unwrap() }

    pub(crate) rule calculate() -> i64 = precedence!{
        x:(@) "+" y:@ { x + y }
        x:(@) "-" y:@ { x - y }
              "-" v:@ { - v }
        x:(@) "*" y:@ { x * y }
        x:(@) "/" y:@ { x / y }
        --
        "(" v:calculate() ")" { v }
        n:number() {n}
    }
});

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn day_eighteen() {
    let total: i64 = include_str!("../day18.txt")
        .split("\n")
        .map(|s| remove_whitespace(s))
        .map(|s| arithmetic::calculate(s.as_str()).unwrap())
        .sum();

    println!("{}", total);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_00() {
        assert_eq!(arithmetic::calculate("2*3+(4*5)"), Ok(26));
        assert_eq!(arithmetic::calculate("5+(8*3+9+3*4*3)"), Ok(437));
        assert_eq!(arithmetic::calculate("5*9*(7*3*3+9*3+(8+6*4))"), Ok(12240));
        assert_eq!(
            arithmetic::calculate("((2+4*9)*(6+9*8+6)+6)+2+4*2"),
            Ok(13632)
        );
    }
}
