use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use strum::IntoEnumIterator;

use crate::{grid::*, TaskCompleter};

pub struct Task20;

const COORDS_IN_TWO_SPACES: [(Coord, i64); 4] = [
    (Coord::new(-2, 0), 2),
    (Coord::new(2, 0), 2),
    (Coord::new(0, -2), 2),
    (Coord::new(0, 2), 2),
];

fn find_cheats(
    min_cost: i64,
    path: &Vec<Coord>,
    grid: &Grid<Option<i64>>,
    possible_cheat_paths: &Vec<(Coord, i64)>,
) -> i64 {
    path.into_par_iter()
        .map(|c| {
            let mut possible_cheats = vec![];
            let starting_cost = grid[*c].unwrap();
            for (o, cheat_cost) in possible_cheat_paths {
                if let Some(possible_cheat_new_pos) = grid.in_bounds(*c - *o) {
                    if let Some(possible_new_pos_cost) = grid[possible_cheat_new_pos] {
                        possible_cheats.push((
                            (*c, possible_cheat_new_pos),
                            possible_new_pos_cost - starting_cost - cheat_cost,
                        ));
                    }
                };
            }
            possible_cheats
        })
        .flatten_iter()
        .filter(|x| x.1 >= min_cost)
        .count() as i64
}

impl TaskCompleter for Task20 {
    fn do_task_1(&self) -> String {
        let grid = Grid::from_string(include_str!("../input/day_20/input"), true);
        let mut cost: Grid<Option<i64>> = Grid::default_with_size(grid.width(), grid.height());
        let start = grid.find_coord(|x| *x == 'S').unwrap();
        let end = grid.find_coord(|x| *x == 'E').unwrap();
        let mut c = start;
        let mut path = vec![start];
        cost[start] = Some(0);
        while c != end {
            // find the correct path
            let new_c = c.translate_no_bounds(
                Direction::iter()
                    .filter(|d| {
                        c.translate(*d, &grid).is_some()
                            && cost[c.translate_no_bounds(*d)].is_none()
                            && grid[c.translate_no_bounds(*d)] != '#'
                    })
                    .last()
                    .unwrap(),
            );
            cost[new_c] = Some(cost[c].unwrap() + 1);
            path.push(new_c);
            c = new_c;
        }

        find_cheats(100, &path, &cost, &COORDS_IN_TWO_SPACES.to_vec()).to_string()
    }

    fn do_task_2(&self) -> String {
        let grid = Grid::from_string(include_str!("../input/day_20/input"), true);
        let mut cost: Grid<Option<i64>> = Grid::default_with_size(grid.width(), grid.height());
        let start = grid.find_coord(|x| *x == 'S').unwrap();
        let end = grid.find_coord(|x| *x == 'E').unwrap();
        let mut c = start;
        let mut path = vec![start];
        cost[start] = Some(0);
        let mut possible_cheat_paths = vec![];
        for i in 1..21 {
            possible_cheat_paths.push((Coord::new(i, 0), i));
            possible_cheat_paths.push((Coord::new(0, i), i));
            possible_cheat_paths.push((Coord::new(-i, 0), i));
            possible_cheat_paths.push((Coord::new(0, -i), i));
            for j in 1..(21 - i) {
                possible_cheat_paths.push((Coord::new(i, j), i + j));
                possible_cheat_paths.push((Coord::new(-i, j), i + j));
                possible_cheat_paths.push((Coord::new(i, -j), i + j));
                possible_cheat_paths.push((Coord::new(-i, -j), i + j));
            }
        }

        while c != end {
            // find the correct path
            let new_c = c.translate_no_bounds(
                Direction::iter()
                    .filter(|d| {
                        c.translate(*d, &grid).is_some()
                            && cost[c.translate_no_bounds(*d)].is_none()
                            && grid[c.translate_no_bounds(*d)] != '#'
                    })
                    .last()
                    .unwrap(),
            );
            cost[new_c] = Some(cost[c].unwrap() + 1);
            path.push(new_c);
            c = new_c;
        }

        find_cheats(100, &path, &cost, &possible_cheat_paths).to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("1426".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("1000697".to_string())
    }
}
