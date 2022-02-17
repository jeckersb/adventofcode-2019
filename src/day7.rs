use itertools::Itertools;

use crate::intcode::Intcode;

pub struct Puzzle<'a> {
    s: &'a str,
}

impl<'a> Puzzle<'a> {
    pub fn new(s: &'a str) -> Self {
        Puzzle { s }
    }

    pub fn solve(&self) -> i64 {
        let mut max = i64::MIN;

        for perm in [0, 1, 2, 3, 4].iter().permutations(5) {
            let thrust = perm.iter().fold(0, |acc, phase| {
                let mut intcode = Intcode::from(self.s);
                intcode.input(**phase);
                intcode.input(acc);
                intcode.run();
                intcode.take_output().pop_front().unwrap()
            });

            max = max.max(thrust);
        }

        max
    }

    pub fn solve2(&self) -> i64 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn examples() {
            assert_eq!(
                Puzzle::new("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0").solve(),
                43210
            );
            assert_eq!(
                Puzzle::new(
                    "3,23,3,24,1002,24,10,24,1002,23,-1,23,\
		     101,5,23,23,1,24,23,23,4,23,99,0,0"
                )
                .solve(),
                54321
            );
            assert_eq!(
                Puzzle::new(
                    "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,\
		     1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
                )
                .solve(),
                65210
            )
        }

        #[test]
        fn input() {
            assert_eq!(Puzzle::new(include_str!("../input/7")).solve(), 20413);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn input() {
            assert_eq!(Puzzle::new(include_str!("../input/7")).solve2(), 8805067);
        }
    }
}
