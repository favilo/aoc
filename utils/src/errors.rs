use std::fmt::{Debug, Display};

use miette::miette;
use winnow::{
    error::{ParseError, ParserError},
    stream::{AsBStr, Stream},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid input: {0}")]
    InvalidInput(String),
}

#[allow(dead_code)]
pub trait ToMiette<O> {
    fn to_miette(self) -> Result<O, miette::Report>;
}

pub trait ToMietteErr {
    fn to_miette(self) -> miette::Report;
}

impl<O, E> ToMiette<O> for Result<O, E>
where
    E: ToMietteErr,
{
    fn to_miette(self) -> Result<O, miette::Report> {
        self.map_err(ToMietteErr::to_miette)
    }
}

impl<S, C> ToMietteErr for ParseError<S, C>
where
    S: Stream + AsBStr,
    C: ParserError<S> + Display + Debug,
{
    fn to_miette(self) -> miette::Report {
        miette!("{self}")
    }
}
