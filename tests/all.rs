macro_rules! check {
    ($name:ident, $struct:ty, $p1:literal, $p2:literal) => {
        mod $name {

            use aoc24_rs::*;

            #[test]
            fn part1() {
                let input = fetch_input_s(stringify!($struct)).unwrap();
                let res = <$struct>::part1(&input);
                assert_eq!($p1.to_string(), res.to_string());
            }

            #[test]
            fn part2() {
                let input = fetch_input_s(stringify!($struct)).unwrap();
                let res = <$struct>::part2(&input);
                assert_eq!($p2.to_string(), res.to_string());
            }
        }
    };
}

check!(day01, Day1, 2904518, 18650129);
check!(day02, Day2, 236, 308);
check!(day03, Day3, 174960292, 56275602);
check!(day04, Day4, 2536, 1875);
check!(day05, Day5, 5268, 5799);
check!(day06, Day6, 4982, 1663);
check!(day07, Day7, 7710205485870_i64, 20928985450275_i64);
check!(day08, Day8, 252, 839);
check!(day09, Day9, 6446899523367_i64, 6478232739671_i64);
check!(day10, Day10, 535, 1186);
check!(day11, Day11, 186996, 221683913164898_i64);
check!(day12, Day12, 1477762, 923480);
check!(day13, Day13, 33427, 91649162972270_i64);
check!(day14, Day14, 224969976, 7892);
check!(day15, Day15, 1446158, 1446175);
// part 2 runs in > .3s
check!(day16, Day16, 83444, 483);
check!(day17, Day17, "4,6,1,4,2,1,3,1,6", 202366627359274_i64);
check!(day18, Day18, "384", "36,10");
check!(day19, Day19, 290, 712058625427487_i64);

check!(day21, Day21, 184718, 228800606998554_i64);

// part 2 runs in > .3s
check!(day22, Day22, 15335183969_i64, 1696);

check!(day24, Day24, 51837135476040_i64, "_");
check!(day25, Day25, 3483, -1);

//-- Car races, collisions and cheats
//-- I'm just not getting the right answer
// check!(day20, Day20, 1406, -1);

// Recursive keypads - can't see how to restructure
// as recursive algo that deals with lengths rather than strings?

// -- LAN Party, filled mem doing graph clustering a naive way
// check!(day23, Day23, "1476", "-1");
