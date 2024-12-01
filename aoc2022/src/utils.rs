pub(crate) mod top;

use std::{
    fs::OpenOptions,
    io::Write,
    ops::{Range, RangeInclusive, Sub},
};

use color_eyre::{eyre::eyre, Result};
use ndarray::{Array2, Axis};

use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, CONTENT_TYPE, COOKIE},
    redirect::Policy,
};

#[allow(dead_code)]
pub fn mean(l: &[usize]) -> f64 {
    let sum = l.iter().sum::<usize>();
    (sum as f64) / (l.len() as f64)
}

#[allow(dead_code)]
pub fn median(l: &[usize]) -> usize {
    let len = l.len();
    let mid = len / 2;
    if len % 2 == 0 {
        (l[(mid - 1)] + l[mid]) / 2
    } else {
        l[mid]
    }
}

#[allow(dead_code)]
pub fn parse_int(b: &[u8]) -> usize {
    b.iter().fold(0, |a, c| a * 10 + (c & 0x0f) as usize)
}

pub fn download_input(day: usize, year: usize, session: &str, filename: &str) -> Result<()> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    log::info!("Downloading: {}", url);
    let cookie_header = HeaderValue::from_str(&format!("session={}", session.trim()))
        .map_err(|err| eyre!("Err: {:?}", err))?;
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
    let _ = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?
        .write(text.as_bytes())?;
    Ok(())
}

#[allow(dead_code)]
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
pub fn six_neighbors(idx: [isize; 3]) -> impl Iterator<Item = [isize; 3]> {
    [
        [idx[0] - 1, idx[1], idx[2]],
        [idx[0], idx[1] - 1, idx[2]],
        [idx[0], idx[1], idx[2] - 1],
        [idx[0] + 1, idx[1], idx[2]],
        [idx[0], idx[1] + 1, idx[2]],
        [idx[0], idx[1], idx[2] + 1],
    ]
    .into_iter()
}

#[allow(dead_code)]
pub fn print_array(array: &Array2<usize>) {
    for row in array.axis_iter(Axis(0)) {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

pub fn trim_ascii_whitespace(x: &[u8]) -> &[u8] {
    let from = match x.iter().position(|x| !x.is_ascii_whitespace()) {
        Some(i) => i,
        None => return &x[0..0],
    };
    let to = x.iter().rposition(|x| !x.is_ascii_whitespace()).unwrap();
    &x[from..=to]
}

pub trait RangeIncExt {
    fn inside(&self, other: &Self) -> bool;
    fn inside_or_surrounding(&self, other: &Self) -> bool {
        self.inside(other) || other.inside(self)
    }

    fn overlaps(&self, other: &Self) -> bool;
    fn union(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;
    fn intersect(&self, other: &Self) -> Self;
    fn size(&self) -> usize;
}

impl<T> RangeIncExt for RangeInclusive<T>
where
    T: Ord + Copy + Sub<T>,
    <T as Sub<T>>::Output: TryInto<usize>,
    <<T as Sub<T>>::Output as TryInto<usize>>::Error: std::fmt::Debug,
{
    fn inside(&self, other: &Self) -> bool {
        other.contains(self.start()) && other.contains(self.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        other.contains(self.start())
            || other.contains(self.end())
            || self.contains(other.start())
            || self.contains(other.end())
    }

    fn union(&self, other: &Self) -> Option<Self>
    where
        Self: Sized,
    {
        self.overlaps(other)
            .then_some(*self.start().min(other.start())..=*self.end().max(other.end()))
    }

    fn intersect(&self, other: &Self) -> Self {
        *self.start().max(other.start())..=*self.end().min(other.end())
    }

    fn size(&self) -> usize {
        (*self.end() - *self.start()).try_into().unwrap()
    }
}

impl<T> RangeIncExt for Range<T>
where
    T: Ord + Copy + Sub<T>,
    <T as Sub<T>>::Output: TryInto<usize>,
    <<T as Sub<T>>::Output as TryInto<usize>>::Error: std::fmt::Debug,
{
    fn inside(&self, other: &Self) -> bool {
        other.contains(&self.start) && other.contains(&self.end)
    }

    fn overlaps(&self, other: &Self) -> bool {
        other.contains(&self.start)
            || other.contains(&self.end)
            || self.contains(&other.start)
            || self.contains(&other.end)
    }

    fn union(&self, other: &Self) -> Option<Self>
    where
        Self: Sized,
    {
        self.overlaps(other)
            .then_some(self.start.min(other.start)..self.end.max(other.end))
    }

    fn intersect(&self, other: &Self) -> Self {
        self.start.max(other.start)..self.end.min(other.end)
    }

    fn size(&self) -> usize {
        (self.end - self.start).try_into().unwrap()
    }
}
