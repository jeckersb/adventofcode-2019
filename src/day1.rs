pub struct Puzzle<'a> {
    s: &'a str,
}

fn required_fuel(mass: i32) -> i32 {
    ((mass as f64 / 3.0) as i32) - 2
}

fn required_fuel2(mass: i32) -> i32 {
    let required = ((mass as f64 / 3.0) as i32) - 2;

    if required <= 0 {
        return 0;
    }

    required + required_fuel2(required)
}

impl<'a> Puzzle<'a> {
    pub fn new(s: &'a str) -> Self {
        Puzzle { s }
    }

    pub fn solve(&self) -> i32 {
        self.s
            .lines()
            .map(|l| required_fuel(l.parse::<i32>().unwrap()))
            .sum()
    }

    pub fn solve2(&self) -> i32 {
        self.s
            .lines()
            .map(|l| required_fuel2(l.parse::<i32>().unwrap()))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn examples() {
            assert_eq!(required_fuel(12), 2);
            assert_eq!(required_fuel(14), 2);
            assert_eq!(required_fuel(1969), 654);
            assert_eq!(required_fuel(100756), 33583);
        }

        #[test]
        fn input() {
            assert_eq!(Puzzle::new(include_str!("../input/1")).solve(), 3394032);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn examples() {
            assert_eq!(required_fuel2(12), 2);
            assert_eq!(required_fuel2(1969), 966);
            assert_eq!(required_fuel2(100756), 50346);
        }

        #[test]
        fn input() {
            assert_eq!(Puzzle::new(include_str!("../input/1")).solve2(), 3394032);
        }
    }
}
