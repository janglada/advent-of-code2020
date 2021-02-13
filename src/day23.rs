use itertools::Itertools;
use std::cmp;
use std::ops::Index;
use std::thread::current;

struct Game {
    current: i8,
    cups: Vec<i8>,
    substracted_cups: Option<[i8; 3]>,
    destination: Option<i8>,
}

struct IterCups<'a, T: 'a> {
    inner: &'a Vec<T>,
    // And there is a position used to know where you are in your iteration.
    pos: usize,
}

// Now you can just implement the `Iterator` trait on your `IterNewType` struct.
impl<'a, T> Iterator for IterCups<'a, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // We increment the position of our iterator.
        self.pos += 1;
        let p = ((self.pos - 1) % self.inner.len()) as usize;
        // We return the current value pointed by our iterator.
        Some(p)
    }
}

impl Game {
    fn iter<'a>(&'a self) -> IterCups<'a, i8> {
        IterCups {
            inner: &self.cups,
            pos: 0,
        }
    }

    fn play(&mut self) {
        println!();
        println!("--------------------------------------");
        println!("cups {:?}", self.cups);
        println!("current {}", self.current);
        let mut substracted = self.substract();

        let dst = self.find_destination(self.current - 1, substracted.clone());
        self.destination = Some(dst);

        self.insert_substracted(dst, substracted.as_ref());

        let idx = self
            .cups
            .iter()
            .find_position(|&&p| p == self.current)
            .unwrap()
            .0;

        let idx_of_next = self
            .iter()
            .skip_while(|&idx| *self.cups.get(idx).unwrap() != self.current)
            .skip(1)
            .next()
            .unwrap();

        self.current = *self.cups.get(idx_of_next).unwrap();
        println!("cups: {:?}", self.cups);
        println!("pick up : {:?}", substracted);
        println!("destination: {}", dst);
    }

    fn find_destination(&mut self, dst: i8, substracted: Vec<i8>) -> i8 {
        if substracted.contains(&dst) {
            self.find_destination(dst - 1, substracted)
        } else if !self.cups.contains(&dst) {
            let m = cmp::max(
                *(self.cups.iter().max().unwrap()),
                *substracted.iter().max().unwrap(),
            );

            println!("DST  {}  max {}", dst, m);

            self.find_destination(m, substracted)
        } else {
            dst
        }
    }

    fn insert_substracted(&mut self, dst: i8, substracted: &[i8]) {
        let start_pos = self.cups.iter().find_position(|&&v| v == dst).unwrap().0;
        // println!("cups {:?}", self.cups);
        // println!("START POS {} (dst {})", start_pos, dst);
        for (idx, cup) in substracted.iter().enumerate() {
            self.cups.insert(start_pos + idx + 1, *cup);
        }
    }

    fn extract_solution(&self) -> String {
        let mut iter = self.iter();
        self.cups.iter().skip_while(|&&v| {
            if v != 1 {
                iter.next();
                false
            } else {
                true
            }
        });

        let str = self
            .iter()
            .skip_while(|&v| *self.cups.get(v).unwrap() != 1)
            .skip(1)
            .take(self.cups.len() - 1)
            .map(|idx| self.cups.get(idx).unwrap())
            .join("");
        println!("{}", str);
        str
    }

    fn substract(&mut self) -> Vec<i8> {
        //     println!(" before CUPS {:?}", self.cups);
        //
        let to_substract = self
            .iter()
            .skip_while(|idx| *self.cups.get(*idx).unwrap() != self.current)
            .skip(1)
            .take(3)
            .collect::<Vec<usize>>();
        //
        //     // self.cups = self
        //     //     .cups
        //     //     .into_iter()
        //     //     .filter(|&v| !to_substarct.iter().any(|(i, n)| *n == v))
        //     //     .collect();
        //
        let mut substracted_cups = Vec::new();

        for &idx in to_substract.iter() {
            substracted_cups.push(*self.cups.get(idx).unwrap());
        }
        for idx in to_substract.iter().sorted().rev() {
            self.cups.remove(*idx);
        }

        substracted_cups
    }

    fn full_solution(&mut self, n: usize) -> String {
        for i in 0..n {
            println!();
            println!("-- move {} --", i + 1);
            self.play();
        }
        self.extract_solution()
    }
    //
    // fn select_destination(&mut self) {}
}

