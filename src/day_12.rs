use std::{collections::HashSet, rc::Rc};

use strum::IntoEnumIterator;

use crate::{grid::*, TaskCompleter};

pub struct Task12;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Area {
    parts: Vec<Coord>,
    permiter: i64,
    char: char,
}

fn mark(visited: &mut Grid<bool>, coord: Coord, dir: Direction) {
    if let Some(x) = coord.translate(dir, visited) {
        visited[x] = true;
    }
}

fn traverse(debug: bool, start: Coord, grid: &Grid<char>, checked: &mut Grid<bool>) -> i64 {
    let mut coord = start;
    let mut sides_count = 0;
    let mut direction = Direction::Left;
    // Ensure starting direction is good to
    if let Some(x) = start.translate(direction, grid) {
        assert_ne!(grid[start], grid[x]);
    }
    if let Some(x) = start.translate(direction.left(), grid) {
        assert!(grid[start] != grid[x] || checked[x]);
    }
    if debug {
        println!("starting coord: {:?}, dir: {:?}", coord, direction,)
    }
    mark(checked, coord, direction);

    // Go round in a direction
    let new_side;
    (coord, direction, new_side) = next_tile_dir(grid, coord, direction);
    mark(checked, coord, direction);
    if new_side {
        sides_count += 1;
    }
    if debug {
        println!(
            "new coord: {:?}, dir: {:?}, {}",
            coord,
            direction,
            if new_side { "new side" } else { "not new side" }
        );
    }
    while coord != start || direction != Direction::Left {
        let new_side;
        (coord, direction, new_side) = next_tile_dir(grid, coord, direction);
        mark(checked, coord, direction);
        if new_side {
            sides_count += 1;
        }
        if debug {
            println!(
                "new coord: {:?}, dir: {:?}, {}",
                coord,
                direction,
                if new_side { "new side" } else { "not new side" }
            );
        }
    }
    sides_count
}

impl Area {
    fn get_value(&self) -> i64 {
        self.permiter * self.parts.len() as i64
    }

    fn get_sides_of_perimiter(&self, grid: &Grid<char>) -> i64 {
        let mut checked: Grid<bool> = Grid::default_with_size(grid.width(), grid.height());
        for c in self.parts.iter() {
            checked[*c] = true;
        }
        let debug = false;
        if debug {
            println!("Checking sides of {}", self.char)
        }
        let start = *self.parts.iter().min().unwrap();
        let mut sides_count = traverse(debug, start, grid, &mut checked);
        // Find internal areas
        // An internal area is one where:
        //   XS
        //   AA
        // And X has not been visited
        while let Some(x) = self.parts.iter().find(|x| {
            x.translate(Direction::Down, grid).is_some_and(|y| {
                grid[**x] == grid[y]
                    && y.translate(Direction::Left, grid)
                        .is_some_and(|y| grid[**x] == grid[y])
            }) && x
                .translate(Direction::Left, grid)
                .is_some_and(|y| !checked[y])
        }) {
            sides_count += traverse(debug, *x, grid, &mut checked);
        }

        assert!(self
            .parts
            .iter()
            .all(|c| Direction::iter().all(|d| c.translate(d, grid).is_none_or(|x| checked[x]))));

        if debug {
            println!("Area: {} has {} sides", self.char, sides_count);
        }
        sides_count
    }
}

/**
 * Possible turns, if travelling Right, thus checking in up Direction:
 * S is tile we're checking from
 * A is tiles that must be the same to make the turn
 * X is tiles that must be different
 * Left:
 *     
 *   XA
 *   SA
 *
 *  Right:
 *
 *   SX
 *
 * Straight:
 *     X
 *    SA
 *
 */
fn next_tile_dir(grid: &Grid<char>, coord: Coord, dir: Direction) -> (Coord, Direction, bool) {
    // Check tile ahead
    let next_tile = coord.translate(dir.right(), grid);
    if let Some(next_tile) = next_tile {
        if grid[next_tile] != grid[coord] {
            // Turn right as next tile is not of same type
            (coord, dir.right(), true)
        } else {
            // check tile above next_tile
            let above_tile = next_tile.translate(dir, grid);
            if let Some(above_tile) = above_tile {
                if grid[above_tile] == grid[coord] {
                    // Must turn left
                    (above_tile, dir.left(), true)
                } else {
                    // Can't turn left so go straight
                    (next_tile, dir, false)
                }
            } else {
                // Can't turn left so go straight
                (next_tile, dir, false)
            }
        }
    } else {
        (coord, dir.right(), true)
    }
}

// Returns permiter
fn visit_tile(grid: &Grid<char>, coord: Coord, visited: &mut Vec<Coord>) -> i64 {
    let char = grid[coord];
    let mut permiter = 4;
    visited.push(coord);
    for dir in Direction::iter() {
        if let Some(c) = coord.translate(dir, grid) {
            if grid[c] == char {
                permiter -= 1;
                if !visited.contains(&c) {
                    let x = visit_tile(grid, c, visited);
                    permiter += x;
                }
            }
        }
    }
    permiter
}

fn create_area(
    grid: &Grid<char>,
    areas: &mut Grid<Option<Rc<Area>>>,
    starting_coord: Coord,
) -> Option<Rc<Area>> {
    if areas[starting_coord].is_some() {
        None
    } else {
        let mut visited = vec![];
        let permiter = visit_tile(grid, starting_coord, &mut visited);
        let area = Rc::new(Area {
            parts: visited,
            char: grid[starting_coord],
            permiter,
        });
        for c in &area.parts {
            areas[*c] = Some(area.clone());
        }
        Some(area)
    }
}

fn construct_areas() -> (Grid<char>, HashSet<Rc<Area>>) {
    let grid = Grid::from_string(include_str!("../input/day_12/input"), true);
    let mut grid_defined_areas: Grid<Option<Rc<Area>>> =
        Grid::default_with_size(grid.width(), grid.height());
    let mut areas = HashSet::new();
    for (coord, _) in grid.iter() {
        if let Some(x) = create_area(&grid, &mut grid_defined_areas, coord) {
            areas.insert(x);
        }
    }
    (grid, areas)
}

impl TaskCompleter for Task12 {
    fn do_task_1(&self) -> String {
        let (_, areas) = construct_areas();
        areas.iter().map(|x| x.get_value()).sum::<i64>().to_string()
    }

    fn do_task_2(&self) -> String {
        let (grid, areas) = construct_areas();

        areas
            .iter()
            .map(|x| x.get_sides_of_perimiter(&grid) * x.parts.len() as i64)
            .sum::<i64>()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("1363682".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("787680".to_string())
    }
}
