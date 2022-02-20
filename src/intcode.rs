use std::collections::{HashMap, VecDeque};

pub struct Intcode {
    memory: Memory,
    ip: usize,
    rb: isize,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
}

struct Memory(HashMap<usize, i64>);

#[derive(PartialEq, Eq)]
enum Opcode {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    Halt = 99,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum ParameterMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RunResult {
    Ok,
    Halted,
    BlockedOnInput,
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
            2 => Opcode::Multiply,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JumpIfTrue,
            6 => Opcode::JumpIfFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
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
            2 => ParameterMode::Relative,
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
            Opcode::Add | Opcode::Multiply | Opcode::LessThan | Opcode::Equals => 4,
            Opcode::JumpIfTrue | Opcode::JumpIfFalse => 3,
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
            memory: Memory::from(s),
            ip: 0,
            rb: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }
}

impl From<&[i64]> for Intcode {
    fn from(s: &[i64]) -> Self {
        Self {
            memory: Memory::from(s),
            ip: 0,
            rb: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }
}

impl From<&str> for Memory {
    fn from(s: &str) -> Self {
        let map = s
            .trim()
            .split(',')
            .map(|i| i.parse::<i64>().unwrap())
            .enumerate()
            .collect();

        Self(map)
    }
}

impl From<&[i64]> for Memory {
    fn from(s: &[i64]) -> Self {
        Self(s.iter().map(|i| *i).enumerate().collect())
    }
}

impl Intcode {
    pub fn input(&mut self, input: i64) {
        self.input.push_back(input);
    }

    pub fn output(&mut self) -> Option<i64> {
        self.output.pop_front()
    }

    pub fn run(&mut self) -> RunResult {
        loop {
            match self.next() {
                RunResult::Ok => continue,
                other => return other,
            }
        }
    }

    pub fn get_mem_range(&self, start: usize, end: usize) -> Vec<i64> {
        (start..end).map(|i| self.mem_get_addr(i)).collect()
    }

    fn next(&mut self) -> RunResult {
        let ins = self.next_instr();
        match ins.opcode {
            Opcode::Add => {
                let op1 = self.mem_get(ins.p_mode[0], 1);
                let op2 = self.mem_get(ins.p_mode[1], 2);

                let dst = match ins.p_mode[2] {
                    ParameterMode::Position => self.mem_get(ParameterMode::Immediate, 3),
                    ParameterMode::Immediate => panic!("Unexpected write in immediate mode"),
                    ParameterMode::Relative => panic!("Unexpected write in relative mode"),
                };

                self.mem_set(dst as usize, op1 + op2);
                self.ip += ins.len;
            }
            Opcode::Multiply => {
                let op1 = self.mem_get(ins.p_mode[0], 1);
                let op2 = self.mem_get(ins.p_mode[1], 2);

                let dst = match ins.p_mode[2] {
                    ParameterMode::Position => self.mem_get(ParameterMode::Immediate, 3),
                    ParameterMode::Immediate => panic!("Unexpected write in immediate mode"),
                    ParameterMode::Relative => panic!("Unexpected write in relative mode"),
                };

                self.mem_set(dst as usize, op1 * op2);
                self.ip += ins.len;
            }
            Opcode::Input => {
                match ins.p_mode[0] {
                    ParameterMode::Position => {
                        let idx = self.mem_get(ParameterMode::Immediate, 1) as usize;

                        match self.input.pop_front() {
                            Some(i) => self.mem_set(idx, i),
                            None => return RunResult::BlockedOnInput,
                        }
                    }
                    ParameterMode::Immediate => panic!("Unexpected input in immediate mode"),
                    ParameterMode::Relative => panic!("Unexpected input in relative mode"),
                }

                self.ip += ins.len;
            }
            Opcode::Output => {
                let op = self.mem_get(ins.p_mode[0], 1);
                self.output.push_back(op);

                self.ip += ins.len;
            }
            Opcode::JumpIfTrue => {
                let cond = self.mem_get(ins.p_mode[0], 1);

                if cond != 0 {
                    let target = self.mem_get(ins.p_mode[1], 2);
                    self.ip = target as usize;
                } else {
                    self.ip += ins.len;
                }
            }
            Opcode::JumpIfFalse => {
                let cond = self.mem_get(ins.p_mode[0], 1);

                if cond == 0 {
                    let target = self.mem_get(ins.p_mode[1], 2);
                    self.ip = target as usize;
                } else {
                    self.ip += ins.len;
                }
            }
            Opcode::LessThan => {
                let op1 = self.mem_get(ins.p_mode[0], 1);
                let op2 = self.mem_get(ins.p_mode[1], 2);

                let dst = match ins.p_mode[2] {
                    ParameterMode::Position => self.mem_get(ParameterMode::Immediate, 3) as usize,
                    ParameterMode::Immediate => panic!("Unexpected write in immediate mode"),
                    ParameterMode::Relative => panic!("Unexpected write in relative mode"),
                };

                if op1 < op2 {
                    self.mem_set(dst, 1);
                } else {
                    self.mem_set(dst, 0);
                }

                self.ip += ins.len;
            }
            Opcode::Equals => {
                let op1 = self.mem_get(ins.p_mode[0], 1);
                let op2 = self.mem_get(ins.p_mode[1], 2);

                let dst = match ins.p_mode[2] {
                    ParameterMode::Position => self.mem_get(ParameterMode::Immediate, 3) as usize,
                    ParameterMode::Immediate => panic!("Unexpected write in immediate mode"),
                    ParameterMode::Relative => panic!("Unexpected write in relative mode"),
                };

                if op1 == op2 {
                    self.mem_set(dst, 1);
                } else {
                    self.mem_set(dst, 0);
                }

                self.ip += ins.len;
            }

            Opcode::Halt => return RunResult::Halted,
        }

        RunResult::Ok
    }

