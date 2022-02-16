use crate::intcode::Intcode;

pub struct Puzzle<'a> {
    s: &'a str,
}

impl<'a> Puzzle<'a> {
    pub fn new(s: &'a str) -> Self {
        Puzzle { s }
    }

    pub fn solve(&self) -> i64 {
        let mut v: Vec<_> = self.s.split(',').collect();
        v[1] = "12";
        v[2] = "2";

        let modified = v.join(",");

        Intcode::from(modified.as_str()).run_and_take()[0]
    }

    pub fn solve2(&self) -> i64 {
        let mut v: Vec<_> = self
            .s
            .trim()
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        for noun in 0..=99 {
            for verb in 0..=99 {
                v[1] = noun;
                v[2] = verb;

                if Intcode::from(&v[..]).run_and_take()[0] == 19690720 {
                    return 100 * noun + verb;
                }
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn input() {
            assert_eq!(Puzzle::new(include_str!("../input/2")).solve(), 5290681);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn input() {
            assert_eq!(Puzzle::new(include_str!("../input/2")).solve2(), 5741);
        }
    }
}
