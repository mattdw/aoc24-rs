use std::collections::HashMap;

use crate::Day;
pub struct Day22 {}

/*

--- Part Two ---

Of course, the secret numbers aren't the prices each buyer is offering! That would be ridiculous. Instead, the prices the buyer offers are just the ones digit of each of their secret numbers.

So, if a buyer starts with a secret number of 123, that buyer's first ten prices would be:

3 (from 123)
0 (from 15887950)
6 (from 16495136)
5 (etc.)
4
4
6
4
4
2

This price is the number of bananas that buyer is offering in exchange for your information about a new hiding spot. However, you still don't speak monkey, so you can't negotiate with the buyers directly. The Historian speaks a little, but not enough to negotiate; instead, he can ask another monkey to negotiate on your behalf.

Unfortunately, the monkey only knows how to decide when to sell by looking at the changes in price. Specifically, the monkey will only look for a specific sequence of four consecutive changes in price, then immediately sell when it sees that sequence.

So, if a buyer starts with a secret number of 123, that buyer's first ten secret numbers, prices, and the associated changes would be:

     123: 3
15887950: 0 (-3)
16495136: 6 (6)
  527345: 5 (-1)
  704524: 4 (-1)
 1553684: 4 (0)
12683156: 6 (2)
11100544: 4 (-2)
12249484: 4 (0)
 7753432: 2 (-2)

Note that the first price has no associated change because there was no previous price to compare it with.

In this short example, within just these first few prices, the highest price will be 6, so it would be nice to give the monkey instructions that would make it sell at that time. The first 6 occurs after only two changes, so there's no way to instruct the monkey to sell then, but the second 6 occurs after the changes -1,-1,0,2. So, if you gave the monkey that sequence of changes, it would wait until the first time it sees that sequence and then immediately sell your hiding spot information at the current price, winning you 6 bananas.

Each buyer only wants to buy one hiding spot, so after the hiding spot is sold, the monkey will move on to the next buyer. If the monkey never hears that sequence of price changes from a buyer, the monkey will never sell, and will instead just move on to the next buyer.

Worse, you can only give the monkey a single sequence of four price changes to look for. You can't change the sequence between buyers.

You're going to need as many bananas as possible, so you'll need to determine which sequence of four price changes will cause the monkey to get you the most bananas overall. Each buyer is going to generate 2000 secret numbers after their initial secret number, so, for each buyer, you'll have 2000 price changes in which your sequence can occur.

Suppose the initial secret number of each buyer is:

1
2
3
2024

There are many sequences of four price changes you could tell the monkey, but for these four buyers, the sequence that will get you the most bananas is -2,1,-1,3. Using that sequence, the monkey will make the following sales:

For the buyer with an initial secret number of 1, changes -2,1,-1,3 first occur when the price is 7.
For the buyer with initial secret 2, changes -2,1,-1,3 first occur when the price is 7.
For the buyer with initial secret 3, the change sequence -2,1,-1,3 does not occur in the first 2000 changes.
For the buyer starting with 2024, changes -2,1,-1,3 first occur when the price is 9.
So, by asking the monkey to sell the first time each buyer's prices go down 2, then up 1, then down 1, then up 3, you would get 23 (7 + 7 + 9) bananas!

Figure out the best sequence to tell the monkey so that by looking for that same sequence of changes in every buyer's future prices, you get the most bananas in total. What is the most bananas you can get?



*/

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
        let inits = input.split_whitespace().map(|v| v.parse::<i64>().unwrap());

        let number_sets: Vec<Vec<_>> = inits
            .map(|n| {
                let mut curr = n as i64;
                let mut v = vec![n];
                for _ in 0..2000 {
                    curr = next(curr);
                    v.push(curr);
                }

                v
            })
            .collect();

        let mut caches = HashMap::<[i8; 4], i32>::new();

        for set in number_sets {
            let mut set_cache = HashMap::<[i8; 4], i32>::new();
            let mut history: [i8; 4] = [
                (set[0] % 10) as i8,
                (set[1] % 10) as i8,
                (set[2] % 10) as i8,
                (set[3] % 10) as i8,
            ];
            let mut deltas: [i8; 4] = [
                0,
                history[1] - history[0],
                history[2] - history[1],
                history[3] - history[2],
            ];

            for n in set.iter().skip(4) {
                history.rotate_left(1);
                history[3] = (n % 10) as i8;
                deltas.rotate_left(1);
                deltas[3] = history[3] - history[2];

                set_cache.entry(deltas).or_insert(*n as i32 % 10);
            }

            for (k, v) in set_cache {
                caches
                    .entry(k)
                    .and_modify(|old_v| {
                        *old_v += v;
                    })
                    .or_insert(v);
            }
        }

        *caches.iter().max_by_key(|(_, v)| **v).unwrap().1 as isize
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

    #[test]
    fn p2() {
        assert_eq!(23, Day22::part2("1 2 3 2024"));
    }
}
