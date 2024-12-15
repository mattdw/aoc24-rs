use std::{
    collections::{HashMap, VecDeque},
    convert::identity,
};

use nalgebra::Vector2;

use crate::Day;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Object {
    Robot,
    Box,
    Wall,
    BoxLeft,
    BoxRight,
}

type Co = Vector2<i64>;
type Map = HashMap<Co, Object>;

fn parse(input: &str) -> (Map, Co, Vec<Co>) {
    let (map_section, move_section) = input.split_once("\n\n").expect("didn't find a blank line");

    let mut map = Map::new();
    let mut robotpos = Co::zeros();
    for (y, line) in map_section.trim().lines().enumerate() {
        for (x, char) in line.trim().chars().enumerate() {
            let co = Co::new(x as i64, y as i64);
            match char {
                '#' => {
                    map.insert(co, Object::Wall);
                }
                '.' => (),
                'O' => {
                    map.insert(co, Object::Box);
                }
                '@' => {
                    map.insert(co, Object::Robot);
                    robotpos = co;
                }
                _ => panic!("unexpected char"),
            }
        }
    }

    let moves: Vec<Co> = move_section
        .trim()
        .chars()
        .map(move_to_dir)
        .filter_map(identity)
        .rev()
        .collect();

    (map, robotpos, moves)
}

fn expand_map(map: Map, robot: Co) -> (Map, Co) {
    let mut new_map = Map::new();
    let mut new_robot = robot;

    let x1 = Co::new(1, 0);
    for (mut co, o) in map.into_iter() {
        co.x *= 2;
        match o {
            Object::Robot => {
                new_map.insert(co, o);
                new_robot = co;
            }
            Object::Wall => {
                new_map.insert(co, Object::Wall);
                new_map.insert(co + x1, Object::Wall);
            }
            Object::Box => {
                new_map.insert(co, Object::BoxLeft);
                new_map.insert(co + x1, Object::BoxRight);
            }
            _ => {
                panic!("can't expand already expanded map");
            }
        }
    }

    (new_map, new_robot)
}

fn move_to_dir(c: char) -> Option<Co> {
    match c {
        '<' => Some(Vector2::new(-1, 0)),
        '>' => Some(Vector2::new(1, 0)),
        '^' => Some(Vector2::new(0, -1)),
        'v' => Some(Vector2::new(0, 1)),
        '\n' => None,
        _ => panic!("bad char"),
    }
}

fn unoccupied(map: &Map, pos: &Co, dir: &Co) -> Option<(Co, Vec<Co>)> {
    let mut mult = 1;
    let mut intervening = vec![pos.clone()];
    loop {
        let offset = dir * mult;
        let new_co = pos + offset;

        let o = map.get(&new_co);
        match o {
            Some(Object::Robot) | Some(Object::Box) => {
                intervening.push(new_co);
                mult += 1;
                continue;
            }
            Some(&Object::Wall) => return None,
            None => return Some((new_co, intervening)),
            _ => panic!(),
        }
    }
}

fn box_score(co: &Co) -> i64 {
    co.y * 100 + co.x
}

fn map_score(map: &Map) -> i64 {
    map.iter()
        .map(|(k, v)| match v {
            Object::Box => box_score(k),
            _ => 0,
        })
        .sum()
}

fn make_move(state: (Map, Co, Vec<Co>)) -> (Map, Co, Vec<Co>) {
    let (mut map, mut robot, mut moves) = state;
    let move_len = moves.len();
    let Some(next_move) = moves.pop() else {
        println!("no moves remaining");
        return (map, robot, vec![]);
    };

    assert_eq!(map.get(&robot), Some(&Object::Robot));

    let Some((_unocc, mut intervening)) = unoccupied(&map, &robot, &next_move) else {
        // if we can't make the move, we jump to the next move
        assert_eq!(moves.len(), move_len - 1);
        return (map, robot, moves);
    };

    assert_eq!(_unocc, intervening.last().unwrap() + next_move);

    // we have to go from the end to not overwrite anything
    while !intervening.is_empty() {
        let i = intervening.pop().unwrap();
        let nxt_i = i + next_move;
        let o = *map.get(&i).expect("cell should not be empty");
        if o == Object::Robot {
            robot = nxt_i;
        }
        map.insert(nxt_i, o);
        map.remove(&i);
    }

    (map, robot, moves)
}

