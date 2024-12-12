macro_rules! check {
    ($name:ident, $struct:ty, $p1:literal, $p2:literal) => {
        mod $name {

            use aoc24_rs::*;

            #[tokio::test]
            async fn part1() {
                let input = fetch_input_s(stringify!($struct)).await.unwrap();
                let res = <$struct>::part1(&input);
                assert_eq!($p1, res);
            }

            #[tokio::test]
            async fn part2() {
                let input = fetch_input_s(stringify!($struct)).await.unwrap();
                let res = <$struct>::part2(&input);
                assert_eq!($p2, res);
            }
        }
    };
}

check!(day1, Day1, 2904518, 18650129);
check!(day7, Day7, 7710205485870, 20928985450275);
check!(day8, Day8, 252, 839);
check!(day9, Day9, 6446899523367, 6478232739671);
check!(day10, Day10, 535, 1186);
check!(day11, Day11, 186996, 221683913164898);
check!(day12, Day12, -1, -1);
