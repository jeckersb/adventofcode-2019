use itertools::Itertools;

use crate::intcode::{Intcode, RunResult};

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
                intcode.output().unwrap()
            });

            max = max.max(thrust);
        }

        max
    }

    pub fn solve2(&self) -> i64 {
        let mut max = i64::MIN;

        for perm in [5, 6, 7, 8, 9].iter().permutations(5) {
            let mut amps: Vec<Intcode> = Vec::new();

            for p in perm.iter() {
                let mut intcode = Intcode::from(self.s);
                intcode.input(**p);
                amps.push(intcode);
            }

            // init
            amps[0].input(0);

            let final_output = 'outer: loop {
                for i in 0..5 {
                    if i < 4 {
                        amps[i].run();
                        let output = amps[i].output().unwrap();
                        amps[i + 1].input(output);
                    } else {
                        match amps[i].run() {
                            RunResult::Halted => break 'outer amps[i].output().unwrap(),
                            _ => {
                                let output = amps[i].output().unwrap();
                                amps[0].input(output);
                            }
                        }
                    }
                }
            };

            max = max.max(final_output);
        }

        max
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
        fn examples() {
            assert_eq!(
                Puzzle::new(
                    "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,\
		     27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
                )
                .solve2(),
                139629729
            );
            assert_eq!(
                Puzzle::new(
                    "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,\
		     -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,\
		     53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
                )
                .solve2(),
                18216
            )
        }

        #[test]
        fn input() {
            assert_eq!(Puzzle::new(include_str!("../input/7")).solve2(), 3321777);
        }
    }
}
