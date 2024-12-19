use std::collections::HashMap;

use crate::Day;

pub struct Day19 {
    patterns: Vec<String>,
    designs: Vec<String>,
}

impl Day19 {
    fn parse(input: &str) -> Day19 {
        let mut patterns = None;
        let mut designs = Vec::<String>::new();
        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if line.contains(",") {
                patterns = Some(
                    line.split(", ")
                        .map(|v| v.to_string())
                        .filter(|v| !v.is_empty())
                        .collect::<Vec<_>>(),
                );
            } else {
                designs.push(line.to_string());
            }
        }

        let mut patterns = patterns.expect("no patterns");
        patterns.sort_by_key(|v| -(v.len() as i64));

        Self { patterns, designs }
    }

    fn count_recur<'a>(
        target: &'a str,
        options: &'a [String],
        all: bool,
        cache: &mut HashMap<&'a str, usize>,
    ) -> usize {
        let mut count = 0;

        if let Some(e) = cache.get(&target) {
            return *e;
        }

        for m in options.iter().filter(|v| v.len() <= target.len()) {
            if target == m {
                count += 1;

                if !all {
                    return 1;
                }
                continue;
            }

            if target.starts_with(m) {
                let len = m.len();
                let res = Self::count_recur(&target[len..], options, all, cache);
                cache.insert(&target[len..], res);
                count += res;
                if !all && res > 0 {
                    return 1;
                }
            }
        }

        cache.insert(target, count);
        count
    }
}

impl Day<usize> for Day19 {
    fn part1(input: &str) -> usize {
        let d = Day19::parse(input);
        d.designs
            .iter()
            .map(|design| {
                let mut cache = HashMap::new();
                Day19::count_recur(design, d.patterns.as_slice(), false, &mut cache)
            })
            .sum::<usize>()
    }

    fn part2(input: &str) -> usize {
        let d = Day19::parse(input);
        d.designs
            .iter()
            .map(|design| {
                let mut cache = HashMap::new();
                Day19::count_recur(design, d.patterns.as_slice(), true, &mut cache)
            })
            .sum::<usize>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
    ";

    #[test]
    fn t1() {
        assert_eq!(6, Day19::part1(TEST_INPUT));
    }

    #[test]
    fn t2() {
        assert_eq!(16, Day19::part2(TEST_INPUT));
    }
}
