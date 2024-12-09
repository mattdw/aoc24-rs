mod day1;
pub use day1::Day1;

mod day7;
pub use day7::Day7;

mod day8;
pub use day8::Day8;

mod day9;
pub use day9::Day9;

use std::{env, fmt::Debug};

use reqwest::header::COOKIE;

pub trait Day<T: Debug + Eq> {
    fn part1(input: &str) -> T;
    fn part2(input: &str) -> T;
}

pub async fn fetch_input(day: u8) -> Result<String, anyhow::Error> {
    let _ = tokio::fs::create_dir("inputs").await;
    let existing = tokio::fs::read_to_string(format!("inputs/{day}.txt")).await;

    match existing {
        Ok(s) => return Ok(s),
        Err(_) => (),
    }

    let session = env::var("SESSION").expect("SESSION env var is required to fetch input");
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("https://adventofcode.com/2024/day/{day}/input"))
        .header(COOKIE, format!("session={session}"))
        .send()
        .await?;

    let text: String = resp.text().await?;
    tokio::fs::write(format!("inputs/{day}.txt"), &text).await?;

    Ok(text)
}

pub async fn fetch_input_s(day: &str) -> Result<String, anyhow::Error> {
    let s: u8 = day
        .trim_matches(['d', 'a', 'y', 'D'])
        .parse()
        .expect("ends with a number");
    fetch_input(s).await
}
