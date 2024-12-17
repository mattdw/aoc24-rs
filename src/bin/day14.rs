use aoc24_rs::{fetch_input, Day};

#[tokio::main]
async fn main() {
    let i = fetch_input(14).await.unwrap();
    aoc24_rs::Day14::part2(&i);
}
