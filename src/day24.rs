use std::{
    collections::{HashMap, HashSet},
    fmt::Write,
};

use crate::Day;
use regex::Regex;

fn func_re() -> Regex {
    Regex::new(r"(?<left>[\w\d]+) (?<op>XOR|OR|AND) (?<right>[\w\d]+) -> (?<out>[\w\d]+)").unwrap()
}

type Name = String;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum Node {
    Const(bool),
    XOR(Name, Name),
    OR(Name, Name),
    AND(Name, Name),
}

fn first(s: &str) -> char {
    s.chars().next().unwrap_or('?')
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Const(v) => f.write_str(if *v { "T" } else { "F" }),
            Node::XOR(l, r) => {
                write!(f, "^",)
            }
            Node::OR(l, r) => {
                write!(f, "|")
            }
            Node::AND(l, r) => {
                write!(f, "&")
            }
        }
    }
}

pub struct Day24 {
    table: HashMap<Name, Node>,
    zmax: usize,
}

impl Day24 {
    fn parse(input: &str) -> Self {
        let (inputs, connects) = input.split_once("\n\n").unwrap();

        let mut table = HashMap::<Name, Node>::new();

        for line in inputs.trim().lines() {
            let (left, right) = line.trim().split_once(": ").unwrap();
            table.insert(
                left.to_string(),
                Node::Const(if right == "1" { true } else { false }),
            );
        }

        let pat = func_re();
        let mut zmax = 0;
        for line in connects.trim().lines() {
            // println!("|{line}|");
            let captures = pat.captures(line).unwrap();
            let left = captures["left"].to_string();
            let right = captures["right"].to_string();
            let node = match &captures["op"] {
                "XOR" => Node::XOR(left, right),
                "OR" => Node::OR(left, right),
                "AND" => Node::AND(left, right),
                _ => panic!(),
            };

            let out = captures["out"].to_string();

            if out.starts_with("z") {
                let num: usize = out[1..].parse().unwrap();
                zmax = zmax.max(num);
            }
            table.insert(out, node);
        }

        Day24 { table, zmax }
    }

    fn combine(n: Node, l: bool, r: bool) -> bool {
        match n {
            Node::AND(_, _) => l && r,
            Node::XOR(_, _) => l != r,
            Node::OR(_, _) => l || r,
            _ => panic!(),
        }
    }

    fn eval(&self, name: &str) -> (bool, Vec<(Name, Node)>) {
        let node = self.table.get(name).unwrap().clone();

        match node.clone() {
            Node::Const(tf) => (tf, vec![]),
            Node::XOR(l, r) | Node::AND(l, r) | Node::OR(l, r) => {
                let (lres, lcontrib) = self.eval(&l);
                let (rres, rcontrib) = self.eval(&r);

                let res = Self::combine(node.clone(), lres, rres);

                let mut contribs = Vec::new();
                contribs.push((name.to_string(), node));
                contribs.extend(lcontrib);
                contribs.extend(rcontrib);
                (res, contribs)
            }
        }
    }

    fn evaluate(&self) -> usize {
        let mut res = 0;
        for digit in 0..=self.zmax {
            let name = format!("z{:02}", self.zmax - digit);
            let digit = self.eval(&name).0;
            res <<= 1;
            res |= if digit { 1 } else { 0 };

            // digits.push(self.eval(&name).0);
        }

        // digits.reverse();

        res
    }

    fn evaluate_trace(&self) -> (u64, Vec<Vec<(Name, Node)>>) {
        let mut res = 0;
        let mut traces = vec![];
        for digit in 0..=self.zmax {
            let name = format!("z{:02}", self.zmax - digit);
            let (digit, trace) = self.eval(&name);
            traces.push(trace);
            res <<= 1;
            res |= if digit { 1 } else { 0 };

            // digits.push(self.eval(&name).0);
        }

        // digits.reverse();

        (res, traces)
    }

    fn visualise(&self) -> Vec<String> {
        let traces = Vec::<String>::new();
        for digit in 0..=self.zmax {
            let name = format!("z{:02}", self.zmax - digit);
            let (digit, trace) = self.eval(&name);
        }

        traces
    }
}

