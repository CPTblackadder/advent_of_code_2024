use std::collections::HashSet;

use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{
    grid::{Coord, Direction, Grid},
    TaskCompleter,
};

pub struct Task6;

fn look_for_loop(
    grid: &Grid<char>,
    been_through: &Grid<Vec<bool>>,
    from: Coord,
    dir: Direction,
) -> bool {
    let mut been_through = been_through.clone();
    let mut c = Some(from);
    let mut dir = dir;
    while c.is_some() {
        let x = c.unwrap();
        if been_through[x][dir] {
            return true;
        } else {
            been_through[x][dir] = true;
        }
        c = x.translate(dir, &grid);
        if let Some(y) = c {
            if grid[y] == '#' || grid[y] == '?' {
                dir = dir.right();
                c = x.translate(dir, &grid);
                if let Some(y) = c {
                    if grid[y] == '#' || grid[y] == '?' {
                        dir = dir.right();
                        c = x.translate(dir, &grid);
                    }
                }
            }
        }
    }
    false
}

fn look_for_loop_2(grid: &Grid<char>, from: Coord, dir: Direction) -> bool {
    let mut seen_turns: HashSet<Coord> = HashSet::new();
    let mut c = from.translate(dir.opposite(), grid);
    let mut dir = dir;
    while c.is_some() {
        let x = c.unwrap();
        c = x.translate(dir, &grid);
        if let Some(y) = c {
            if grid[y] == '#' || y == from {
                if seen_turns.contains(&x) {
                    return true;
                }
                seen_turns.insert(x);
                dir = dir.right();
                c = x.translate(dir, &grid);
                if let Some(y) = c {
                    if grid[y] == '#' || y == from {
                        dir = dir.right();
                        c = x.translate(dir, &grid);
                    }
                }
            }
        }
    }
    false
}

fn look_for_loop_hash_set(
    grid: &Grid<char>,
    been_through: &Vec<(Coord, Direction)>,
    from: Coord,
    dir: Direction,
) -> bool {
    let mut h: HashSet<(Coord, Direction)> = HashSet::from_iter(been_through.iter().cloned());
    let mut c = Some(from);
    let mut dir = dir;
    while c.is_some() {
        let x = c.unwrap();
        if h.contains(&(x, dir)) {
            return true;
        } else {
            h.insert((x, dir));
        }
        c = x.translate(dir, &grid);
        if let Some(y) = c {
            if grid[y] == '#' || grid[y] == '?' {
                dir = dir.right();
                c = x.translate(dir, &grid);
                if let Some(y) = c {
                    if grid[y] == '#' || grid[y] == '?' {
                        dir = dir.right();
                        c = x.translate(dir, &grid);
                    }
                }
            }
        }
    }
    false
}

fn do_task_2_unoptimized() -> String {
    let mut grid = Grid::<char>::from_string(include_str!("../input/day_06/input"), true);
    let mut c = grid.find_coord(|x| *x == '^');
    let mut dir = Direction::Up;
    let mut moved_over: Grid<Vec<bool>> =
        Grid::init_with_size(vec![false; 4], grid.width(), grid.height());
    while c.is_some() {
        let x = c.unwrap();
        moved_over[x][dir] = true;
        c = x.translate(dir, &grid);
        if let Some(y) = c {
            if grid[y] == '#' {
                dir = dir.right();
                c = x.translate(dir, &grid);
                if let Some(y) = c {
                    if grid[y] == '#' {
                        dir = dir.right();
                        c = x.translate(dir, &grid);
                    } else if grid[y] != 'O' && grid[y] != 'N' {
                        // Pretend y is blocked and see if we loop
                        grid[y] = '?';
                        if look_for_loop(&grid, &moved_over, x, dir.right()) {
                            grid[y] = 'O';
                        } else {
                            // This is not a blockable tile
                            grid[y] = 'N';
                        }
                    }
                }
            } else if grid[y] != 'O' && grid[y] != 'N' {
                // Pretend y is blocked and see if we loop
                grid[y] = '?';
                if look_for_loop(&grid, &moved_over, x, dir.right()) {
                    grid[y] = 'O';
                } else {
                    // This is not a blockable tile
                    grid[y] = 'N';
                }
            }
        }
    }
    grid.iter().filter(|(_, x)| **x == 'O').count().to_string()
}

fn do_task_2_optimized() -> String {
    let char_grid = Grid::<char>::from_string(include_str!("../input/day_06/input"), true);
    let mut path_grid: Grid<Option<Direction>> =
        Grid::default_with_size(char_grid.width(), char_grid.height());
    // Find starting pos
    let mut dir = Direction::Up;
    let mut c = char_grid
        .find_coord(|x| *x == '^')
        .unwrap()
        .translate(dir, &char_grid);
    while c.is_some() {
        let x = c.unwrap();
        if !path_grid[x].is_some() {
            path_grid[x] = Some(dir);
        }
        c = x.translate(dir, &char_grid);
        if let Some(y) = c {
            if char_grid[y] == '#' {
                dir = dir.right();
                c = x.translate(dir, &char_grid);
                if let Some(y) = c {
                    if char_grid[y] == '#' {
                        dir = dir.right();
                        c = x.translate(dir, &char_grid);
                    }
                }
            }
        }
    }

    path_grid
        .iter()
        .par_bridge()
        .filter(|(_, y)| y.is_some())
        .map(|(x, y)| (x, y.unwrap()))
        .map(|(x, y)| {
            if look_for_loop_2(&char_grid, x, y) {
                1
            } else {
                0
            }
        })
        .sum::<i64>()
        .to_string()
}

impl TaskCompleter for Task6 {
    fn do_task_1(&self) -> String {
        let mut grid = Grid::<char>::from_string(include_str!("../input/day_06/input"), true);
        // Find starting pos
        let mut c = grid.find_coord(|x| *x == '^');
        let mut dir = Direction::Up;
        while c.is_some() {
            let x = c.unwrap();
            grid[x] = 'F';
            c = x.translate(dir, &grid);
            if let Some(y) = c {
                if grid[y] == '#' {
                    dir = dir.right();
                    c = x.translate(dir, &grid);
                    if let Some(y) = c {
                        if grid[y] == '#' {
                            dir = dir.right();
                            c = x.translate(dir, &grid);
                        }
                    }
                }
            }
        }
        grid.iter().filter(|(_, x)| **x == 'F').count().to_string()
    }

    fn do_task_2(&self) -> String {
        do_task_2_optimized()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("4967".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("1789".to_string())
    }
}
