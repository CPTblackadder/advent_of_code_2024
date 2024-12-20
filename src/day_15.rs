use std::{
    collections::HashMap,
    fmt::{Display, Write},
};

use crate::{grid::*, TaskCompleter};

pub struct Task15;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    Character,
    Box,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Wall => f.write_char('#'),
            Tile::Empty => f.write_char('.'),
            Tile::Character => f.write_char('@'),
            Tile::Box => f.write_char('O'),
        }
    }
}

impl Tile {
    fn from_char(i: char) -> Self {
        match i {
            '.' => Self::Empty,
            '@' => Self::Character,
            'O' => Self::Box,
            '#' => Self::Wall,
            _ => panic!("Unexpected character {i}"),
        }
    }
}

fn get_tiles_to_move(grid: &Grid<Tile>, start: Coord, dir: Direction) -> Vec<Coord> {
    let mut start = start;
    let mut tiles_to_move = vec![];
    while let Some(pos) = start.translate(dir, grid) {
        if grid[pos] == Tile::Box {
            tiles_to_move.push(pos);
            start = pos;
        } else if grid[pos] == Tile::Wall {
            return vec![];
        } else if grid[pos] == Tile::Empty {
            tiles_to_move.push(pos);
            break;
        } else {
            panic!("Shouldn't see the character while doing this");
        }
    }
    tiles_to_move
}

fn get_tiles_to_move_large_box_layered(
    grid: &Grid<LargeBoxTile>,
    start: Coord,
    dir: Direction,
) -> HashMap<Coord, LargeBoxTile> {
    let mut tiles_to_move: HashMap<Coord, LargeBoxTile> = HashMap::new();
    let mut visited = vec![];
    let mut to_visit = vec![start];
    while let Some(x) = to_visit.pop() {
        // dbg!(x);
        // dbg!(dir);
        if visited.contains(&x) {
            continue;
        }
        visited.push(x);
        let new_pos = x.translate(dir, grid).unwrap();
        match grid[new_pos] {
            LargeBoxTile::LeftBox => {
                to_visit.push(new_pos);
                to_visit.push(new_pos.translate_no_bounds(Direction::Right))
            }
            LargeBoxTile::RightBox => {
                to_visit.push(new_pos);
                to_visit.push(new_pos.translate_no_bounds(Direction::Left))
            }
            LargeBoxTile::Empty => {} // Do nothing
            LargeBoxTile::Character => panic!("Didn't expect character here"),
            LargeBoxTile::Wall => return HashMap::new(),
        }
        tiles_to_move.insert(new_pos, grid[x]);
        if !tiles_to_move.contains_key(&x) {
            tiles_to_move.insert(x, LargeBoxTile::Empty);
        }
    }

    return tiles_to_move;
}

fn apply_command(grid: &mut Grid<Tile>, char_pos: &mut Coord, command: char) {
    let direction = match command {
        '^' => Direction::Up,
        'v' => Direction::Down,
        '<' => Direction::Left,
        '>' => Direction::Right,
        _ => return, // Unknown command, probably a new line, skip
    };
    let tiles_to_move = get_tiles_to_move(grid, *char_pos, direction);
    for new_pos in tiles_to_move.iter().rev() {
        let from = new_pos.translate_no_bounds(direction.opposite());
        let value = grid[from];
        match value {
            Tile::Character => {
                *char_pos = *new_pos;
                grid[*new_pos] = Tile::Character;
                grid[from] = Tile::Empty;
                return;
            }
            Tile::Box => grid[*new_pos] = Tile::Box,
            _ => panic!("Unexpected value {value}"),
        }
    }
    if tiles_to_move.len() > 0 {
        panic!("Should return through moving charater or be empty")
    }
}

