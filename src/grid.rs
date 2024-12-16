use itertools::Itertools;
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Index, IndexMut, Sub},
};
// You need to bring the trait into scope to use it!
use strum_macros::EnumIter;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Default)]
pub struct Grid<T> {
    g: Vec<Vec<T>>,
}

pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    coord: Coord,
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = (Coord, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.coord.0 >= self.grid.width() as i64 {
            self.coord = Coord::new(0, self.coord.1 + 1);
        }
        if self.coord.1 >= self.grid.height() as i64 {
            None
        } else {
            let coord = self.coord;
            let ret = &self.grid[self.coord];
            self.coord = Coord::new(self.coord.0 + 1, self.coord.1);

            Some((coord, ret))
        }
    }
}

impl Grid<i64> {
    pub fn from_string_i64(input: &str) -> Self {
        let v = input
            .lines()
            .rev()
            .map(|x| {
                x.chars()
                    .map(|y| y.to_string().parse::<i64>().unwrap())
                    .collect()
            })
            .collect();
        Grid { g: v }
    }
}

impl Grid<char> {
    pub fn from_string(input: &str, reverse: bool) -> Self {
        let g;
        if reverse {
            g = input.lines().rev().map(|x| x.chars().collect()).collect();
        } else {
            g = input.lines().map(|x| x.chars().collect()).collect();
        }
        Grid { g }
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = "\n".to_owned();
        for y in (0..self.height()).rev() {
            let mut l = "".to_owned();

            for x in 0..self.width() {
                l += &self[Coord(x as i64, y as i64)].to_string();
            }
            str += &l;
            str += "\n";
        }
        f.write_str(&str)
    }
}

impl<T> IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.g[index.1 as usize][index.0 as usize]
    }
}

impl<T> Index<Coord> for Grid<T> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.g[index.1 as usize][index.0 as usize]
    }
}

impl<T: Default + Clone> Grid<T> {
    pub fn default_with_size(width: usize, height: usize) -> Self {
        let row_def = vec![T::default(); width];
        let g = vec![row_def; height];
        Self { g }
    }
}

impl<T: Clone> Grid<T> {
    pub fn init_with_size(init_value: T, width: usize, height: usize) -> Self {
        let row_def = vec![init_value.clone(); width];
        let g = vec![row_def; height];
        Self { g }
    }
}

