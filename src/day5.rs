use std::collections::VecDeque;

use crate::intcode::Intcode;

pub struct Puzzle<'a> {
    s: &'a str,
}

impl<'a> Puzzle<'a> {
    pub fn new(s: &'a str) -> Self {
        Puzzle { s }
    }

    pub fn solve(&self) -> i64 {
        let mut intcode = Intcode::from(self.s);
        intcode.set_input(VecDeque::from([1]));
        intcode.run();
        let output = intcode.take_output();
        let mut iter = output.iter();

        for _ in 0..output.len() - 1 {
            assert_eq!(*iter.next().unwrap(), 0);
        }

        *iter.next().unwrap()
    }

    pub fn solve2(&self) -> i64 {
        let mut intcode = Intcode::from(self.s);
        intcode.set_input(VecDeque::from([5]));
        intcode.run();
        let mut output = intcode.take_output();
        output.pop_front().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn input() {
            assert_eq!(Puzzle::new(include_str!("../input/5")).solve(), 8332629);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn input() {
            assert_eq!(Puzzle::new(include_str!("../input/5")).solve2(), 8805067);
        }
    }
}
