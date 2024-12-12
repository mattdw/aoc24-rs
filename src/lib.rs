mod day01;
pub use day01::Day1;

mod day02;
pub use day02::Day2;

mod day07;
pub use day07::Day7;

mod day08;
pub use day08::Day8;

mod day09;
pub use day09::Day9;

mod day10;
pub use day10::Day10;

mod day11;
pub use day11::Day11;

mod day12;
pub use day12::Day12;

use std::{env, fmt::Debug};

use reqwest::header::COOKIE;

pub trait Day<T: Debug + Eq> {
    fn part1(input: &str) -> T;
    fn part2(input: &str) -> T;
}

pub async fn fetch_input(day: u8) -> Result<String, anyhow::Error> {
    let _ = tokio::fs::create_dir("inputs").await;
    let existing = tokio::fs::read_to_string(format!("inputs/{day}.txt")).await;

    if let Ok(s) = existing {
        return Ok(s);
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
