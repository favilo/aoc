mod parse;

pub(crate) use self::parse::parse_program;

#[derive(Default, Debug, Clone)]
pub struct Machine {
    instructions: Vec<(Op, bool)>,
    ip: usize,
    acc: i32,
}

impl Machine {
    pub fn new(ops: &[Op]) -> Self {
        Self {
            instructions: ops.iter().map(|op| (*op, false)).collect(),
            ..Default::default()
        }
    }

    pub fn step(&mut self) -> Result<(), &'static str> {
        let op = self.instructions[self.ip];
        self.instructions[self.ip].1 = true;
        match op {
            (Op::Nop(_), _) => (),
            (Op::Acc(i), _) => self.acc += i,
            (Op::Jmp(i), _) => {
                if self.ip >= self.instructions.len() || self.ip as i32 + i < 0 {
                    // eprintln!("Out of range: {}<>{}", self.ip, self.instructions.len());
                    return Err("Out of range");
                }
                self.ip = (self.ip as i32 + i) as usize;
                return Ok(());
            }
        }
        self.ip += 1;
        Ok(())
    }

    pub fn run(&mut self) -> Result<&Self, &Self> {
        while self.ip < self.instructions.len() && !self.instructions[self.ip].1 {
            if self.step().is_err() {
                return Err(self);
            }
        }

        Ok(self)
    }

    pub fn flip(&mut self, ip: usize) -> &Self {
        match self.instructions[ip] {
            (Op::Nop(i), _) => self.instructions[ip].0 = Op::Jmp(i),
            (Op::Jmp(i), _) => self.instructions[ip].0 = Op::Nop(i),
            _ => (),
        }
        self
    }

    pub fn get(&self) -> i32 {
        self.acc
    }

    pub fn curr_ip(&self) -> usize {
        self.ip
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}
