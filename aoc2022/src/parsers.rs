use std::fmt::Debug;

use heapless::Vec;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, one_of},
    combinator::{map, opt},
    error::{ErrorKind, ParseError},
    multi::many1,
    sequence::{terminated, tuple},
    AsBytes, AsChar, Err, IResult, InputLength, InputTakeAtPosition, Parser,
};
use nom_locate::LocatedSpan;
use nom_supreme::error::BaseErrorKind;

use crate::utils::parse_int;

#[allow(dead_code)]
pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(thiserror::Error, Debug, miette::Diagnostic)]
#[error("Bad input")]
pub struct BadInput<'input> {
    #[source_code]
    src: &'input str,

    #[label("{kind}")]
    bad_bit: miette::SourceSpan,

    kind: BaseErrorKind<&'input str, Box<dyn std::error::Error + Send + Sync + 'input>>,
}

pub fn single_digit_line<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], std::vec::Vec<usize>, E>
where
    E: ParseError<&'a [u8]>,
{
    terminated(
        many1(map(one_of("0123456789"), |s| (s as u8 - b'0') as usize)),
        multispace0,
    )(input)
}

pub fn number<S, U, E>(input: S) -> IResult<S, U, E>
where
    S: AsBytes + Clone + InputTakeAtPosition,
    U: TryFrom<usize>,
    E: ParseError<S>,
    <S as InputTakeAtPosition>::Item: AsChar,
    <U as TryFrom<usize>>::Error: Debug,
{
    map(digit1, |d: S| U::try_from(parse_int(d.as_bytes())).unwrap())(input)
}

pub fn signed_number<'input, E>(input: &'input [u8]) -> IResult<&'input [u8], isize, E>
where
    E: ParseError<&'input [u8]>,
{
    map(
        tuple((opt(tag(b"-")), digit1)),
        |(s, d): (Option<&[u8]>, &[u8])| {
            let parsed = parse_int(d.as_bytes()) as isize;
            if s.is_some() {
                -parsed
            } else {
                parsed
            }
        },
    )(input)
}

#[allow(dead_code)]
pub fn many1_heapless<I, O, E, F, const N: usize>(
    mut f: F,
) -> impl FnMut(I) -> IResult<I, Vec<O, N>, E>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    E: ParseError<I>,
{
    move |mut i: I| match f.parse(i.clone()) {
        Err(Err::Error(err)) => Err(Err::Error(E::append(i, ErrorKind::Many1, err))),
        Err(e) => Err(e),
        Ok((i1, o)) => {
            let mut acc = heapless::Vec::<_, N>::new();
            acc.push(o)
                .map_err(|_| panic!("vector not big enough"))
                .unwrap();
            i = i1;

            loop {
                let len = i.input_len();
                match f.parse(i.clone()) {
                    Err(Err::Error(_)) => return Ok((i, acc)),
                    Err(e) => return Err(e),
                    Ok((i1, o)) => {
                        // infinite loop check: the parser must always consume
                        if i1.input_len() == len {
                            return Err(Err::Error(E::from_error_kind(i, ErrorKind::Many1)));
                        }

                        i = i1;
                        acc.push(o)
                            .map_err(|_| panic!("vector not big enough"))
                            .unwrap();
                    }
                }
            }
        }
    }
}
