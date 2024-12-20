use std::collections::HashSet;

use pathfinding::prelude::{astar, astar_bag};

use crate::{grid::*, TaskCompleter};

pub struct Task16;

#[derive(Eq, Clone, Copy, Hash, PartialEq, Debug)]
struct ReindeerPos {
    rotation: Direction,
    location: Coord,
}

impl ReindeerPos {
    fn new(coord: Coord) -> Self {
        Self {
            rotation: Direction::Right,
            location: coord,
        }
    }

    fn get_next_positions(&self, grid: &Grid<char>) -> Vec<(ReindeerPos, i64)> {
        if grid[self.location.translate_no_bounds(self.rotation)] != '#' {
            vec![
                (
                    Self {
                        rotation: self.rotation.right(),
                        location: self.location,
                    },
                    1000,
                ),
                (
                    Self {
                        rotation: self.rotation.left(),
                        location: self.location,
                    },
                    1000,
                ),
                (
                    Self {
                        rotation: self.rotation,
                        location: self.location.translate_no_bounds(self.rotation),
                    },
                    1,
                ),
            ]
        } else {
            vec![
                (
                    Self {
                        rotation: self.rotation.right(),
                        location: self.location,
                    },
                    1000,
                ),
                (
                    Self {
                        rotation: self.rotation.left(),
                        location: self.location,
                    },
                    1000,
                ),
            ]
        }
    }
}

impl TaskCompleter for Task16 {
    fn do_task_1(&self) -> String {
        let grid = Grid::from_string(include_str!("../input/day_16/input"), true);
        let start = ReindeerPos::new(grid.find_coord(|x| *x == 'S').unwrap());
        let goal = grid.find_coord(|x| *x == 'E').unwrap();
        let result = astar(
            &start,
            |s| s.get_next_positions(&grid),
            |s| s.location.non_diagnal_distance(&goal),
            |s| s.location == goal,
        );
        result.unwrap().1.to_string()
    }

    fn do_task_2(&self) -> String {
        let grid = Grid::from_string(include_str!("../input/day_16/input"), true);
        let start = ReindeerPos::new(grid.find_coord(|x| *x == 'S').unwrap());
        let goal = grid.find_coord(|x| *x == 'E').unwrap();
        let result = astar_bag(
            &start,
            |s| s.get_next_positions(&grid),
            |s| s.location.non_diagnal_distance(&goal),
            |s| s.location == goal,
        );
        HashSet::<Coord>::from_iter(result.unwrap().0.flatten().map(|x| x.location))
            .len()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        None
    }

    fn task_2_result(&self) -> Option<String> {
        None
    }
}
