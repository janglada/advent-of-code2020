use std::mem::take;

struct Game {
    words: Vec<usize>,
}

impl Iterator for Game {
    type Item = usize;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<usize> {
        let last = self.words.last().unwrap();
        let count = self.words.iter().filter(|&x| x == last).count();

        if count == 1 {
            self.words.push(0);
            Some(0)
        } else {
            let idx_of_last = self.words.len() - 1;
            match self
                .words
                .iter()
                .enumerate()
                .rposition(|(i, &x)| x == *last && i != idx_of_last)
            {
                None => {
                    panic!("{}");
                    Some(0)
                }
                Some(idx) => {
                    let v = idx_of_last - idx;
                    self.words.push(v);
                    Some(v)
                }
            }
        }
    }
}

impl Game {
    fn play_n_rounds(&mut self, rounds: usize) -> usize {
        let takeN = rounds - self.words.len();

        self.nth(takeN - 1).unwrap()
    }
}
pub fn day_fifteen() {
    let mut game = Game {
        words: vec![11, 0, 1, 10, 5, 19],
    };

    println!(" {}", game.play_n_rounds(30000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let mut game = Game {
            words: vec![11, 0, 1, 10, 5, 19],
        };

        println!("4 : {}", game.next().unwrap());
        println!("5 : {}", game.next().unwrap());
        println!("6 : {}", game.next().unwrap());
        println!("7 : {}", game.next().unwrap());
        println!("8 : {}", game.next().unwrap());
        println!("9 : {}", game.next().unwrap());
        println!("10 : {}", game.next().unwrap());
    }

    #[test]
    fn test_1() {
        let mut game = Game {
            words: vec![1, 3, 2],
        };

        assert_eq!(1, game.play_n_rounds(2020));
    }
}
