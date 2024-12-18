use crate::{intmap::IntMap, Day};

fn parse(input: &str) -> IntMap<char> {
    IntMap::from_raw(input, |c| c)
}

fn gather(
    m: &IntMap<char>,
    pos: (isize, isize),
    dir: (isize, isize),
    count: usize,
) -> Option<Vec<char>> {
    let mut chars = vec![];
    for i in 0..count {
        if let Some(c) = m.get((pos.0 + dir.0 * i as isize, pos.1 + dir.1 * i as isize)) {
            chars.push(*c);
        } else {
            return None;
        }
    }

    Some(chars)
}

pub struct Day4 {}

impl Day<i64> for Day4 {
    fn part1(input: &str) -> i64 {
        let m = parse(input);
        let mut count = 0;

        for (idx, &c) in m.cells.iter().enumerate() {
            if c != 'X' {
                continue;
            }

            for &direction in &[
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                let res = gather(&m, m.idx_to_pt(idx as isize), direction, 4);
                if res == Some(vec!['X', 'M', 'A', 'S']) {
                    println!("{:?}", res);
                    count += 1;
                }
            }
        }

        count
    }

    fn part2(input: &str) -> i64 {
        let m = parse(input);
        let mut count = 0;

        for (idx, &c) in m.cells.iter().enumerate() {
            if c != 'A' {
                continue;
            }

            let co = m.idx_to_pt(idx as isize);

            let runs = &[
                ((-1, -1), (1, 1)),
                ((1, 1), (-1, -1)),
                ((1, -1), (-1, 1)),
                ((-1, 1), (1, -1)),
            ]
            .map(|(start, dir)| gather(&m, (co.0 + start.0, co.1 + start.1), dir, 3));

            if runs
                .iter()
                .filter(|v| {
                    if let Some(v) = v {
                        v == &vec!['M', 'A', 'S']
                    } else {
                        false
                    }
                })
                .count()
                >= 2
            {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
    ";

    #[test]
    fn parsing() {
        let m = parse(TEST_INPUT);

        assert_eq!(m.width, 10);
        assert_eq!(m.height, 10);

        assert_eq!(m.get((2, 1)), Some(&'A'));
        assert_eq!(m.get((0, 2)), Some(&'A'));
        assert_eq!(m.get((9, 9)), Some(&'X'));
    }

    #[test]
    fn test_p1() {
        assert_eq!(18, Day4::part1(TEST_INPUT));
    }
}
