use std::{fs::OpenOptions, io::Write};

use ndarray::{Array2, Axis};
use nom::{
    character::complete::{multispace0, one_of},
    combinator::map,
    multi::many1,
    sequence::terminated,
    IResult,
};
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, CONTENT_TYPE, COOKIE},
    redirect::Policy,
};

pub fn mean(l: &[usize]) -> f64 {
    let sum = l.iter().sum::<usize>();
    (sum as f64) / (l.len() as f64)
}

pub fn median(l: &[usize]) -> usize {
    let len = l.len();
    let mid = len / 2;
    if len % 2 == 0 {
        (l[(mid - 1)] + l[mid]) / 2
    } else {
        l[mid]
    }
}

pub fn parse_int(b: &[u8]) -> usize {
    b.iter().fold(0, |a, c| a * 10 + (c & 0x0f) as usize)
}

pub fn single_digit_line<'a>(input: &'a [u8]) -> IResult<&'a [u8], Vec<usize>> {
    terminated(
        many1(map(one_of("0123456789"), |s| {
            (s as u8 - '0' as u8) as usize
        })),
        multispace0,
    )(input)
}

pub fn download_input(
    day: usize,
    year: usize,
    session: &str,
    filename: &str,
) -> anyhow::Result<()> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    log::info!("Downloading: {}", url);
    let cookie_header = HeaderValue::from_str(&format!("session={}", session.trim()))
        .map_err(|err| anyhow::anyhow!("Err: {:?}", err))?;
    let content_header = HeaderValue::from_str("text/plain")?;
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, cookie_header);
    headers.insert(CONTENT_TYPE, content_header);
    let client = Client::builder()
        .default_headers(headers)
        .redirect(Policy::none())
        .build()?;
    let text = client
        .get(&url)
        .send()
        .and_then(|response| response.text())?;
    log::info!("Saving file: {}", filename);
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?
        .write(text.as_bytes())?;
    Ok(())
}

pub fn four_neighbors(
    idx: (usize, usize),
    shape: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    [
        (idx.0 as isize - 1, idx.1 as isize),
        (idx.0 as isize, idx.1 as isize - 1),
        (idx.0 as isize + 1, idx.1 as isize),
        (idx.0 as isize, idx.1 as isize + 1),
    ]
    .into_iter()
    .filter(|&(x, y)| x >= 0 && y >= 0)
    .filter(move |&(x, y)| x < shape.0 as isize && y < shape.1 as isize)
    .map(|(x, y)| (x as usize, y as usize))
}

#[allow(dead_code)]
pub fn print_array(array: &Array2<usize>) {
    for row in array.axis_iter(Axis(0)) {
        for c in row {
            print!("{}", c);
        }
        println!("");
    }
}
