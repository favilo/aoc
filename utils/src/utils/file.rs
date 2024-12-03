use miette::{IntoDiagnostic, Result, WrapErr};
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, CONTENT_TYPE, COOKIE},
    redirect::Policy,
};
use std::{
    fs::{create_dir_all, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

pub fn download_input(
    day: usize,
    year: usize,
    session: &str,
    filename: impl AsRef<Path>,
) -> Result<()> {
    let text = fetch_input(year, day, session)?;

    log::info!("Saving file: {}", filename.as_ref().display());
    save_input(filename, text)?;
    Ok(())
}

fn save_input(filename: impl AsRef<Path>, text: String) -> Result<(), miette::Error> {
    create_dir_all(
        filename
            .as_ref()
            .parent()
            .ok_or_else(|| miette::miette!("file {} has no parent", filename.as_ref().display()))?,
    )
    .into_diagnostic()?;
    let _ = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)
        .into_diagnostic()
        .wrap_err("failed to open file")?
        .write(text.as_bytes())
        .into_diagnostic()
        .wrap_err("failed to write to file")?;
    Ok(())
}

fn fetch_input(year: usize, day: usize, session: &str) -> Result<String, miette::Error> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    log::info!("Downloading: {}", url);
    let cookie_header = HeaderValue::from_str(&format!("session={}", session.trim()))
        .into_diagnostic()
        .wrap_err("invalid cookie header")?;
    let content_header = HeaderValue::from_str("text/plain")
        .into_diagnostic()
        .wrap_err("invalid content header")?;
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, cookie_header);
    headers.insert(CONTENT_TYPE, content_header);
    let client = Client::builder()
        .default_headers(headers)
        .redirect(Policy::none())
        .build()
        .into_diagnostic()
        .wrap_err("failed to build client")?;
    let text = client
        .get(&url)
        .send()
        .and_then(reqwest::blocking::Response::text)
        .into_diagnostic()
        .wrap_err("failed to download input")?;
    Ok(text)
}

pub fn get_input_path(year: usize, day: usize) -> Result<PathBuf, miette::Report> {
    let input_path = PathBuf::from("input")
        .join(year.to_string())
        .join(format!("day{day:02}.txt"));
    let env_path = dotenv::dotenv()
        .into_diagnostic()
        .wrap_err("loading .env file")?;
    let parent_path = env_path
        .parent()
        .ok_or_else(|| miette::miette!("no parent path for .env file"))?;
    log::info!("Loaded .env file from {}", parent_path.display());
    let input_full_path = if input_path.exists() {
        input_path
    } else {
        let joined_path = parent_path.join(&input_path);
        log::info!("Joined path: {}", joined_path.display());
        joined_path.clone()
    };
    Ok(input_full_path)
}
