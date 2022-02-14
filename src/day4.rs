pub struct Puzzle<'a> {
    s: &'a str,
}

fn into_digits(mut n: i32) -> [i32; 6] {
    let mut res: [i32; 6] = [0; 6];

    if n > 100_000 {
        res[0] = n / 100_000;
        n -= res[0] * 100_000;
    }

    if n > 10_000 {
        res[1] = n / 10_000;
        n -= res[1] * 10_000;
    }

    if n > 1_000 {
        res[2] = n / 1_000;
        n -= res[2] * 1_000;
    }

    if n > 100 {
        res[3] = n / 100;
        n -= res[3] * 100;
    }

    if n > 10 {
        res[4] = n / 10;
        n -= res[4] * 10;
    }

    res[5] = n;

    res
}

fn valid_password(n: i32) -> bool {
    let digits = into_digits(n);

    digits.windows(2).any(|pair| pair[0] == pair[1])
        && digits.windows(2).all(|pair| pair[1] >= pair[0])
}

fn valid_password2(n: i32) -> bool {
    let digits = into_digits(n);

    if !digits.windows(2).all(|pair| pair[1] >= pair[0]) {
        return false;
    }

    match digits {
        [a, b, c, ..] if a == b && b != c => true,
        [a, b, c, d, ..] if a != b && b == c && c != d => true,
        [_, a, b, c, d, _] if a != b && b == c && c != d => true,
        [_, _, a, b, c, d] if a != b && b == c && c != d => true,
        [_, _, _, a, b, c] if a != b && b == c => true,
        _ => false,
    }
}

impl<'a> Puzzle<'a> {
    pub fn new(s: &'a str) -> Self {
        Puzzle { s }
    }

    pub fn solve(&self) -> usize {
        let parts: Vec<_> = self.s.split('-').collect();
        let from = parts[0].parse::<i32>().unwrap();
        let to = parts[1].parse::<i32>().unwrap();

        (from..=to).filter(|n| valid_password(*n)).count()
    }

    pub fn solve2(&self) -> usize {
        let parts: Vec<_> = self.s.split('-').collect();
        let from = parts[0].parse::<i32>().unwrap();
        let to = parts[1].parse::<i32>().unwrap();

        (from..=to).filter(|n| valid_password2(*n)).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn examples() {}

        #[test]
        fn input() {
            assert_eq!(Puzzle::new(include_str!("../input/4")).solve(), 1864);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn examples() {}

        #[test]
        fn input() {
            assert_eq!(Puzzle::new(include_str!("../input/4")).solve2(), 1258);
        }
    }
}
