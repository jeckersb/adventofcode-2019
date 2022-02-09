pub struct Intcode {
    data: Vec<i64>,
    ip: usize,
}

#[derive(PartialEq, Eq)]
enum Opcode {
    Add = 1,
    Mul = 2,
    Halt = 99,
}

impl From<i64> for Opcode {
    fn from(n: i64) -> Self {
        match n {
            1 => Opcode::Add,
            2 => Opcode::Mul,
            99 => Opcode::Halt,
            other => panic!("Unknown opcode {other}"),
        }
    }
}

impl From<&str> for Intcode {
    fn from(s: &str) -> Self {
        Self {
            data: s
                .trim()
                .split(',')
                .map(|i| i.parse::<i64>().unwrap())
                .collect(),
            ip: 0,
        }
    }
}

impl From<&[i64]> for Intcode {
    fn from(s: &[i64]) -> Self {
        Self {
            data: Vec::from(s),
            ip: 0,
        }
    }
}

impl Intcode {
    pub fn run(mut self) -> Vec<i64> {
        while self.next_instr() != Opcode::Halt {
            self.next();
        }
        self.data
    }

    fn next(&mut self) {
        match self.next_instr() {
            Opcode::Add => match self.data[self.ip + 1..self.ip + 4] {
                [term1, term2, dst] => {
                    self.data[dst as usize] = self.data[term1 as usize] + self.data[term2 as usize]
                }
                _ => panic!("malformed add operation"),
            },
            Opcode::Mul => match self.data[self.ip + 1..self.ip + 4] {
                [term1, term2, dst] => {
                    self.data[dst as usize] = self.data[term1 as usize] * self.data[term2 as usize]
                }
                _ => panic!("malformed mul operation"),
            },

            Opcode::Halt => {}
        }

        self.ip += 4;
    }

    fn next_instr(&self) -> Opcode {
        Opcode::from(self.data[self.ip])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod day2 {
        use super::*;

        #[test]
        fn examples() {
            assert_eq!(
                Intcode::from("1,9,10,3,2,3,11,0,99,30,40,50").run(),
                vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
            );
            assert_eq!(Intcode::from("1,0,0,0,99").run(), vec![2, 0, 0, 0, 99]);
            assert_eq!(Intcode::from("2,3,0,3,99").run(), vec![2, 3, 0, 6, 99]);
            assert_eq!(
                Intcode::from("2,4,4,5,99,0").run(),
                vec![2, 4, 4, 5, 99, 9801]
            );
            assert_eq!(
                Intcode::from("1,1,1,4,99,5,6,0,99").run(),
                vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
            );
        }
    }
}
