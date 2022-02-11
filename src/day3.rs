use std::collections::{HashMap, HashSet};

pub struct Puzzle<'a> {
    s: &'a str,
}

struct Wire(HashMap<(i32, i32), usize>);

impl From<&str> for Wire {
    fn from(s: &str) -> Self {
        let mut map = HashMap::new();

        let mut cur_x = 0;
        let mut cur_y = 0;
        let mut cur_len = 0;

        for m in s.split(',') {
            let n = m[1..].parse::<usize>().unwrap();

            let stepper: Box<dyn Fn(&mut i32, &mut i32)> = match m.chars().next().unwrap() {
                'U' => Box::new(|_, y| *y += 1),
                'D' => Box::new(|_, y| *y -= 1),
                'L' => Box::new(|x, _| *x -= 1),
                'R' => Box::new(|x, _| *x += 1),
                dir => panic!("unknown direction {dir}"),
            };

            for _ in 0..n {
                stepper(&mut cur_x, &mut cur_y);
                cur_len += 1;
                map.entry((cur_x, cur_y)).or_insert(cur_len);
            }
        }

        Self(map)
    }
}

impl Wire {
    fn point_set(&self) -> HashSet<(i32, i32)> {
        let mut set = HashSet::new();

        for (x, y) in self.0.keys() {
            set.insert((*x, *y));
        }

        set
    }
}

impl<'a> Puzzle<'a> {
    pub fn new(s: &'a str) -> Self {
        Puzzle { s }
    }

    pub fn solve(&self) -> i32 {
        let wires: Vec<Wire> = self.s.lines().map(Wire::from).collect();
        assert_eq!(wires.len(), 2);

        wires[0]
            .point_set()
            .intersection(&wires[1].point_set())
            .map(|(x, y)| x.abs() + y.abs())
            .min()
            .unwrap()
    }

    pub fn solve2(&self) -> i32 {
        let wires: Vec<Wire> = self.s.lines().map(Wire::from).collect();
        assert_eq!(wires.len(), 2);

        wires[0]
            .point_set()
            .intersection(&wires[1].point_set())
            .map(|p| wires[0].0.get(p).unwrap() + wires[1].0.get(p).unwrap())
            .min()
            .unwrap()
            .try_into()
            .unwrap()
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
                Puzzle::new(
                    "R8,U5,L5,D3\n\
		     U7,R6,D4,L4\n"
                )
                .solve(),
                6
            );
            assert_eq!(
                Puzzle::new(
                    "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
		     U62,R66,U55,R34,D71,R55,D58,R83\n"
                )
                .solve(),
                159
            );
            assert_eq!(
                Puzzle::new(
                    "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
		     U98,R91,D20,R16,D67,R40,U7,R15,U6,R7\n"
                )
                .solve(),
                135
            );
        }

        #[test]
        fn input() {
            assert_eq!(Puzzle::new(include_str!("../input/3")).solve(), 2180);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn examples() {
            assert_eq!(
                Puzzle::new(
                    "R8,U5,L5,D3\n\
		     U7,R6,D4,L4\n"
                )
                .solve2(),
                30
            );
            assert_eq!(
                Puzzle::new(
                    "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
		     U62,R66,U55,R34,D71,R55,D58,R83\n"
                )
                .solve2(),
                610
            );
            assert_eq!(
                Puzzle::new(
                    "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
		     U98,R91,D20,R16,D67,R40,U7,R15,U6,R7\n"
                )
                .solve2(),
                410
            );
        }

        #[test]
        fn input() {
            assert_eq!(Puzzle::new(include_str!("../input/3")).solve2(), 112316);
        }
    }
}
