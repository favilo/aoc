use std::fmt::{Debug, Display};

use miette::miette;
use winnow::{
    error::{ParseError, ParserError},
    stream::{AsBStr, Stream},
    PResult,
};

#[allow(dead_code)]
pub trait ToMiette<O> {
    fn to_miette(self) -> Result<O, miette::Report>;
}

#[allow(dead_code)]
impl<O, E> ToMiette<O> for PResult<O, E> {
    fn to_miette(self) -> Result<O, miette::Report> {
        todo!()
    }
}

impl<O, S, C> ToMiette<O> for Result<O, ParseError<S, C>>
where
    S: Stream + AsBStr,
    C: ParserError<S> + Display + Debug,
{
    fn to_miette(self) -> Result<O, miette::Report> {
        self.map_err(|e| miette!("{e}"))
    }
}
