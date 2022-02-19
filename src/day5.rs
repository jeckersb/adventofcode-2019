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
        intcode.input(1);
        intcode.run();

        loop {
            match intcode.output() {
                Some(0) => continue,
                Some(n) => break n,
                None => panic!("unexpected end of output"),
            }
        }
    }

    pub fn solve2(&self) -> i64 {
        let mut intcode = Intcode::from(self.s);
        intcode.input(5);
        intcode.run();
        intcode.output().unwrap()
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
