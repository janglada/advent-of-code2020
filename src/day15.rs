use multimap::MultiMap;
use std::mem::take;

struct Game {
    words: Vec<u32>,
    words2: MultiMap<u32, u32>,
}

impl Iterator for Game {
    type Item = u32;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<u32> {
        let last = self.words.last().unwrap();

        let mut indexes = self.words2.get_vec(last).unwrap();
        // println!("M = {:?} {:?}", self.words2, self.words);
        // println!("L = {:?} ,{}", indexes, indexes.len());
        if indexes.len() == 1 {
            self.words2.insert(0, self.words.len() as u32);
            self.words.push(0);
            Some(0)
        } else {
            let idx =
                indexes.get(indexes.len() - 1).unwrap() - indexes.get(indexes.len() - 2).unwrap();
            self.words2.insert(idx, self.words.len() as u32);
            self.words.push(idx);
            Some(idx)
        }
    }
}

impl Game {
    fn new(starting_words: Vec<u32>) -> Game {
        let mut map: MultiMap<u32, u32> = MultiMap::new();
        starting_words
            .iter()
            .enumerate()
            .for_each(|(i, &v)| map.insert(v, i as u32));

        Game {
            words: starting_words,
            words2: map,
        }
    }

    fn play_n_rounds(&mut self, rounds: u32) -> u32 {
        let takeN = rounds as usize - self.words.len();

        self.nth(takeN - 1).unwrap()
    }
}
pub fn day_fifteen() {
    let mut game = Game::new(vec![11, 0, 1, 10, 5, 19]);

    // println!(" {}", game.play_n_rounds(2020));
    println!(" {}", game.play_n_rounds(30000000));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_00() {
        let mut game = Game::new(vec![0, 3, 6]);

        assert_eq!(0, game.next().unwrap());
        assert_eq!(3, game.next().unwrap());
        assert_eq!(3, game.next().unwrap());
        assert_eq!(1, game.next().unwrap());
        assert_eq!(0, game.next().unwrap());
        assert_eq!(4, game.next().unwrap());
        assert_eq!(0, game.next().unwrap());
    }
    #[test]
    fn test_0() {
        let mut game = Game::new(vec![11, 0, 1, 10, 5, 19]);

        assert_eq!(870, game.play_n_rounds(2020));
    }

    #[test]
    fn test_1() {
        let mut game = Game::new(vec![1, 3, 2]);
        assert_eq!(1, game.play_n_rounds(2020));
    }
}
