use crate::Day;

pub struct Day9 {}

type Packet = (u64, Option<u64>);

#[allow(dead_code)]
fn disk_size(map: &str) -> u64 {
    map.trim()
        .chars()
        .map(|v| v.to_digit(10).expect("should be a digit") as u64)
        .sum()
}

fn expand_map(map: &str) -> Vec<Packet> {
    map.trim()
        .chars()
        .map(|c| c.to_digit(10).expect("should be a digit"))
        .enumerate()
        .map(|(idx, val)| {
            if idx % 2 == 0 {
                (val as u64, Some((idx / 2) as u64))
            } else {
                (val as u64, None)
            }
        })
        .filter(|(len, _)| *len > 0)
        .collect()
}

fn repack(mut disk: Vec<Packet>) -> Vec<Packet> {
    let mut left = 0;
    let mut right = disk.len() - 1;

    loop {
        while disk[left].0 == 0 {
            disk.remove(left);
        }
        while disk[left].1.is_some() {
            left += 1
        }
        while disk[right].1.is_none() || disk[right].0 == 0 {
            right -= 1
        }

        if left >= right {
            break;
        }

        if disk[left].0 > 0 {
            let avail = disk[left].0.min(disk[right].0);
            let new_entry = (avail, disk[right].1);
            disk[left].0 -= avail;
            disk[right].0 -= avail;
            disk.insert(left, new_entry);
            left += 1;
            //disk.push((avail, None));
            // right += 1;
            continue;
        }
    }

    disk
}

fn checksum(disk: &[Packet]) -> u64 {
    disk.iter()
        .filter(|&(len, _)| *len > 0)
        .take_while(|&(len, name)| name.is_some() || *len == 0)
        .fold((0_u64, 0_u64), |(sum, idx), (len, name)| {
            let mut idx_by = 0;
            let mut sum_by = 0;
            for offset in 0..*len {
                sum_by += (idx + offset) * name.expect("we already checked this");
                idx_by += 1;
            }

            (sum + sum_by, idx + idx_by)
        })
        .0
}

fn repack2(mut disk: Vec<Packet>) -> Vec<Packet> {
    let mut right = disk.len() - 1;

    while right > 0 {
        let mut left = 1;
        if disk[right].1.is_none() {
            right -= 1;
            continue;
        }
        while left < right {
            if disk[left].1.is_some() {
                left += 1;
                continue;
            }
            if disk[left].0 < disk[right].0 {
                left += 1;
                continue;
            }

            let new_entry = disk[right];
            disk[right].1 = None;
            disk[left].0 -= new_entry.0;
            if disk[left].0 == 0 {
                disk[left] = new_entry;
            } else {
                disk.insert(left, new_entry);
                right += 1;
            }
            break;
        }
        right -= 1;
    }

    disk
}

fn checksum2(disk: &[Packet]) -> u64 {
    disk.iter()
        .filter(|&(len, _)| *len > 0)
        .fold((0_u64, 0_u64), |(sum, idx), (len, name)| {
            let mut idx_by = 0;
            let mut sum_by = 0;
            match name {
                // we need to keep track of skipped block positions
                // so our index is continuous and correct
                None => (sum, idx + len),
                Some(name) => {
                    for offset in 0..*len {
                        sum_by += (idx + offset) * name;
                        idx_by += 1;
                    }

                    (sum + sum_by, idx + idx_by)
                }
            }
        })
        .0
}

impl Day<u64> for Day9 {
    fn part1(input: &str) -> u64 {
        let d = expand_map(input);
        let d = repack(d);
        checksum(&d)
    }

    fn part2(input: &str) -> u64 {
        let d = expand_map(input);
        let d = repack2(d);
        checksum2(&d)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fetch_input;

    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_disk_sizes() {
        assert_eq!(42, disk_size(TEST_INPUT));
        assert_eq!(95567, disk_size(&fetch_input(9).expect("can load")));
    }

    #[test]
    fn test_expand() {
        let d = expand_map(TEST_INPUT);
        println!("{:?}", d);
        assert_eq!(d.len(), 18);
    }

    #[test]
    fn test_expand_small() {
        let r = vec![
            (1, Some(0)),
            (2, None),
            (3, Some(1)),
            (4, None),
            (5, Some(2)),
        ];
        assert_eq!(expand_map("12345"), r);
    }

    // #[test]
    // fn test_repack() {
    //     let d = repack(expand_map("12345"));
    //     assert_eq!(
    //         vec![(1, Some(0)), (2, Some(2)), (3, Some(1)), (3, Some(2))],
    //         d
    //     );
    // }

    #[test]
    fn end_to_end() {
        let d = expand_map(TEST_INPUT);
        let d = repack(d);
        println!("{:?}", d);
        assert_eq!(1928, checksum(&d));
    }

    #[test]
    fn end_to_end2() {
        let d = &repack2(expand_map(TEST_INPUT));
        println!("{:?}", d);
        assert_eq!(2858, checksum2(d));
    }
}
