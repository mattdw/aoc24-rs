use aoc24_rs::{fetch_input, Day, Day11};

#[tokio::main]
async fn main() {
    println!(
        "{:?}",
        Day11::part2(&fetch_input(11).await.expect("fetched input"))
    );
}
