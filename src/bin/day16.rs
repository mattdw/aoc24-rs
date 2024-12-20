use aoc24_rs::fetch_input;
use aoc24_rs::{Day, Day16};

fn main() {
    let inp = fetch_input(16).unwrap();
    assert_eq!(483, Day16::part2(&inp));
}
