use aoc24_rs::{fetch_input, Day, Day17};

#[tokio::main]
async fn main() {
    let input = fetch_input(17).await.unwrap();
    assert_eq!("", Day17::part2(&input));
}
