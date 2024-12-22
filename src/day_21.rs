use cached::SizedCache;
use std::iter::zip;

use crate::{grid::*, TaskCompleter};
use cached::proc_macro::cached;
use itertools::Itertools;
use lazy_static::lazy_static;
use pathfinding::prelude::astar_bag;

pub struct Task21;

lazy_static! {
    static ref NUMERIC_KEYPAD_GRID: Grid<char> =
        Grid::from_string(include_str!("../input/day_21/numeric_keypad"), true);
    static ref DIRECTION_KEYPAD_GRID: Grid<char> =
        Grid::from_string(include_str!("../input/day_21/directional_keypad"), true);
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct DirectionalKeypadState {
    pos: Coord,
}
impl DirectionalKeypadState {
    fn new() -> Self {
        let pos = DIRECTION_KEYPAD_GRID.find_coord(|x| *x == 'A').unwrap();
        Self { pos: pos }
    }

    fn press_button(&mut self, button: char) -> Result<Option<char>, ()> {
        match button {
            '>' => {
                if let Some(x) = self.pos.translate(Direction::Right, &DIRECTION_KEYPAD_GRID) {
                    if DIRECTION_KEYPAD_GRID[x] != 'N' {
                        self.pos = x;
                        return Ok(None);
                    }
                }
                Err(())
            }
            '<' => {
                if let Some(x) = self.pos.translate(Direction::Left, &DIRECTION_KEYPAD_GRID) {
                    if DIRECTION_KEYPAD_GRID[x] != 'N' {
                        self.pos = x;
                        return Ok(None);
                    }
                }
                Err(())
            }
            '^' => {
                if let Some(x) = self.pos.translate(Direction::Up, &DIRECTION_KEYPAD_GRID) {
                    if DIRECTION_KEYPAD_GRID[x] != 'N' {
                        self.pos = x;
                        return Ok(None);
                    }
                }
                Err(())
            }
            'v' => {
                if let Some(x) = self.pos.translate(Direction::Down, &DIRECTION_KEYPAD_GRID) {
                    if DIRECTION_KEYPAD_GRID[x] != 'N' {
                        self.pos = x;
                        return Ok(None);
                    }
                }
                Err(())
            }
            'A' => Ok(Some(DIRECTION_KEYPAD_GRID[self.pos])),
            _ => panic!("Unexpected input"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct NumericKeypadState2 {
    pos: Coord,
    directions_taken: String,
}
impl NumericKeypadState2 {
    fn new() -> Self {
        let pos = NUMERIC_KEYPAD_GRID.find_coord(|x| *x == 'A').unwrap();
        Self {
            pos: pos,
            directions_taken: "".to_owned(),
        }
    }

    fn get_successors(&self) -> Vec<(Self, i64)> {
        vec!['>', '^', '<', 'v']
            .iter()
            .filter_map(|x| self.move_direction(*x).ok())
            .map(|x| (x, 1))
            .collect_vec()
    }

    fn move_direction(&self, dir: char) -> Result<Self, ()> {
        let mut new = self.clone();
        match dir {
            '>' => {
                if let Some(x) = new.pos.translate(Direction::Right, &NUMERIC_KEYPAD_GRID) {
                    if NUMERIC_KEYPAD_GRID[x] != 'N' {
                        new.pos = x;
                        new.directions_taken.push(dir);
                        return Ok(new);
                    }
                }
                Err(())
            }
            '<' => {
                if let Some(x) = new.pos.translate(Direction::Left, &NUMERIC_KEYPAD_GRID) {
                    if NUMERIC_KEYPAD_GRID[x] != 'N' {
                        new.pos = x;
                        new.directions_taken.push(dir);
                        return Ok(new);
                    }
                }
                Err(())
            }
            '^' => {
                if let Some(x) = new.pos.translate(Direction::Up, &NUMERIC_KEYPAD_GRID) {
                    if NUMERIC_KEYPAD_GRID[x] != 'N' {
                        new.pos = x;
                        new.directions_taken.push(dir);
                        return Ok(new);
                    }
                }
                Err(())
            }
            'v' => {
                if let Some(x) = new.pos.translate(Direction::Down, &NUMERIC_KEYPAD_GRID) {
                    if NUMERIC_KEYPAD_GRID[x] != 'N' {
                        new.pos = x;
                        new.directions_taken.push(dir);
                        return Ok(new);
                    }
                }
                Err(())
            }
            _ => panic!("Unexpected input"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct NumericKeypadState {
    pos: Coord,
}

impl NumericKeypadState {
    fn new() -> Self {
        let pos = NUMERIC_KEYPAD_GRID.find_coord(|x| *x == 'A').unwrap();
        Self { pos: pos }
    }

    fn press_button(&mut self, button: char) -> Result<Option<char>, ()> {
        match button {
            '>' => {
                if let Some(x) = self.pos.translate(Direction::Right, &NUMERIC_KEYPAD_GRID) {
                    if NUMERIC_KEYPAD_GRID[x] != 'N' {
                        self.pos = x;
                        return Ok(None);
                    }
                }
                Err(())
            }
            '<' => {
                if let Some(x) = self.pos.translate(Direction::Left, &NUMERIC_KEYPAD_GRID) {
                    if NUMERIC_KEYPAD_GRID[x] != 'N' {
                        self.pos = x;
                        return Ok(None);
                    }
                }
                Err(())
            }
            '^' => {
                if let Some(x) = self.pos.translate(Direction::Up, &NUMERIC_KEYPAD_GRID) {
                    if NUMERIC_KEYPAD_GRID[x] != 'N' {
                        self.pos = x;
                        return Ok(None);
                    }
                }
                Err(())
            }
            'v' => {
                if let Some(x) = self.pos.translate(Direction::Down, &NUMERIC_KEYPAD_GRID) {
                    if NUMERIC_KEYPAD_GRID[x] != 'N' {
                        self.pos = x;
                        return Ok(None);
                    }
                }
                Err(())
            }
            'A' => Ok(Some(NUMERIC_KEYPAD_GRID[self.pos])),
            _ => panic!("Unexpected input"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct PlayState {
    numeric_keys_pressed: String,
    numeric_key_robot: NumericKeypadState,
    direct_robot_1: DirectionalKeypadState,
    direct_robot_2: DirectionalKeypadState,
}

enum Transition {
    Stationary,
    AToDown,
    AToUp,
    AToRight,
    AToLeft,
    RightToA,
    RightToDown,
    RightToUp,
    RightToLeft,
    DownToA,
    DownToUp,
    DownToRight,
    DownToLeft,
    UpToA,
    UpToDown,
    UpToRight,
    UpToLeft,
    LeftToA,
    LeftToDown,
    LeftToUp,
    LeftToRight,
}

fn string_to_transitions(input: &str) -> Vec<Transition> {
    let mut v = vec![];
    let mut chars = input.chars();
    let mut char = chars.next().unwrap();
    while let Some(next_char) = chars.next() {
        v.push(Transition::new(char, next_char));
        char = next_char;
    }
    v
}

impl Transition {
    fn new(from: char, to: char) -> Self {
        match from {
            '>' => match to {
                '>' => Self::Stationary,
                '<' => Self::RightToLeft,
                '^' => Self::RightToUp,
                'v' => Self::RightToDown,
                'A' => Self::RightToA,
                _ => panic!("Unexpected input"),
            },
            '<' => match to {
                '>' => Self::LeftToRight,
                '<' => Self::Stationary,
                '^' => Self::LeftToUp,
                'v' => Self::LeftToDown,
                'A' => Self::LeftToA,
                _ => panic!("Unexpected input"),
            },
            '^' => match to {
                '>' => Self::UpToRight,
                '<' => Self::UpToLeft,
                '^' => Self::Stationary,
                'v' => Self::UpToDown,
                'A' => Self::UpToA,
                _ => panic!("Unexpected input"),
            },
            'v' => match to {
                '>' => Self::DownToRight,
                '<' => Self::DownToLeft,
                '^' => Self::DownToUp,
                'v' => Self::Stationary,
                'A' => Self::DownToA,
                _ => panic!("Unexpected input"),
            },
            'A' => match to {
                '>' => Self::AToRight,
                '<' => Self::AToLeft,
                '^' => Self::AToUp,
                'v' => Self::AToDown,
                'A' => Self::Stationary,
                _ => panic!("Unexpected input"),
            },
            _ => panic!("Unexpected input"),
        }
    }
}
fn shortest_inputs_from_to_direcitonal_backup(from: char, to: char) -> Vec<&'static str> {
    let res = match from {
        '>' => match to {
            '>' => vec!["A"],
            '<' => vec!["<<A"],
            '^' => vec!["<^A", "^<A"],
            'v' => vec!["<A"],
            'A' => vec!["^A"],
            _ => panic!("Unexpected input"),
        },
        '<' => match to {
            '>' => vec![">>A"],
            '<' => vec!["A"],
            '^' => vec![">^A"],
            'v' => vec![">A"],
            'A' => vec![">>^A", ">^>A"],
            _ => panic!("Unexpected input"),
        },
        '^' => match to {
            '>' => vec![">vA", "v>A"],
            '<' => vec!["v<A"],
            '^' => vec!["A"],
            'v' => vec!["vA"],
            'A' => vec![">A"],
            _ => panic!("Unexpected input"),
        },
        'v' => match to {
            '>' => vec![">A"],
            '<' => vec!["<A"],
            '^' => vec!["^A"],
            'v' => vec!["A"],
            'A' => vec![">^A", "^>A"],
            _ => panic!("Unexpected input"),
        },
        'A' => match to {
            '>' => vec!["vA"],
            '<' => vec!["v<<A", "<v<A"],
            '^' => vec!["<A"],
            'v' => vec!["<vA", "v<A"],
            'A' => vec!["A"],
            _ => panic!("Unexpected input"),
        },
        _ => panic!("Unexpected input"),
    };
    // println!("Getting input from: {} to {}, as {:?}", from, to, &res);
    res
}

fn shortest_inputs_from_to_direcitonal(from: char, to: char) -> Vec<&'static str> {
    let res = match from {
        '>' => match to {
            '>' => vec!["AA"],
            '<' => vec!["A<<A"],
            '^' => vec!["A<^A", "A^<A"],
            'v' => vec!["A<A"],
            'A' => vec!["A^A"],
            _ => panic!("Unexpected input"),
        },
        '<' => match to {
            '>' => vec!["A>>A"],
            '<' => vec!["AA"],
            '^' => vec!["A>^A"],
            'v' => vec!["A>A"],
            'A' => vec!["A>>^A", "A>^>A"],
            _ => panic!("Unexpected input"),
        },
        '^' => match to {
            '>' => vec!["A>vA", "Av>A"],
            '<' => vec!["Av<A"],
            '^' => vec!["AA"],
            'v' => vec!["AvA"],
            'A' => vec!["A>A"],
            _ => panic!("Unexpected input"),
        },
        'v' => match to {
            '>' => vec!["A>A"],
            '<' => vec!["A<A"],
            '^' => vec!["A^A"],
            'v' => vec!["AA"],
            'A' => vec!["A>^A", "A^>A"],
            _ => panic!("Unexpected input"),
        },
        'A' => match to {
            '>' => vec!["AvA"],
            '<' => vec!["Av<<A", "A<v<A"],
            '^' => vec!["A<A"],
            'v' => vec!["A<vA", "Av<A"],
            'A' => vec!["AA"],
            _ => panic!("Unexpected input"),
        },
        _ => panic!("Unexpected input"),
    };
    // println!("Getting input from: {} to {}, as {:?}", from, to, &res);
    res
}

fn shortest_inputs_from_to_numeric(from: char, to: char) -> Vec<String> {
    let start = NumericKeypadState2 {
        pos: NUMERIC_KEYPAD_GRID.find_coord(|x| *x == from).unwrap(),
        directions_taken: "".to_owned(),
    };

    astar_bag(
        &start,
        |x| x.get_successors(),
        |_| 0,
        |x| NUMERIC_KEYPAD_GRID[x.pos] == to,
    )
    .unwrap()
    .0
    .map(|x| {
        let mut v = x.last().unwrap().directions_taken.clone();
        v.push('A');
        v
    })
    .collect_vec()
}

fn shortest_inputs_for_presses_directional(input: &str) -> Vec<String> {
    zip(input.chars(), input[1..].chars())
        .map(|(from, to)| shortest_inputs_from_to_direcitonal(from, to))
        .multi_cartesian_product()
        .min_set_by_key(|x| x.into_iter().map(|z| z.len()).sum::<usize>())
        .into_iter()
        .map(|x| x.into_iter().join(""))
        .collect_vec()
}

#[cached(
    ty = "SizedCache<(String, i64), i64>",
    create = "{ SizedCache::with_size(100) }",
    convert = r#"{ (input.to_owned(), recur) }"#
)]
fn shortest_inputs_for_presses_directional_rec(input: &str, recur: i64) -> i64 {
    // println!("Tring to find for {input} at depth {recur}");
    let res = if recur == 0 {
        if input.len() < 2 {
            panic!("shouldn't happen");
        } else if input.len() == 2 {
            shortest_inputs_from_to_direcitonal_backup(
                input.chars().nth(0).unwrap(),
                input.chars().nth(1).unwrap(),
            )
            .first()
            .unwrap()
            .len() as i64
        } else {
            let split_point = input.len() / 2;
            let res1 = shortest_inputs_for_presses_directional_rec(&input[..split_point + 1], 0);
            let res2 = shortest_inputs_for_presses_directional_rec(&input[split_point..], 0);
            res1 + res2
        }
    } else {
        // Get all transitions and figure out cheapeast way to do them
        zip(input.chars(), input[1..].chars())
            .map(|(x, y)| {
                shortest_inputs_from_to_direcitonal(x, y)
                    .iter()
                    .map(|s| shortest_inputs_for_presses_directional_rec(s, recur - 1))
                    .min()
                    .unwrap()
            })
            .sum::<i64>()
    };
    // println!("Found for {input} at depth {recur}, result is {res}");
    res
}

fn shortest_inputs_for_presses(input: &str, number_of_directions: i64) -> i64 {
    zip("A".chars().chain(input.chars()), input.chars())
        .map(|(from, to)| shortest_inputs_from_to_numeric(from, to))
        .multi_cartesian_product()
        .min_set_by_key(|x| x.into_iter().map(|z| z.len()).sum::<usize>())
        .into_iter()
        .map(|x| x.into_iter().join(""))
        .collect_vec()
        .iter()
        .map(|x| {
            shortest_inputs_for_presses_directional_rec(&format!("A{x}"), number_of_directions)
        })
        .min()
        .unwrap()
}

impl PlayState {
    fn get_successors(&self) -> Vec<(PlayState, i64)> {
        vec!['>', '^', '<', 'v', 'A']
            .iter()
            .filter_map(|x| self.press_button(*x).ok())
            .map(|x| (x, 1))
            .collect_vec()
    }

    fn press_button(&self, button: char) -> Result<Self, ()> {
        let mut new = self.clone();
        if let Some(button) = new.direct_robot_1.press_button(button)? {
            if let Some(button) = new.direct_robot_2.press_button(button)? {
                if let Some(button) = new.numeric_key_robot.press_button(button)? {
                    new.numeric_keys_pressed.push(button);
                }
            }
        }
        Ok(new)
    }
}

impl TaskCompleter for Task21 {
    fn do_task_1(&self) -> String {
        let input = include_str!("../input/day_21/input").lines();
        input
            .map(|l| shortest_inputs_for_presses(l, 1) * l[0..3].parse::<i64>().unwrap())
            .sum::<i64>()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let input = include_str!("../input/day_21/input").lines();
        input
            .map(|l| shortest_inputs_for_presses(l, 24) * l[0..3].parse::<i64>().unwrap())
            .sum::<i64>()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("222670".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("271397390297138".to_string())
    }
}
