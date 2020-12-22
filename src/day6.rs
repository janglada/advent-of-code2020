use itertools::Itertools;
use std::collections::BTreeMap;
use std::io::Error;

pub fn day_six() -> Result<(), Error> {
    // first star
    let count: usize = include_str!("../day6.txt")
        .split("\n\n")
        .map(|line| {
            line.chars()
                .filter(|c| !c.is_whitespace())
                .sorted()
                .dedup()
                .count()
        })
        //.for_each(|c| println!("{}", c))
        .sum();

    println!(" SUM {}", count);

    let count: usize = include_str!("../day6.txt")
        .split("\n\n")
        .map(|line| count_answers(line))
        //.for_each(|c| println!("{}", c))
        .sum();
    println!(" ** {}", count);
    Ok(())
}

fn count_answers(question: &str) -> usize {
    let num_person = question.matches('\n').count() + 1;

    let mut count = BTreeMap::new();

    for c in question.chars().filter(|c| !c.is_whitespace()) {
        *count.entry(c).or_insert(0) += 1;
    }

    count.iter().filter(|&(c, n)| *n == num_person).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(count_answers("abc"), 3);
        assert_eq!(count_answers("a\nb\nc"), 0);
        assert_eq!(count_answers("ab\nac"), 1);
        assert_eq!(count_answers("a\na\na\na"), 1);
        assert_eq!(count_answers("b"), 1);
    }
}
