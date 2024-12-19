use std::collections::VecDeque;

use itertools::Itertools;
use miette::{IntoDiagnostic, Result};
use winnow::{
    ascii::{dec_int, dec_uint, line_ending, multispace0},
    combinator::{alt, opt, preceded, separated, terminated},
    seq,
    stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial},
    PResult, Parser,
};

use crate::{errors::ToMiette, Runner};

pub struct Day;

type LiteralOperand = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComboOperand {
    Zero,
    One,
    Two,
    Three,
    A,
    B,
    C,
    Reserved,
}

impl From<u8> for ComboOperand {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::A,
            5 => Self::B,
            6 => Self::C,
            7 => Self::Reserved,
            _ => panic!("Bad operand"),
        }
    }
}

impl ComboOperand {
    pub fn value(&self, machine: &Machine) -> isize {
        match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::A => machine.a,
            Self::B => machine.b,
            Self::C => machine.c,
            Self::Reserved => panic!("Reserved"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    /// The adv instruction (opcode 0) performs division.
    /// The numerator is the value in the A register.
    /// The denominator is found by raising 2 to the power of the instruction's combo operand.
    Adv(ComboOperand),

    /// The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the
    /// instruction's literal operand, then stores the result in register B.
    Bxl(LiteralOperand),

    /// The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
    /// (thereby keeping only its lowest 3 bits), then writes that value to the B register.
    Bst(ComboOperand),

    /// The jnz instruction (opcode 3) does nothing if the A register is 0.
    /// However, if the A register is not zero, it jumps by setting the instruction pointer to the
    /// value of its literal operand; if this instruction jumps, the instruction pointer is not
    /// increased by 2 after this instruction.
    Jnz(LiteralOperand),

    /// The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C,
    /// then stores the result in register B.
    /// (For legacy reasons, this instruction reads an operand but ignores it.)
    Bxc,

    /// The out instruction (opcode 5) calculates the value of its combo operand modulo 8,
    /// then outputs that value.
    /// (If a program outputs multiple values, they are separated by commas.)
    Out(ComboOperand),

    /// The bdv instruction (opcode 6) works exactly like the adv instruction except that
    /// the result is stored in the B register.
    /// (The numerator is still read from the A register.)
    Bdv(ComboOperand),

    /// The cdv instruction (opcode 7) works exactly like the adv instruction except that the
    /// result is stored in the C register.
    /// (The numerator is still read from the A register.)
    Cdv(ComboOperand),
}

impl Instruction {
    pub fn try_from([operator, operand]: [u8; 2]) -> Result<Self> {
        Ok(match operator {
            0 => Self::Adv(ComboOperand::from(operand)),
            1 => Self::Bxl(LiteralOperand::from(operand)),
            2 => Self::Bst(ComboOperand::from(operand)),
            3 => Self::Jnz(LiteralOperand::from(operand)),
            4 => Self::Bxc,
            5 => Self::Out(ComboOperand::from(operand)),
            6 => Self::Bdv(ComboOperand::from(operand)),
            7 => Self::Cdv(ComboOperand::from(operand)),
            _ => miette::bail!("Invalid instruction"),
        })
    }

    pub fn parser<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone + Copy,
        <S as Stream>::Slice: AsBStr + AsRef<str>,
    {
        terminated(
            alt((
                Self::adv,
                Self::bxl,
                Self::bst,
                Self::jnz,
                Self::bxc,
                Self::out,
                Self::bdv,
                Self::cdv,
            )),
            (opt(","), multispace0),
        )
        .parse_next(input)
    }

    fn adv<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone + Copy,
        <S as Stream>::Slice: AsBStr + AsRef<str>,
    {
        preceded(
            ("0,", multispace0),
            dec_uint::<_, u8, _>.map(ComboOperand::from).map(Self::Adv),
        )
        .parse_next(input)
    }

    fn bxl<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone + Copy,
        <S as Stream>::Slice: AsBStr + AsRef<str>,
    {
        preceded(("1,", multispace0), dec_uint::<_, u8, _>.map(Self::Bxl)).parse_next(input)
    }

    fn bst<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone + Copy,
        <S as Stream>::Slice: AsBStr + AsRef<str>,
    {
        preceded(
            ("2,", multispace0),
            dec_uint::<_, u8, _>.map(ComboOperand::from).map(Self::Bst),
        )
        .parse_next(input)
    }

    fn jnz<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone + Copy,
        <S as Stream>::Slice: AsBStr + AsRef<str>,
    {
        preceded(("3,", multispace0), dec_uint::<_, u8, _>.map(Self::Jnz)).parse_next(input)
    }

    fn bxc<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone + Copy,
        <S as Stream>::Slice: AsBStr + AsRef<str>,
    {
        preceded(("4,", multispace0), dec_uint::<_, u8, _>.map(|_| Self::Bxc)).parse_next(input)
    }

    fn out<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone + Copy,
        <S as Stream>::Slice: AsBStr + AsRef<str>,
    {
        preceded(
            ("5,", multispace0),
            dec_uint::<_, u8, _>.map(ComboOperand::from).map(Self::Out),
        )
        .parse_next(input)
    }

    fn bdv<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone + Copy,
        <S as Stream>::Slice: AsBStr + AsRef<str>,
    {
        preceded(
            ("6,", multispace0),
            dec_uint::<_, u8, _>.map(ComboOperand::from).map(Self::Bdv),
        )
        .parse_next(input)
    }

    fn cdv<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone + Copy,
        <S as Stream>::Slice: AsBStr + AsRef<str>,
    {
        preceded(
            ("7,", multispace0),
            dec_uint::<_, u8, _>.map(ComboOperand::from).map(Self::Cdv),
        )
        .parse_next(input)
    }

    fn size(&self, machine: &Machine) -> usize {
        match self {
            Self::Jnz(_) if machine.a != 0 => 0,
            _ => 2,
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Machine {
    a: isize,
    b: isize,
    c: isize,

    ip: usize,
    bytes: Vec<u8>,
}

impl std::fmt::Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Machine")
            .field("a", &self.a)
            .field("b", &self.b)
            .field("c", &self.c)
            .field("ip", &self.ip)
            .field_with("bytes", |f| {
                f.debug_list()
                    .entries(
                        self.bytes
                            .iter()
                            .copied()
                            .tuples()
                            .map(|(a, b)| Instruction::try_from([a, b]).unwrap()),
                    )
                    .finish()
            })
            .finish()
    }
}

impl Machine {
    pub fn parser<S>(input: &mut S) -> PResult<Self>
    where
        for<'a> S: Stream + StreamIsPartial + Compare<&'a str>,
        <S as Stream>::Token: AsChar + Clone + Copy,
        <S as Stream>::Slice: AsBStr + AsRef<str>,
    {
        seq!(
            seq!(_: "Register A: ", dec_int, _: multispace0).map(|t| t.0),
            seq!(_: "Register B: ", dec_int, _: multispace0).map(|t| t.0),
            seq!(_: "Register C: ", dec_int, _: multispace0).map(|t| t.0),
            _: multispace0,
            seq!(_: "Program: ", separated(1.., dec_uint::<_, u8,_>, ","), _: line_ending).map(|t| t.0),
        )
        .map(|(a, b, c, bytes)| Self {
            a,
            b,
            c,
            ip: 0,
            bytes,
        })
        .parse_next(input)
    }

    pub fn run(&mut self) -> Result<Vec<u8>> {
        let mut outs = Vec::new();
        while self.ip < self.bytes.len() {
            let instruction =
                Instruction::try_from(self.bytes[self.ip..][..2].try_into().into_diagnostic()?)?;
            // self.print_state(&outs);
            self.execute(instruction, &mut outs);
            self.ip += instruction.size(self);
        }
        Ok(outs)
    }

    pub fn run_to_first_output(&mut self) -> Option<u8> {
        let mut outs = Vec::new();
        while self.ip < self.bytes.len() {
            let instruction =
                Instruction::try_from(self.bytes[self.ip..][..2].try_into().ok()?).ok()?;
            self.execute(instruction, &mut outs);
            if !outs.is_empty() {
                break;
            }
            self.ip += instruction.size(self);
        }
        outs.last().copied()
    }

    fn execute(&mut self, inst: Instruction, outs: &mut Vec<u8>) {
        match inst {
            Instruction::Adv(operand) => self.a >>= operand.value(self),
            Instruction::Bxl(operand) => self.b ^= operand as isize,
            Instruction::Bst(operand) => self.b = operand.value(self) % 8,
            Instruction::Jnz(operand) => {
                if self.a != 0 {
                    self.ip = operand as usize;
                }
            }
            Instruction::Bxc => self.b ^= self.c,
            Instruction::Out(operand) => {
                let value = operand.value(self);
                outs.push(value as u8 % 8);
            }
            Instruction::Bdv(operand) => self.b = self.a >> operand.value(self),
            Instruction::Cdv(operand) => self.c = self.a >> operand.value(self),
        }
    }

    #[allow(dead_code)]
    fn print_state(&self, outs: &[u8]) {
        log::debug!(
            "a={:16o} b={:16o} c={:16o} ip={:4} inst:{:?}\n{outs:?}",
            self.a,
            self.b,
            self.c,
            self.ip,
            Instruction::try_from(self.bytes[self.ip..][..2].try_into().unwrap()).unwrap(),
        );
    }
}

impl Runner<String> for Day {
    type Input<'input> = Machine;

    fn day() -> usize {
        17
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        Machine::parser.parse(input).to_miette()
    }

    fn part1(input: &Self::Input<'_>) -> Result<String> {
        let mut machine = input.clone();
        let outs = machine.run()?;
        Ok(outs.iter().map(ToString::to_string).join(","))
    }

    fn part2(input: &Self::Input<'_>) -> Result<usize> {
        let machine = input.clone();
        let bytes = machine.bytes.clone();

        let a = find_a(&machine)?;

        let mut machine = machine;
        machine.a = a as isize;
        let output = machine.run()?;
        log::debug!("Output from {a}: {output:?}");
        assert_eq!(output, bytes);

        Ok(a as usize)
    }
}

fn find_a(machine: &Machine) -> std::result::Result<usize, miette::Error> {
    let bytes = machine.bytes.clone();
    log::debug!("bytes: {bytes:?}");
    let mut queue = VecDeque::from([(0_isize, bytes.as_slice())]);
    while let Some((a, bytes)) = queue.pop_front() {
        if bytes.is_empty() {
            return Ok(a as usize);
        }
        log::debug!("a: {a} looking for: {}", bytes.last().unwrap());
        get_potential_digits(machine, a, bytes, &mut queue);
        log::debug!("queue: {queue:?}");
    }
    miette::bail!("No solution found")
}

fn get_potential_digits<'bytes>(
    machine: &Machine,
    a: isize,
    bytes: &'bytes [u8],
    queue: &mut VecDeque<(isize, &'bytes [u8])>,
) {
    for i in 0..8 {
        let mut new_machine = machine.clone();
        let all_bytes = &machine.bytes;
        let a = (a << 3) + i;
        new_machine.a = a;
        let output = new_machine.run_to_first_output();
        log::debug!("found: {output:?} with {a}");
        if output == bytes.last().copied() {
            let mut machine = machine.clone();
            machine.a = a;
            let output = machine.run().unwrap();
            log::debug!("Output from {a}: {output:?}");
            let these = &all_bytes[all_bytes.len() - output.len()..];
            log::debug!("Bytes: {:?}", these);
            if output == these {
                let next_bytes = &bytes[..bytes.len() - 1];
                let entry = (a, next_bytes);
                log::debug!("Pushing: {entry:?}");
                queue.push_back(entry);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input1 = "\
                Register A: 729\n\
                Register B: 0\n\
                Register C: 0\n\
                \n\
                Program: 0,1,5,4,3,0\n\
            ";
            part1 = "4,6,3,5,6,3,5,2,1,0";
            input2 = "\
               Register A: 117440\n\
                Register B: 0\n\
                Register C: 0\n\
                \n\
                Program: 0,3,5,4,3,0\n\
            ";
            part2 = 117440;
    }

    prod_case! {
        part1 = "1,5,3,0,2,5,2,5,3";
        part2 = 108107566389757;
    }

    #[test]
    fn test1() {
        let mut machine = Machine {
            c: 9,
            bytes: vec![2, 6],
            ..Default::default()
        };
        machine.run().unwrap();
        assert_eq!(machine.b, 1);
    }

    #[test]
    fn test2() {
        let mut machine = Machine {
            a: 10,
            bytes: vec![5, 0, 5, 1, 5, 4],
            ..Default::default()
        };
        assert_eq!(machine.run().unwrap(), [0, 1, 2]);
    }

    #[test]
    fn test3() {
        let mut machine = Machine {
            a: 2024,
            bytes: vec![0, 1, 5, 4, 3, 0],
            ..Default::default()
        };
        assert_eq!(machine.run().unwrap(), [4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(machine.a, 0)
    }

    #[test]
    fn test4() {
        let mut machine = Machine {
            b: 29,
            bytes: vec![1, 7],
            ..Default::default()
        };
        assert_eq!(machine.run().unwrap(), [0; 0]);
        assert_eq!(machine.b, 26);
    }

    #[test]
    fn test5() {
        let mut machine = Machine {
            b: 2024,
            c: 43690,
            bytes: vec![4, 0],
            ..Default::default()
        };
        assert_eq!(machine.run().unwrap(), [0; 0]);
        assert_eq!(machine.b, 44354);
    }

    #[test]
    fn test_quine() {
        let input = "\
            Register A: 117440\n\
            Register B: 0\n\
            Register C: 0\n\
            \n\
            Program: 0,3,5,4,3,0\n\
        ";
        let mut machine = Machine::parser.parse(input).unwrap();
        let expected = machine.bytes.clone();
        assert_eq!(machine.run().unwrap(), expected);
    }
}