fn apply_command_large_box(grid: &mut Grid<LargeBoxTile>, char_pos: &mut Coord, command: char) {
    let direction = match command {
        '^' => Direction::Up,
        'v' => Direction::Down,
        '<' => Direction::Left,
        '>' => Direction::Right,
        _ => return, // Unknown command, probably a new line, skip
    };

    let tiles_to_move = get_tiles_to_move_large_box_layered(grid, *char_pos, direction);
    for (new_pos, new_tile) in tiles_to_move.iter() {
        grid[*new_pos] = *new_tile;
    }
    if tiles_to_move.len() > 0 {
        *char_pos = char_pos.translate_no_bounds(direction);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LargeBoxTile {
    LeftBox,
    RightBox,
    Empty,
    Character,
    Wall,
}

impl Display for LargeBoxTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LargeBoxTile::LeftBox => f.write_char('['),
            LargeBoxTile::RightBox => f.write_char(']'),
            LargeBoxTile::Empty => f.write_char('.'),
            LargeBoxTile::Character => f.write_char('@'),
            LargeBoxTile::Wall => f.write_char('#'),
        }
    }
}

impl TaskCompleter for Task15 {
    fn do_task_1(&self) -> String {
        let mut s = include_str!("../input/day_15/input").split("\n\n");
        let mut grid = Grid::from_string(s.next().unwrap(), true).map(|x| Tile::from_char(x));
        let commands = s.next().unwrap();
        let mut char_pos = grid.find_coord(|x| *x == Tile::Character).unwrap();
        for s in commands.chars() {
            apply_command(&mut grid, &mut char_pos, s);
        }

        grid.iter()
            .map(|(c, t)| match t {
                Tile::Wall => 0,
                Tile::Empty => 0,
                Tile::Character => 0,
                Tile::Box => c.x() + ((grid.height() as i64 - c.y() - 1) * 100),
            })
            .sum::<i64>()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let mut s = include_str!("../input/day_15/input").split("\n\n");
        let g = Grid::from_string(s.next().unwrap(), true);
        let mut grid = Grid::init_with_size(LargeBoxTile::Empty, g.width() * 2, g.height());
        for (coord, char) in g.iter() {
            match char {
                '.' => {
                    let p = coord
                        .translate_with_distance(Direction::Right, coord.x(), &grid)
                        .unwrap();
                    let p1 = coord
                        .translate_with_distance(Direction::Right, coord.x() + 1, &grid)
                        .unwrap();
                    grid[p] = LargeBoxTile::Empty;
                    grid[p1] = LargeBoxTile::Empty;
                }
                '@' => {
                    let p = coord
                        .translate_with_distance(Direction::Right, coord.x(), &grid)
                        .unwrap();
                    let p1 = coord
                        .translate_with_distance(Direction::Right, coord.x() + 1, &grid)
                        .unwrap();

                    grid[p] = LargeBoxTile::Character;
                    grid[p1] = LargeBoxTile::Empty;
                }
                'O' => {
                    let p = coord
                        .translate_with_distance(Direction::Right, coord.x(), &grid)
                        .unwrap();
                    let p1 = coord
                        .translate_with_distance(Direction::Right, coord.x() + 1, &grid)
                        .unwrap();

                    grid[p] = LargeBoxTile::LeftBox;
                    grid[p1] = LargeBoxTile::RightBox;
                }
                '#' => {
                    let p = coord
                        .translate_with_distance(Direction::Right, coord.x(), &grid)
                        .unwrap();
                    let p1 = coord
                        .translate_with_distance(Direction::Right, coord.x() + 1, &grid)
                        .unwrap();
                    grid[p] = LargeBoxTile::Wall;
                    grid[p1] = LargeBoxTile::Wall;
                }
                _ => panic!("Unexpected character {char}"),
            }
        }
        let commands = s.next().unwrap();
        let mut char_pos = grid.find_coord(|x| *x == LargeBoxTile::Character).unwrap();
        for s in commands.chars() {
            apply_command_large_box(&mut grid, &mut char_pos, s);
        }

        grid.iter()
            .map(|(c, t)| match t {
                LargeBoxTile::Wall => 0,
                LargeBoxTile::Empty => 0,
                LargeBoxTile::Character => 0,
                LargeBoxTile::LeftBox => c.x() + ((grid.height() as i64 - c.y() - 1) * 100),
                LargeBoxTile::RightBox => 0,
            })
            .sum::<i64>()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("1465152".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("1511259".to_string())
    }
}
