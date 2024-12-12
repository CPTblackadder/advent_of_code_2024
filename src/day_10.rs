use strum::IntoEnumIterator;

use crate::{grid::*, TaskCompleter};

pub struct Task10;

fn score_trail_by_dest(grid: &Grid<u32>, pos: Coord, req_value: u32) -> Vec<Coord> {
    let v = grid[pos];
    if v != req_value {
        vec![]
    } else if req_value == 9 {
        vec![pos]
    } else {
        let mut h = vec![];
        for d in Direction::iter() {
            if let Some(c) = pos.translate(d, grid) {
                h.append(&mut score_trail_by_dest(grid, c, req_value + 1))
            }
        }
        h.sort();
        h.dedup();
        h
    }
}

fn score_trail_by_routes(grid: &Grid<u32>, pos: Coord, req_value: u32) -> i64 {
    let v = grid[pos];
    if v != req_value {
        0
    } else if req_value == 9 {
        1
    } else {
        Direction::iter()
            .map(|d| {
                if let Some(c) = pos.translate(d, grid) {
                    score_trail_by_routes(grid, c, req_value + 1)
                } else {
                    0
                }
            })
            .sum::<i64>()
    }
}

impl TaskCompleter for Task10 {
    fn do_task_1(&self) -> String {
        let grid = Grid::<char>::from_string(include_str!("../input/day_10/input"), true)
            .map(|x| x.to_digit(10).unwrap());
        let trail_heads = grid.iter().filter(|(_, x)| **x == 0);
        trail_heads
            .map(|(x, _)| score_trail_by_dest(&grid, x, 0).len() as i64)
            .sum::<i64>()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let grid = Grid::<char>::from_string(include_str!("../input/day_10/input"), true)
            .map(|x| x.to_digit(10).unwrap());
        let trail_heads = grid.iter().filter(|(_, x)| **x == 0);
        trail_heads
            .map(|(x, _)| score_trail_by_routes(&grid, x, 0))
            .sum::<i64>()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("811".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("1794".to_string())
    }
}
