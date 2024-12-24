use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Write},
};

use crate::Day;

pub struct Day23 {
    connections: HashMap<Name, HashSet<Name>>,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug)]
struct Name([u8; 2]);

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.0[0] as char)?;
        f.write_char(self.0[1] as char)?;

        Ok(())
    }
}

impl Day23 {
    fn parse(input: &str) -> Day23 {
        let mut connections = HashMap::<Name, HashSet<Name>>::new();

        input.split_ascii_whitespace().for_each(|line| {
            let (left, right) = line.split_once('-').unwrap();

            let mut bleft: [u8; 2] = [0; 2];
            let mut bright: [u8; 2] = [0; 2];
            bleft.copy_from_slice(&left.as_bytes()[..2]);
            bright.copy_from_slice(&right.as_bytes()[..2]);

            connections
                .entry(Name(bleft))
                .or_default()
                .insert(Name(bright));

            connections
                .entry(Name(bright))
                .or_default()
                .insert(Name(bleft));
        });

        Day23 { connections }
    }

    fn find_threes(d: &Day23, n: usize) -> HashSet<Vec<Name>> {
        let mut out = HashSet::new();
        for a in d.connections.keys() {
            let a_conns = d.connections.get(a).unwrap();

            for b in d.connections.keys() {
                if a >= b {
                    continue;
                }

                if !a_conns.contains(b) {
                    continue;
                }
                let b_conns = d.connections.get(b).unwrap();

                for c in d.connections.keys() {
                    if c <= b || c <= a {
                        continue;
                    }

                    if a_conns.contains(c) && b_conns.contains(c) {
                        out.insert(vec![*a, *b, *c]);
                    }
                }
            }
        }

        out
    }

    fn all_subsets(item: &Vec<Name>) -> Vec<Vec<Name>> {
        let mut subs = vec![];
        if item.len() < 4 {
            return vec![];
        }

        for i in 0..item.len() {
            let sub = [&item[0..i], &item[i + 1..]].concat();
            subs.extend(Self::all_subsets(&sub));
            subs.push(sub);
        }
        subs
    }

    // this works but uses all my memory
    fn find_clusters(d: &Day23) -> Vec<Vec<Name>> {
        let sets_of_outgoing = d.connections.iter().map(|(k, v)| {
            let mut v = v.clone();
            v.insert(*k);

            let mut v2 = Vec::from_iter(v.iter().copied());
            v2.sort();

            v2
        });

        let groups: Vec<_> = sets_of_outgoing
            .fold(HashMap::<Vec<Name>, usize>::new(), |mut acc, item| {
                // acc.insert(item);
                *acc.entry(item.clone()).or_default() += 1;
                for item in Self::all_subsets(&item) {
                    let mut l = item.len();
                    let e = acc.entry(item.clone()).or_default();
                    *e += 1;
                    if e > &mut l {
                        acc.remove(&item);
                    }
                }

                acc
            })
            .iter()
            // this is the magic - as many members in the set as
            // seen this particular set
            .filter(|(s, c)| s.len() == **c)
            .map(|s| s.0.clone())
            .collect();

        groups
    }
}

impl Day<String> for Day23 {
    fn part1(input: &str) -> String {
        let d = Day23::parse(input);
        let triples = Day23::find_threes(&d, 3);

        let res = triples
            .iter()
            .filter(|t| t.iter().any(|name| name.0[0] == b't'))
            .count();

        format!("{}", res)
    }

    fn part2(input: &str) -> String {
        let d = Day23::parse(input);
        let cs = Day23::find_clusters(&d);

        let res = cs.iter().max_by_key(|v| v.len()).unwrap();

        res.iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<_>>()
            .join("")
    }
}

#[cfg(test)]
mod test {
    use crate::Day23;

    const TEST_INPUT: &'static str = "
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
    ";

    #[test]
    fn triples() {
        assert_eq!(12, Day23::find_threes(&Day23::parse(TEST_INPUT), 3).len());
    }

    #[test]
    fn clusters() {
        let d = Day23::parse(TEST_INPUT);
        let cs = Day23::find_clusters(&d);

        for cl in &cs {
            for n in cl {
                println!("{}", n);
            }
            println!("");
        }

        let res = cs.iter().max_by_key(|v| v.len()).unwrap();
        assert_eq!(4, res.len());
        assert_eq!(
            "codekata",
            res.iter()
                .map(|v| format!("{}", v))
                .collect::<Vec<_>>()
                .join("")
        );
    }
}