fn map_score_2(map: &Map) -> i64 {
    map.iter()
        .map(|(k, v)| match v {
            Object::BoxLeft => box_score(k),
            _ => 0,
        })
        .sum()
}

fn unoccupied_2(map: &Map, pos: &Co, dir: &Co) -> Option<(Co, Vec<Co>)> {
    let mut intervening = vec![pos.clone()];
    let mut rays = VecDeque::new();
    rays.push_back(pos.clone());
    let x1 = Co::new(1, 0);
    loop {
        let Some(pos) = rays.pop_front() else {
            break;
        };
        intervening.push(pos);
        let new_co = pos + dir;

        let o = map.get(&new_co);
        match o {
            Some(Object::Robot) | Some(Object::Box) => {
                panic!("we shouldn't be here!");
            }
            Some(Object::BoxLeft) => {
                rays.push_back(new_co);
                if dir.y != 0 {
                    rays.push_back(new_co + x1);
                }
            }
            Some(Object::BoxRight) => {
                rays.push_back(new_co);
                if dir.y != 0 {
                    rays.push_back(new_co - x1);
                }
            }
            // if we ever hit a wall, we can't move
            Some(Object::Wall) => return None,
            None => {}
        }
    }

    Some((pos.clone(), intervening))
}

fn make_move_2(state: (Map, Co, Vec<Co>)) -> (Map, Co, Vec<Co>) {
    let (mut map, mut robot, mut moves) = state;
    let move_len = moves.len();
    let Some(next_move) = moves.pop() else {
        println!("no moves remaining");
        return (map, robot, vec![]);
    };

    assert_eq!(map.get(&robot), Some(&Object::Robot));

    // unoccupied needs to branch now, and check for collisions
    let Some((_unocc, mut intervening)) = unoccupied_2(&map, &robot, &next_move) else {
        // if we can't make the move, we jump to the next move
        assert_eq!(moves.len(), move_len - 1);
        return (map, robot, moves);
    };

    // assert_eq!(_unocc, intervening.last().unwrap() + next_move);

    // we have to go from the end to not overwrite anything
    loop {
        let Some(i) = intervening.pop() else {
            break;
        };
        let nxt_i = i + next_move;
        let o = map.get(&i);
        if o == Some(&Object::Robot) {
            robot = nxt_i;
        }
        if let Some(&o) = o {
            map.insert(nxt_i, o);
            map.remove(&i);
        } else {
            // println!("found blank?");
        }
    }

    (map, robot, moves)
}

pub struct Day15 {}

impl Day<i64> for Day15 {
    fn part1(input: &str) -> i64 {
        let mut s = parse(input);

        while !&s.2.is_empty() {
            // println!("{:?} / {:?}", s.1, s.2);
            s = make_move(s);
        }

        map_score(&s.0)
    }

    fn part2(input: &str) -> i64 {
        let mut s = parse(input);
        (s.0, s.1) = expand_map(s.0, s.1);

        while !&s.2.is_empty() {
            // println!("{:?} / {:?}", s.1, s.2);
            s = make_move_2(s);
        }

        map_score_2(&s.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT_SMALL: &'static str = "
        ########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
    ";

    const TEST_INPUT: &'static str = "
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    #[test]
    fn end_to_end_1() {
        assert_eq!(10092, Day15::part1(TEST_INPUT));
    }

    #[test]
    fn end_to_end_small() {
        assert_eq!(2028, Day15::part1(TEST_INPUT_SMALL));
    }

    #[test]
    fn part2_t() {
        assert_eq!(9021, Day15::part2(TEST_INPUT));
    }
}
