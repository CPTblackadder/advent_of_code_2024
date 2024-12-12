use strum::IntoEnumIterator;

use crate::{
    grid::{CompassDirection, Coord, CoordIterator, Grid},
    TaskCompleter,
};

pub struct Task4;

fn is_xmass_in_dir(grid: &Grid<char>, coord: Coord, dir: CompassDirection) -> bool {
    let mut coord = coord;
    for char in ['M', 'A', 'S'] {
        if let Some(c) = coord.translate_compass(dir, grid) {
            coord = c;
            if grid[c] != char {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn count_xmass(grid: &Grid<char>, coord: Coord) -> i64 {
    if grid[coord] == 'X' {
        CompassDirection::iter()
            .filter(|dir| is_xmass_in_dir(grid, coord, *dir))
            .count() as i64
    } else {
        0
    }
}

struct Void;
fn check_if_x_max(grid: &Grid<char>, coord: Coord) -> Option<Void> {
    if grid[coord] == 'A' {
        let up_right = coord.translate_compass(CompassDirection::NorthWest, grid)?;
        let down_left = coord.translate_compass(CompassDirection::SouthEast, grid)?;
        let down_left_desired;
        if grid[up_right] == 'M' {
            down_left_desired = 'S';
        } else if grid[up_right] == 'S' {
            down_left_desired = 'M';
        } else {
            return None;
        }
        if grid[down_left] != down_left_desired {
            return None;
        }

        let up_left = coord.translate_compass(CompassDirection::NorthEast, grid)?;
        let down_right = coord.translate_compass(CompassDirection::SouthWest, grid)?;
        let down_right_desired;
        if grid[up_left] == 'M' {
            down_right_desired = 'S';
        } else if grid[up_left] == 'S' {
            down_right_desired = 'M';
        } else {
            return None;
        }
        if grid[down_right] != down_right_desired {
            return None;
        }
        return Some(Void);
    }
    None
}

impl TaskCompleter for Task4 {
    fn do_task_1(&self) -> String {
        let grid = Grid::<char>::from_string(include_str!("../input/day_04/input"), false);

        CoordIterator::from_grid(&grid)
            .map(|c| count_xmass(&grid, c))
            .sum::<i64>()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let grid = Grid::<char>::from_string(include_str!("../input/day_04/input"), false);

        CoordIterator::from_grid(&grid)
            .filter(|c| check_if_x_max(&grid, *c).is_some())
            .count()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("2646".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("2000".to_string())
    }
}
