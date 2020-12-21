use itertools::Itertools;
use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0 }
    }
}
#[derive(Debug, Clone)]
pub struct Forest {
    trees: Vec<Vec<bool>>,
    pub depth: usize,
    pub width: usize,
    pub pos: Position,
}

pub struct SlopeBy {
    iter: Forest,
    step_x: usize,
    step_y: usize,
}

impl SlopeBy {
    pub(super) fn new(iter: Forest, step_x: usize, step_y: usize) -> SlopeBy {
        SlopeBy {
            iter,
            step_x,
            step_y,
        }
    }
}

impl Default for Forest {
    fn default() -> Self {
        Forest {
            trees: Vec::new(),
            depth: 0,
            width: 0,
            pos: Default::default(),
        }
    }
}

impl Forest {
    pub fn addRow(&mut self, row: Vec<bool>) {
        self.width = cmp::max(row.len(), self.width);
        self.trees.push(row);
        self.depth = self.trees.len();
    }

    pub fn valueAt(&self, x: usize, y: usize) -> bool {
        match self.trees.get(y) {
            Some(vec) => match vec.get(x % self.width) {
                Some(b) => *b,
                _ => false,
            },
            _ => false,
        }
    }

    fn slope_by(self, step_x: usize, step_y: usize) -> SlopeBy
    where
        Self: Sized,
    {
        SlopeBy::new(self, step_x, step_y)
    }
}

impl Iterator for Forest {
    type Item = bool;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        self.pos.x = self.pos.x + 3;
        self.pos.y = self.pos.y + 1;

        if self.pos.y >= self.depth {
            None
        } else {
            Some(self.valueAt(self.pos.x % self.width, self.pos.y))
        }
    }
}

impl Iterator for SlopeBy {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.pos.x = self.iter.pos.x + self.step_x;
        self.iter.pos.y = self.iter.pos.y + self.step_y;

        if self.iter.pos.y >= self.iter.depth {
            None
        } else {
            Some(
                self.iter
                    .valueAt(self.iter.pos.x % self.iter.width, self.iter.pos.y),
            )
        }
    }
}

pub fn day_three() -> Result<(), Error> {
    let mut forest: Forest = Default::default();
    let br = BufReader::new(File::open("day3.txt")?);

    for (i, row) in br
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.chars().map(|c| c == '#').collect_vec())
        .enumerate()
    {
        // dbg!(row);
        forest.addRow(row)
    }

    //
    // dbg!(forest.filter(|x| *x).count());

    // forest.filter(|x| *x).count()

    let a = forest.clone().slope_by(1, 1).filter(|x| *x).count()
        * forest.clone().slope_by(3, 1).filter(|x| *x).count()
        * forest.clone().slope_by(5, 1).filter(|x| *x).count()
        * forest.clone().slope_by(7, 1).filter(|x| *x).count()
        * forest.clone().slope_by(1, 2).filter(|x| *x).count();

    dbg!(a);

    Ok(())
}
