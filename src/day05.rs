use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
};

use crate::Day;

type Page = u16;

fn parse(input: &str) -> Day5 {
    let mut depends_on = HashMap::<Page, HashSet<Page>>::new();
    let mut depended_on = HashMap::<Page, HashSet<Page>>::new();

    let mut updates = vec![];

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        if let Some((left, right)) = line.trim().split_once("|") {
            let l: Page = left.parse().unwrap();
            let r: Page = right.parse().unwrap();
            depends_on.entry(r).or_default().insert(l);
            depended_on.entry(l).or_default().insert(r);
            continue;
        }

        let nums = line.trim().split(",").map(|v| v.parse::<Page>().unwrap());

        updates.push(nums.collect());
    }

    Day5 {
        depends_on,
        depended_on,
        updates,
    }
}

fn fetch_all_filtered(
    m: &HashMap<Page, HashSet<Page>>,
    n: Page,
    including: &HashSet<Page>,
) -> HashSet<Page> {
    let mut out = HashSet::<Page>::new();
    let mut q = VecDeque::<Page>::new();

    q.push_back(n);

    loop {
        let Some(page) = q.pop_front() else {
            break;
        };

        if !including.contains(&page) {
            continue;
        }

        let deps = m.get(&page);
        if let Some(deps) = deps {
            for &dep in deps {
                if !out.contains(&dep) {
                    q.push_back(dep);
                    out.insert(dep);
                }
            }
        }
    }

    out
}

fn valid_order(u: &[Page], d: &Day5) -> bool {
    let update_pages = HashSet::<Page>::from_iter(u.iter().copied());
    let mut seen = HashSet::<Page>::new();

    for &page in u {
        let deps = fetch_all_filtered(&d.depended_on, page, &update_pages);
        if deps.iter().any(|p| seen.contains(p)) {
            return false;
        }
        seen.insert(page);
    }

    true
}

fn make_checksum<'a, T: Iterator<Item = &'a [Page]>>(updates: T) -> i64 {
    updates
        .map(|u| {
            assert_eq!(1, u.len() % 2);
            u[u.len() / 2] as i64
        })
        .sum()
}

fn reorder_update(u: &[Page], d: &Day5) -> Vec<Page> {
    let numbers = HashSet::from_iter(u.iter().copied());
    let mut u = Vec::from(u);

    u.sort_by(|a, b| {
        if fetch_all_filtered(&d.depends_on, *a, &numbers).contains(b) {
            return Ordering::Greater;
        }

        if fetch_all_filtered(&d.depends_on, *b, &numbers).contains(a) {
            return Ordering::Less;
        }

        Ordering::Equal
    });

    u
}

pub struct Day5 {
    depends_on: HashMap<Page, HashSet<Page>>,
    depended_on: HashMap<Page, HashSet<Page>>,

    updates: Vec<Vec<Page>>,
}

impl Day<i64> for Day5 {
    fn part1(input: &str) -> i64 {
        let d = parse(input);

        let valid = d
            .updates
            .iter()
            .filter(|&u| valid_order(u.as_slice(), &d))
            .map(|v| v.as_slice());

        make_checksum(valid)
    }

    fn part2(input: &str) -> i64 {
        let d = parse(input);
        let reordered: Vec<Vec<Page>> = d
            .updates
            .iter()
            .filter(|&u| !valid_order(u.as_slice(), &d))
            .map(|v| reorder_update(v, &d))
            .collect();

        make_checksum(reordered.iter().map(|v| v.as_slice()))
    }
}
