use aoc24_rs::fetch_input;
use aoc24_rs::{Day, Day16};

#[tokio::main]
async fn main() {
    let inp = fetch_input(16).await.unwrap();
    assert_eq!(483, Day16::part2(&inp));
}
