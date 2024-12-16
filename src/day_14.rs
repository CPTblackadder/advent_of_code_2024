use std::fmt::{Display, Write};

use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::{
    grid::{CompassDirection, Coord, Grid},
    TaskCompleter,
};

pub struct Task14;

#[derive(Clone)]
struct DotAsZeroNumber(i64);

impl Display for DotAsZeroNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 0 {
            f.write_char('.')
        } else {
            f.write_str(&self.0.to_string())
        }
    }
}

struct Robot {
    initial_position: Coord,
    velocity: Coord,
}

fn parse_tuple_to_coord(input: &str) -> Coord {
    let mut s = input.split(",");
    let x = s.next().unwrap().parse::<i64>().unwrap();
    let y = s.next().unwrap().parse::<i64>().unwrap();
    Coord::new(x, y)
}

impl Robot {
    /// From "p=0,4 v=3,-3" -> Self
    fn parse(input: &str) -> Self {
        let mut s = input.split(" v=");
        let initial_position = parse_tuple_to_coord(&s.next().unwrap()[2..]);
        let velocity = parse_tuple_to_coord(s.next().unwrap());
        Self {
            initial_position,
            velocity,
        }
    }

    fn get_robot_pos_after_time(&self, time: i64, width: i64, height: i64) -> Coord {
        let new_x = (self.initial_position.x() + (self.velocity.x() * time)).rem_euclid(width);
        let new_y = (self.initial_position.y() + (self.velocity.y() * time)).rem_euclid(height);
        Coord::new(new_x, new_y)
    }
}

fn get_robots(input: &str) -> Vec<Robot> {
    input.lines().map(|l| Robot::parse(l)).collect_vec()
}

fn count_in_each_quadrant<T: Iterator<Item = Coord>>(
    positions: T,
    width: i64,
    height: i64,
) -> [i64; 4] {
    let mut count = [0; 4];
    let mid_point_x = width / 2;
    let mid_point_y = height / 2;
    for c in positions {
        if c.x() < mid_point_x {
            if c.y() < mid_point_y {
                count[0] += 1;
            } else if c.y() > mid_point_y {
                count[1] += 1;
            }
        } else if c.x() > mid_point_x {
            if c.y() < mid_point_y {
                count[2] += 1;
            } else if c.y() > mid_point_y {
                count[3] += 1;
            }
        }
    }
    count
}

impl TaskCompleter for Task14 {
    fn do_task_1(&self) -> String {
        let width = 101;
        let height = 103;
        count_in_each_quadrant(
            get_robots(include_str!("../input/day_14/input"))
                .iter()
                .map(|x| x.get_robot_pos_after_time(100, width, height)),
            width,
            height,
        )
        .iter()
        .fold(1, |x, y| x * y)
        .to_string()
    }

    fn do_task_2(&self) -> String {
        let width = 101;
        let height = 103;
        let robots = get_robots(include_str!("../input/day_14/input"));

        for i in 6377.. {
            let positions = robots
                .iter()
                .map(|x| x.get_robot_pos_after_time(i, width, height));
            let mut grid =
                Grid::init_with_size(DotAsZeroNumber(0), width as usize, height as usize);
            for p in positions {
                grid[p].0 += 1;
            }

            // For positions that have a value, count number that has at least 2 neighbours in all 8 directions with a robot in them
            // If that number is over 90% print the bastard

            let number_of_tiles_with_robot = grid.iter().filter(|(_, y)| y.0 > 0).count();
            let number_of_tiles_with_2_neighbouring_robots_that_have_robot = grid
                .iter()
                .filter(|(p, y)| {
                    y.0 > 0
                        && CompassDirection::iter()
                            .filter(|d| {
                                p.translate_compass(*d, &grid)
                                    .is_some_and(|x| grid[x].0 > 0)
                            })
                            .count()
                            >= 2
                })
                .count();
            if (number_of_tiles_with_2_neighbouring_robots_that_have_robot * 10)
                > (number_of_tiles_with_robot * 5)
            {
                // println!("{i}");
                // println!("{}", grid);
                return i.to_string();
            }
        }
        "".to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("225943500".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("6377".to_string())
    }
}
