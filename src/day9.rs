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

        let output = intcode.output().unwrap();
        assert!(intcode.output().is_none());

        output
    }

    pub fn solve2(&self) -> i64 {
        let mut intcode = Intcode::from(self.s);
        intcode.input(2);
        intcode.run();

        let output = intcode.output().unwrap();
        assert!(intcode.output().is_none());

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn examples() { /* in intcode */
        }

        #[test]
        fn input() {
            assert_eq!(Puzzle::new(include_str!("../input/9")).solve(), 3742852857);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn examples() {}

        #[test]
        fn input() {
            assert_eq!(Puzzle::new(include_str!("../input/9")).solve2(), 73439);
        }
    }
}
