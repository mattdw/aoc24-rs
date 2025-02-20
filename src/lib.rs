mod intmap;
mod iterext;

mod day01;
pub use day01::Day1;

mod day02;
pub use day02::Day2;

mod day03;
pub use day03::Day3;

mod day04;
pub use day04::Day4;

mod day05;
pub use day05::Day5;

mod day06;
pub use day06::Day6;

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

mod day13;
pub use day13::Day13;

mod day14;
pub use day14::Day14;

mod day15;
pub use day15::Day15;

mod day16;
pub use day16::Day16;

mod day17;
pub use day17::Day17;

mod day18;
pub use day18::Day18;

mod day19;
pub use day19::Day19;

mod day20;
pub use day20::Day20;

mod day21;
pub use day21::Day21;

mod day22;
pub use day22::Day22;

mod day23;
pub use day23::Day23;

mod day24;
pub use day24::Day24;

mod day25;
pub use day25::Day25;

use std::{env, fmt::Debug};

use reqwest::header::COOKIE;

pub trait Day<T: Debug + Eq> {
    fn part1(input: &str) -> T;
    fn part2(input: &str) -> T;
}

pub fn fetch_input(day: u8) -> Result<String, anyhow::Error> {
    let _ = std::fs::create_dir("inputs");
    let existing = std::fs::read_to_string(format!("inputs/{day}.txt"));

    if let Ok(s) = existing {
        return Ok(s);
    }

    let session = env::var("SESSION").expect("SESSION env var is required to fetch input");
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(format!("https://adventofcode.com/2024/day/{day}/input"))
        .header(COOKIE, format!("session={session}"))
        .send()?;

    let text: String = resp.text()?;
    std::fs::write(format!("inputs/{day}.txt"), &text)?;

    Ok(text)
}

pub fn fetch_input_s(day: &str) -> Result<String, anyhow::Error> {
    let s: u8 = day
        .trim_matches(['d', 'a', 'y', 'D'])
        .parse()
        .expect("ends with a number");
    fetch_input(s)
}