    fn next_instr(&self) -> Instruction {
        Instruction::from(self.mem_get(ParameterMode::Immediate, 0))
    }

    fn mem_get(&self, mode: ParameterMode, offset: usize) -> i64 {
        self.memory.get(&self.ip, &self.rb, mode, offset)
    }

    fn mem_get_addr(&self, addr: usize) -> i64 {
        self.memory.get_addr(addr)
    }

    fn mem_set(&mut self, addr: usize, val: i64) {
        self.memory.set(addr, val)
    }
}

impl Memory {
    fn get(&self, ip: &usize, rb: &isize, mode: ParameterMode, offset: usize) -> i64 {
        match mode {
            ParameterMode::Position => {
                let pos = self.get_addr(*ip + offset);
                self.get_addr(pos as usize)
            }
            ParameterMode::Immediate => self.get_addr(*ip + offset),
            ParameterMode::Relative => {
                self.get_addr((*ip as isize + offset as isize + *rb) as usize)
            }
        }
    }

    fn get_addr(&self, addr: usize) -> i64 {
        match self.0.get(&addr) {
            None => 0,
            Some(n) => *n,
        }
    }

    fn set(&mut self, addr: usize, val: i64) {
        self.0.insert(addr, val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod day2 {
        use super::*;

        #[test]
        fn examples() {
            let mut intcode = Intcode::from("1,9,10,3,2,3,11,0,99,30,40,50");
            intcode.run();
            assert_eq!(
                intcode.get_mem_range(0, 12),
                vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
            );

            let mut intcode = Intcode::from("1,0,0,0,99");
            intcode.run();
            assert_eq!(intcode.get_mem_range(0, 5), vec![2, 0, 0, 0, 99]);

            let mut intcode = Intcode::from("2,3,0,3,99");
            intcode.run();
            assert_eq!(intcode.get_mem_range(0, 5), vec![2, 3, 0, 6, 99]);

            let mut intcode = Intcode::from("2,4,4,5,99,0");
            intcode.run();
            assert_eq!(intcode.get_mem_range(0, 6), vec![2, 4, 4, 5, 99, 9801]);

            let mut intcode = Intcode::from("1,1,1,4,99,5,6,0,99");
            intcode.run();
            assert_eq!(
                intcode.get_mem_range(0, 9),
                vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
            );
        }
    }

    mod day5 {
        use super::*;

        #[test]
        fn examples() {
            // part 1
            let mut intcode = Intcode::from("1002,4,3,4,33");
            intcode.run();
            assert_eq!(intcode.get_mem_range(0, 5), vec![1002, 4, 3, 4, 99]);

            let mut intcode = Intcode::from("1101,100,-1,4,0");
            intcode.run();
            assert_eq!(intcode.get_mem_range(0, 5), vec![1101, 100, -1, 4, 99]);

            // part 2

            // 3,9,8,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).

            let mut intcode = Intcode::from("3,9,8,9,10,9,4,9,99,-1,8");
            intcode.input(8);
            intcode.run();
            assert_eq!(intcode.output().unwrap(), 1);

            let mut intcode = Intcode::from("3,9,8,9,10,9,4,9,99,-1,8");
            intcode.input(42);
            intcode.run();
            assert_eq!(intcode.output().unwrap(), 0);

            // 3,9,7,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).

            let mut intcode = Intcode::from("3,9,7,9,10,9,4,9,99,-1,8");
            intcode.input(7);
            intcode.run();
            assert_eq!(intcode.output().unwrap(), 1);

            let mut intcode = Intcode::from("3,9,7,9,10,9,4,9,99,-1,8");
            intcode.input(42);
            intcode.run();
            assert_eq!(intcode.output().unwrap(), 0);

            // 3,3,1108,-1,8,3,4,3,99 - Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
            let mut intcode = Intcode::from("3,3,1108,-1,8,3,4,3,99");
            intcode.input(8);
            intcode.run();
            assert_eq!(intcode.output().unwrap(), 1);

            let mut intcode = Intcode::from("3,3,1108,-1,8,3,4,3,99");
            intcode.input(42);
            intcode.run();
            assert_eq!(intcode.output().unwrap(), 0);

            // 3,3,1107,-1,8,3,4,3,99 - Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
            let mut intcode = Intcode::from("3,3,1107,-1,8,3,4,3,99");
            intcode.input(7);
            intcode.run();
            assert_eq!(intcode.output().unwrap(), 1);

            let mut intcode = Intcode::from("3,3,1107,-1,8,3,4,3,99");
            intcode.input(42);
            intcode.run();
            assert_eq!(intcode.output().unwrap(), 0);

            // Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:

            // 3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9 (using position mode)
            let mut intcode = Intcode::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
            intcode.input(0);
            intcode.run();
            assert_eq!(intcode.output().unwrap(), 0);

            let mut intcode = Intcode::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
            intcode.input(42);
            intcode.run();
            assert_eq!(intcode.output().unwrap(), 1);

            // 3,3,1105,-1,9,1101,0,0,12,4,12,99,1 (using immediate mode)
            let mut intcode = Intcode::from("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
            intcode.input(0);
            intcode.run();
            assert_eq!(intcode.output().unwrap(), 0);

            let mut intcode = Intcode::from("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
            intcode.input(42);
            intcode.run();
            assert_eq!(intcode.output().unwrap(), 1);

            /*
            Here's a larger example:

            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99

            The above example program uses an input instruction to ask
            for a single number. The program will then output 999 if
            the input value is below 8, output 1000 if the input value
            is equal to 8, or output 1001 if the input value is
            greater than 8.
             */
            let mut intcode = Intcode::from(
                "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,\
		 1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,\
		 999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
            );
            intcode.input(7);
            intcode.run();
            assert_eq!(intcode.output().unwrap(), 999);

            let mut intcode = Intcode::from(
                "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,\
		 1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,\
		 999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
            );
            intcode.input(8);
            intcode.run();
            assert_eq!(intcode.output().unwrap(), 1000);

            let mut intcode = Intcode::from(
                "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,\
		 1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,\
		 999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
            );
            intcode.input(9);
            intcode.run();
            assert_eq!(intcode.output().unwrap(), 1001);
        }
    }
}
