use crate::Day;

fn parse(input: &str) -> Day25 {
    // split on blank lines
    let sections = input.trim().split("\n\n");

    let mut locks: Vec<[u8; 5]> = vec![];
    let mut keys: Vec<[u8; 5]> = vec![];

    for mech in sections {
        let mech = mech.trim();

        let is_lock = mech.starts_with("#####");

        let mut heights: [u8; 5] = [0; 5];
        for (height, line) in mech.lines().skip(1).take(5).enumerate() {
            let line = line.trim();
            // println!("{}", line);
            for pos in 0..5 {
                if line.as_bytes()[pos] == b'.' {
                    continue;
                }
                if is_lock {
                    heights[pos] = heights[pos].max(1 + height as u8);
                } else {
                    heights[pos] = heights[pos].max(5 - height as u8);
                }
            }
        }

        if is_lock {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    // dbg!((&locks, &keys));

    Day25 { locks, keys }
}

fn overlaps(lock: &[u8; 5], key: &[u8; 5]) -> bool {
    for i in 0..5 {
        if lock[i] + key[i] > 5 {
            return true;
        }
    }

    false
}

#[derive(Debug, Clone)]
pub struct Day25 {
    locks: Vec<[u8; 5]>,
    keys: Vec<[u8; 5]>,
}

impl Day<isize> for Day25 {
    fn part1(input: &str) -> isize {
        let d = parse(input);

        let mut count = 0;
        for lock in d.locks.iter() {
            for key in d.keys.iter() {
                if !overlaps(lock, key) {
                    count += 1;
                }
            }
        }

        count
    }

    fn part2(input: &str) -> isize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "
        #####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####
    ";

    #[test]
    fn parse_test() {
        let d = parse(TEST_INPUT);

        assert_eq!(2, d.locks.len());
        assert_eq!(3, d.keys.len());
        assert_eq!([0, 5, 3, 4, 3], d.locks[0]);
        assert_eq!([5, 0, 2, 1, 3], d.keys[0]);
    }

    #[test]
    fn p1_test() {
        assert_eq!(3, Day25::part1(TEST_INPUT));
    }
}