pub fn run() {
    // 87534692

    let mut game = Game {
        current: 1,
        cups: "469217538"
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i8)
            .collect::<Vec<i8>>(),
        substracted_cups: None,
        destination: None,
    };

    let s = game.full_solution(100);
    println!("{}", s)
}
#[cfg(test)]
mod tests {
    use super::*;
    use itertools::assert_equal;

    #[test]
    fn test_iter() {
        let mut game = Game {
            current: 3,
            cups: "469217538"
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect::<Vec<i8>>(),
            substracted_cups: None,
            destination: None,
        };

        game.iter().take(100).for_each(|x| {
            println!("{}", x);
        })
    }

    #[test]
    fn test_subs() {
        let mut game = Game {
            current: 3,
            cups: "389125467"
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect::<Vec<i8>>(),
            substracted_cups: None,
            destination: None,
        };

        assert_eq!("92658374", game.full_solution(10));
        println!("{:?}", game.cups);
        assert_eq!("67384529", game.full_solution(90));
    }

    fn are_equal(v1: Vec<i8>, v2: Vec<i8>) -> bool {
        let first = v1.get(0).unwrap();
        let len = v1.len();

        let pos = v2.iter().find_position(|&x| x == first).unwrap().0;
        let mut equals = true;
        for (i, v) in v2.iter().cycle().skip(pos).take(len).enumerate() {
            // println!("COMPARE {} {}", v, v1.get(i % len).unwrap());
            if v != v1.get(i % len).unwrap() {
                equals = false;
                break;
            }
        }

        equals
    }

    #[test]
    fn test_steps() {
        let mut game = Game {
            current: 3,
            cups: "389125467"
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect::<Vec<i8>>(),
            substracted_cups: None,
            destination: None,
        };
        assert_eq!(3, game.current);
        assert_eq!(
            true,
            are_equal(vec![3, 8, 9, 1, 2, 5, 4, 6, 7], game.cups.clone())
        );
        assert_eq!("3  8  9  1  2  5  4  6  7", game.cups.iter().join("  "));

        game.play();

        assert_eq!(2, game.destination.unwrap());
        assert_eq!(2, game.current);
        assert_eq!(
            true,
            are_equal(vec![3, 2, 8, 9, 1, 5, 4, 6, 7], game.cups.clone())
        );
        assert_eq!("3  2  8  9  1  5  4  6  7", game.cups.iter().join("  "));

        game.play();
        assert_eq!(5, game.current);
        assert_eq!(7, game.destination.unwrap());
        assert_eq!(
            true,
            are_equal(vec![3, 2, 5, 4, 6, 7, 8, 9, 1], game.cups.clone())
        );
        assert_eq!("3  2  5  4  6  7  8  9  1", game.cups.iter().join("  "));

        game.play();
        assert_eq!(8, game.current);
        assert_eq!(3, game.destination.unwrap());
        assert_eq!(
            true,
            are_equal(vec![7, 2, 5, 8, 9, 1, 3, 4, 6], game.cups.clone())
        );
        // assert_eq!("7  2  5  8  9  1  3  4  6", game.cups.iter().join("  "));

        game.play();
        assert_eq!(7, game.destination.unwrap());
        assert_eq!(
            true,
            are_equal(vec![3, 2, 5, 8, 4, 6, 7, 9, 1], game.cups.clone())
        );
        // assert_eq!("3  2  5  8  4  6  7  9  1", game.cups.iter().join("  "));
    }

    #[test]
    fn test_equals() {
        assert_eq!(
            true,
            are_equal(
                vec![3, 8, 9, 1, 2, 5, 4, 6, 7],
                vec![3, 8, 9, 1, 2, 5, 4, 6, 7]
            )
        );
        assert_eq!(
            true,
            are_equal(
                vec![3, 8, 9, 1, 2, 5, 4, 6, 7],
                vec![8, 9, 1, 2, 5, 4, 6, 7, 3]
            )
        );

        assert_eq!(
            true,
            are_equal(
                vec![3, 8, 9, 1, 2, 5, 4, 6, 7],
                vec![9, 1, 2, 5, 4, 6, 7, 3, 8]
            )
        );

        assert_eq!(
            false,
            are_equal(
                vec![3, 8, 1, 9, 2, 5, 4, 6, 7],
                vec![9, 1, 2, 5, 4, 6, 7, 3, 8]
            )
        );
    }
}
