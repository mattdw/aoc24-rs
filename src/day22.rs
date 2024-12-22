use crate::Day;
pub struct Day22 {}

fn mix(secret: &mut i64, mixer: i64) {
    *secret ^= mixer;
}

fn prune(secret: &mut i64) {
    *secret %= 16777216;
}

fn next(secret: i64) -> i64 {
    let mut s = secret;

    let mixer = s * 64;
    mix(&mut s, mixer);
    prune(&mut s);

    let mixer = s / 32;
    mix(&mut s, mixer);
    prune(&mut s);

    let mixer = s * 2048;
    mix(&mut s, mixer);
    prune(&mut s);

    s
}

impl Day<isize> for Day22 {
    fn part1(input: &str) -> isize {
        let inits = input.split_whitespace().map(|v| v.parse::<i64>().unwrap());

        inits
            .map(|n| {
                let mut curr = n as i64;
                for _ in 0..2000 {
                    curr = next(curr);
                }

                curr
            })
            .sum::<i64>() as isize
    }

    fn part2(input: &str) -> isize {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn next_secret() {
        let numbers = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];

        let mut prev = 123;
        for n in numbers {
            prev = next(prev);
            assert_eq!(n, prev);
        }
    }

    #[test]
    fn p1() {
        assert_eq!(37327623, Day22::part1("1 10 100 2024"));
    }
}
