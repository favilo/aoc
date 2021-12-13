use std::{fs::OpenOptions, io::Write};

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
