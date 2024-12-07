macro_rules! check {
    ($name:ident, $struct:ty, $p1:literal, $p2:literal) => {
        mod $name {

            use aoc24_rs::*;

            #[tokio::test]
            async fn part1() {
                let input = fetch_input_s(stringify!($struct)).await.unwrap();
                let res = <$struct>::part1(&input);
                assert_eq!(res, $p1);
            }

            #[tokio::test]
            async fn part2() {
                let input = fetch_input_s(stringify!($struct)).await.unwrap();
                let res = <$struct>::part2(&input);
                assert_eq!(res, $p2);
            }
        }
    };
}

check!(day1, Day1, 2904518, 18650129);
check!(day7, Day7, 7710205485870, 20928985450275);