impl<T: Clone + PartialEq> Grid<T> {
    pub fn do_flood_fill(&mut self, centre: Coord, to: T, unfilled: T, diagnols: bool) {
        let mut queue = vec![centre];

        while let Some(coord) = queue.pop() {
            if self.in_bounds(coord).is_some() && self[coord] == unfilled {
                self[coord] = to.clone();
                queue.append(&mut coord.get_neighbours(diagnols));
            }
        }
    }
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.g[0].len()
    }
    pub fn height(&self) -> usize {
        self.g.len()
    }

    pub fn in_bounds(&self, d: Coord) -> Option<Coord> {
        if d.0 >= 0 && d.0 < self.width() as i64 && d.1 >= 0 && d.1 < self.height() as i64 {
            Some(d)
        } else {
            None
        }
    }

    pub fn grid(&self) -> &Vec<Vec<T>> {
        &self.g
    }

    pub fn iter<'a>(&'a self) -> GridIter<'a, T> {
        GridIter {
            grid: self,
            coord: Coord::new(0, 0),
        }
    }

    pub fn find_coord<F>(&self, predicate: F) -> Option<Coord>
    where
        F: Fn(&T) -> bool,
    {
        for (coord, item) in self.iter() {
            if predicate(item) {
                return Some(coord);
            }
        }
        None
    }

    pub fn map<U, F: FnMut(T) -> U>(self, mut func: F) -> Grid<U> {
        Grid {
            g: self
                .g
                .into_iter()
                .map(|x| x.into_iter().map(|y| func(y)).collect_vec())
                .collect_vec(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, PartialOrd, Ord, Default)]
pub struct Coord(i64, i64);

impl Coord {
    pub fn translate_no_bounds(&self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Self(self.0, self.1 + 1),
            Direction::Down => Self(self.0, self.1 - 1),
            Direction::Left => Self(self.0 - 1, self.1),
            Direction::Right => Self(self.0 + 1, self.1),
        }
    }

    pub fn translate<T>(&self, dir: Direction, grid: &Grid<T>) -> Option<Self> {
        grid.in_bounds(self.translate_no_bounds(dir))
    }

    pub fn translate_with_distance<T>(
        &self,
        dir: Direction,
        distance: i64,
        grid: &Grid<T>,
    ) -> Option<Self> {
        // assert!(distance > 0);
        let d = match dir {
            Direction::Up => Self(self.0, self.1 + distance),
            Direction::Down => Self(self.0, self.1 - distance),
            Direction::Left => Self(self.0 - distance, self.1),
            Direction::Right => Self(self.0 + distance, self.1),
        };
        grid.in_bounds(d)
    }

    pub fn translate_compass<T>(&self, dir: CompassDirection, grid: &Grid<T>) -> Option<Self> {
        let d = match dir {
            CompassDirection::North => Self(self.0, self.1 + 1),
            CompassDirection::NorthEast => Self(self.0 + 1, self.1 + 1),
            CompassDirection::East => Self(self.0 + 1, self.1),
            CompassDirection::SouthEast => Self(self.0 + 1, self.1 - 1),
            CompassDirection::South => Self(self.0, self.1 - 1),
            CompassDirection::SouthWest => Self(self.0 - 1, self.1 - 1),
            CompassDirection::West => Self(self.0 - 1, self.1),
            CompassDirection::NorthWest => Self(self.0 - 1, self.1 + 1),
        };
        grid.in_bounds(d)
    }

    pub fn move_until<T, F: Fn(&T) -> bool>(
        &self,
        dir: Direction,
        grid: &Grid<T>,
        predicate: F,
    ) -> (Self, i64) {
        let mut i = 0;
        let mut c = *self;
        let mut x = self.translate(dir, grid);
        while x.is_some() && !predicate(&grid[x.unwrap()]) {
            i += 1;
            c = x.unwrap();
            x = x.unwrap().translate(dir, grid);
        }
        (c, i)
    }

    pub(crate) fn new(arg1: i64, arg2: i64) -> Coord {
        Coord(arg1, arg2)
    }

    pub(crate) fn non_diagnal_distance(&self, dest: &Coord) -> i64 {
        (self.0.abs_diff(dest.0) + self.1.abs_diff(dest.1)) as i64
    }

    fn get_neighbours(&self, diagnols: bool) -> Vec<Coord> {
        if diagnols {
            vec![
                Coord::new(self.0, self.1 + 1),
                Coord::new(self.0, self.1 - 1),
                Coord::new(self.0 + 1, self.1),
                Coord::new(self.0 - 1, self.1),
                Coord::new(self.0 + 1, self.1 + 1),
                Coord::new(self.0 + 1, self.1 - 1),
                Coord::new(self.0 - 1, self.1 + 1),
                Coord::new(self.0 - 1, self.1 - 1),
            ]
        } else {
            vec![
                Coord::new(self.0, self.1 + 1),
                Coord::new(self.0, self.1 - 1),
                Coord::new(self.0 + 1, self.1),
                Coord::new(self.0 - 1, self.1),
            ]
        }
    }

    pub(crate) fn is_on_edge(&self, grid: &Grid<char>) -> bool {
        (self.0 == grid.width() as i64 - 1 || self.0 == 0)
            || (self.1 == grid.height() as i64 - 1 || self.1 == 0)
    }

    pub fn x(&self) -> i64 {
        self.0
    }

    pub fn y(&self) -> i64 {
        self.1
    }

    /// Number of times self goes into rhs
    pub fn divides(&self, rhs: Self) -> Option<i64> {
        if rhs.0 % self.0 == 0 && rhs.1 % self.1 == 0 {
            let a = rhs.0 / self.0;
            let b = rhs.1 / self.1;
            if a == b {
                Some(a)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<Coord> for Coord {
    fn add_assign(&mut self, rhs: Coord) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

pub struct CoordIterator {
    max_x: i64,
    max_y: i64,
    curr_x: i64,
    curr_y: i64,
}

impl CoordIterator {
    pub fn from_grid<T>(grid: &Grid<T>) -> Self {
        CoordIterator {
            max_x: grid.width() as i64,
            max_y: grid.height() as i64,
            curr_x: 0,
            curr_y: 0,
        }
    }
}

impl Iterator for CoordIterator {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_x < self.max_x && self.curr_y < self.max_y {
            let coord = Coord(self.curr_x, self.curr_y);
            self.curr_x += 1;
            if self.curr_x == self.max_x {
                self.curr_x = 0;
                self.curr_y += 1;
            }
            Some(coord)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, EnumIter)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl<T> Index<Direction> for Vec<T> {
    type Output = T;

    fn index(&self, index: Direction) -> &Self::Output {
        match index {
            Direction::Up => &self[0],
            Direction::Down => &self[1],
            Direction::Left => &self[2],
            Direction::Right => &self[3],
        }
    }
}

impl<T> IndexMut<Direction> for Vec<T> {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        match index {
            Direction::Up => &mut self[0],
            Direction::Down => &mut self[1],
            Direction::Left => &mut self[2],
            Direction::Right => &mut self[3],
        }
    }
}

impl Direction {
    pub fn right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    pub(crate) fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, EnumIter)]
pub enum CompassDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}
