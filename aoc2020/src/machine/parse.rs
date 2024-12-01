use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0, space1},
    combinator::opt,
    multi::many0,
    sequence::{terminated, tuple},
    IResult,
};

use super::Op;

fn acc(input: &str) -> IResult<&str, &str> {
    tag("acc")(input)
}

fn nop(input: &str) -> IResult<&str, &str> {
    tag("nop")(input)
}

fn jmp(input: &str) -> IResult<&str, &str> {
    tag("jmp")(input)
}

fn op_code(input: &str) -> IResult<&str, &str> {
    Ok(alt((nop, acc, jmp))(input)?)
}

fn parse_int<'a>(input: &'a str) -> IResult<&'a str, i32> {
    let (input, (sign, int)) = tuple((opt(alt((tag("+"), tag("-")))), digit1))(input)?;
    let mut int = int.parse::<i32>().unwrap();
    if sign == Some("-") {
        int = -int;
    }
    Ok((input, int))
}

fn parse_opcode<'a>(input: &'a str) -> IResult<&'a str, Op> {
    let (input, (op, int)) = terminated(
        tuple((terminated(op_code, space1), parse_int)), // `OP int`
        multispace0,
    )(input)?;
    let op = match op {
        "nop" => Op::Nop(int),
        "jmp" => Op::Jmp(int),
        "acc" => Op::Acc(int),
        _ => unreachable!(),
    };
    Ok((input, op))
}

pub(crate) fn parse_program<'a>(input: &'a str) -> IResult<&'a str, Vec<Op>> {
    let (input, ops) = many0(parse_opcode)(input)?;
    Ok((input, ops))
}