impl Day<String> for Day24 {
    fn part1(input: &str) -> String {
        let d = Day24::parse(input);

        let out = d.evaluate();

        format!("{}", out)
    }

    fn part2(input: &str) -> String {
        let mut d = Day24::parse(input);

        // let z14 = d.table.get("z14").unwrap().clone();
        // let z22 = d.table.get("z22").unwrap().clone();
        // let z31 = d.table.get("z31").unwrap().clone();
        // let z35 = d.table.get("z35").unwrap().clone();
        // d.table.insert("z14".to_owned(), z31);
        // d.table.insert("z22".to_owned(), z14);
        // d.table.insert("z31".to_owned(), z35);
        // d.table.insert("z35".to_owned(), z22);

        let xnames = (0..=44).map(|i| format!("x{i:02}")).collect::<Vec<_>>();
        let ynames = (0..=44).map(|i| format!("y{i:02}")).collect::<Vec<_>>();

        let mut good_gates = HashSet::<Name>::new();
        let mut bad_gates = Vec::<HashSet<Name>>::new();

        for z in 0..=44 {
            for x in 0..=44 {
                if x == z {
                    d.table.insert(xnames[x].clone(), Node::Const(true));
                } else {
                    d.table.insert(xnames[x].clone(), Node::Const(false));
                }
            }

            for y in 0..=44 {
                if y == z {
                    d.table.insert(ynames[y].clone(), Node::Const(true));
                } else {
                    d.table.insert(ynames[y].clone(), Node::Const(false));
                }
            }

            let x = 1_u64 << z;
            // let y = 1_u64 << z;
            let y = 1_u64 << z;
            let expected = x + y;

            let (d, trace) = d.evaluate_trace();
            println!("{z}: {expected} / got {d} = {x} + {y} ");

            if expected == d {
                good_gates.extend(trace[z].iter().map(|(n, _)| n.clone()));
            } else {
                let set = trace[z].iter().map(|(n, _)| n.clone()).collect();
                bad_gates.push(set);
            }
        }

        // bad_gates = bad_gates.difference(&good_gates).cloned().collect();

        // println!("{:?}", bad_gates);

        println!("found {} bad digits", bad_gates.len());

        let mut all_intersect = HashSet::new();

        for s in bad_gates {
            all_intersect = all_intersect.intersection(&s).cloned().collect();
        }
        println!("all intersect: {:?}", all_intersect);

        // for s1 in &bad_gates {
        //     for s2 in &bad_gates {
        //         println!("{:?}", s1.intersection(s2));
        //     }
        // }

        "-".to_string()

        /*

            from visual inspections
            Z10
            Z14
            Z31

            from 1-bit test
            13/14
            30
            34
            35

            14
            22
            31
            35
        */
    }
}

#[cfg(test)]
mod test {
    use crate::fetch_input;

    use super::*;

    const TEST_INPUT: &'static str = "
        x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
    ";

    #[test]
    fn parse_test() {
        let d = Day24::parse(TEST_INPUT);

        assert_eq!(d.zmax, 12);
        assert_eq!(
            d.table.get("pbm").unwrap(),
            &Node::AND("y01".to_string(), "x02".to_string())
        );
    }

    #[test]
    fn eval_test() {
        let mut d = Day24::parse(&TEST_INPUT);

        let out = d.evaluate();

        let outs = format!("{:b}", out);

        assert_eq!("11111101000", &outs);
        assert_eq!(2024, usize::from_str_radix(&outs, 2).unwrap());
    }

    #[test]
    fn vis() {
        let mut d = Day24::parse(&fetch_input(24).unwrap());

        let (out, traces) = d.evaluate_trace();

        for (idx, t) in traces.iter().enumerate() {
            println!(
                "{idx} {}",
                &t.iter()
                    .map(|(a, b)| format!("{}", b.to_string()))
                    .collect::<Vec<String>>()
                    .join("")
            )
        }
    }
}
