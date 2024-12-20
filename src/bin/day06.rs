use aoc24_rs::{fetch_input, Day, Day6};

fn main() {
    let input = fetch_input(6).unwrap();
    assert_eq!(1663, Day6::part2(&input));
}
