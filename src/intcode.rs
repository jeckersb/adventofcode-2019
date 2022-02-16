use std::collections::VecDeque;

pub struct Intcode {
    data: Vec<i64>,
    ip: usize,
    input: Option<VecDeque<i64>>,
    output: Option<VecDeque<i64>>,
}

#[derive(PartialEq, Eq)]
enum Opcode {
    Add = 1,
    Mul = 2,
    Input = 3,
    Output = 4,
    Halt = 99,
}

#[derive(PartialEq, Eq)]
enum ParameterMode {
    Position = 0,
    Immediate = 1,
}

struct Instruction {
    opcode: Opcode,
    p_mode: [ParameterMode; 3],
    len: usize,
}

impl From<i64> for Opcode {
    fn from(n: i64) -> Self {
        match n {
            1 => Opcode::Add,
            2 => Opcode::Mul,
            3 => Opcode::Input,
            4 => Opcode::Output,
            99 => Opcode::Halt,
            other => panic!("Unknown opcode {other}"),
        }
    }
}

impl From<i64> for ParameterMode {
    fn from(n: i64) -> Self {
        match n {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            other => panic!("Unknown parameter mode {other}"),
        }
    }
}

impl From<i64> for Instruction {
    fn from(n: i64) -> Self {
        let mut digits = n.to_string();

        while digits.len() < 5 {
            digits.insert(0, '0');
        }

        let opcode = Opcode::from(digits[3..].parse::<i64>().unwrap());
        let p_mode = [
            ParameterMode::from(digits[2..3].parse::<i64>().unwrap()),
            ParameterMode::from(digits[1..2].parse::<i64>().unwrap()),
            ParameterMode::from(digits[0..1].parse::<i64>().unwrap()),
        ];

        let len = match opcode {
            Opcode::Add | Opcode::Mul => 4,
            Opcode::Input | Opcode::Output => 2,
            Opcode::Halt => 1,
        };

        Self {
            opcode,
            p_mode,
            len,
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
            input: None,
            output: Some(VecDeque::new()),
        }
    }
}

impl From<&[i64]> for Intcode {
    fn from(s: &[i64]) -> Self {
        Self {
            data: Vec::from(s),
            ip: 0,
            input: None,
            output: Some(VecDeque::new()),
        }
    }
}

impl Intcode {
    pub fn set_input(&mut self, input: VecDeque<i64>) {
        self.input = Some(input);
    }

    pub fn run(&mut self) {
        loop {
            match self.next_instr() {
                Instruction { opcode, .. } if opcode == Opcode::Halt => break,
                _ => self.next(),
            }
        }
    }

    pub fn run_and_take(mut self) -> Vec<i64> {
        self.run();
        self.data
    }

    fn next(&mut self) {
        let ins = self.next_instr();
        match ins.opcode {
            Opcode::Add => {
                let op1 = match ins.p_mode[0] {
                    ParameterMode::Position => self.data[self.data[self.ip + 1] as usize],
                    ParameterMode::Immediate => self.data[self.ip + 1],
                };

                let op2 = match ins.p_mode[1] {
                    ParameterMode::Position => self.data[self.data[self.ip + 2] as usize],
                    ParameterMode::Immediate => self.data[self.ip + 2],
                };

                let dst = match ins.p_mode[2] {
                    ParameterMode::Position => self.data[self.ip + 3] as usize,
                    ParameterMode::Immediate => panic!("Unexpected write in immediate mode"),
                };

                self.data[dst] = op1 + op2;
            }
            Opcode::Mul => {
                let op1 = match ins.p_mode[0] {
                    ParameterMode::Position => self.data[self.data[self.ip + 1] as usize],
                    ParameterMode::Immediate => self.data[self.ip + 1],
                };

                let op2 = match ins.p_mode[1] {
                    ParameterMode::Position => self.data[self.data[self.ip + 2] as usize],
                    ParameterMode::Immediate => self.data[self.ip + 2],
                };

                let dst = match ins.p_mode[2] {
                    ParameterMode::Position => self.data[self.ip + 3] as usize,
                    ParameterMode::Immediate => panic!("Unexpected write in immediate mode"),
                };

                self.data[dst] = op1 * op2;
            }
            Opcode::Input => {
                assert!(self.input.is_some());

                match ins.p_mode[0] {
                    ParameterMode::Position => {
                        let idx = self.data[self.ip + 1] as usize;
                        self.data[idx] = self.input.as_mut().unwrap().pop_front().unwrap()
                    }
                    ParameterMode::Immediate => panic!("Unexpected input in immediate mode"),
                }
            }
            Opcode::Output => {
                assert!(self.output.is_some());

                match ins.p_mode[0] {
                    ParameterMode::Position => {
                        let idx = self.data[self.ip + 1] as usize;

                        self.output.as_mut().unwrap().push_back(self.data[idx]);
                    }
                    ParameterMode::Immediate => self
                        .output
                        .as_mut()
                        .unwrap()
                        .push_back(self.data[self.ip + 1]),
                }
            }
            Opcode::Halt => {}
        }

        self.ip += ins.len;
    }

    fn next_instr(&self) -> Instruction {
        Instruction::from(self.data[self.ip])
    }

    pub fn take_output(&mut self) -> VecDeque<i64> {
        self.output.take().unwrap()
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
                Intcode::from("1,9,10,3,2,3,11,0,99,30,40,50").run_and_take(),
                vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
            );
            assert_eq!(
                Intcode::from("1,0,0,0,99").run_and_take(),
                vec![2, 0, 0, 0, 99]
            );
            assert_eq!(
                Intcode::from("2,3,0,3,99").run_and_take(),
                vec![2, 3, 0, 6, 99]
            );
            assert_eq!(
                Intcode::from("2,4,4,5,99,0").run_and_take(),
                vec![2, 4, 4, 5, 99, 9801]
            );
            assert_eq!(
                Intcode::from("1,1,1,4,99,5,6,0,99").run_and_take(),
                vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
            );
        }
    }

    mod day5 {
        use super::*;

        #[test]
        fn examples() {
            assert_eq!(
                Intcode::from("1002,4,3,4,33").run_and_take(),
                vec![1002, 4, 3, 4, 99]
            );
            assert_eq!(
                Intcode::from("1101,100,-1,4,0").run_and_take(),
                vec![1101, 100, -1, 4, 99]
            );
        }
    }
}
