use itertools::Itertools;
use pathfinding::prelude::astar;

use crate::{
    grid::{Coord, Grid},
    TaskCompleter,
};

pub struct Task18;

fn create_grid(input: &str, dimension: usize, bytes_to_use: usize) -> Grid<char> {
    let mut g = Grid::init_with_size('.', dimension, dimension);
    for l in input.lines().take(bytes_to_use) {
        let mut s = l.split(",");
        let x = s.next().unwrap().parse::<i64>().unwrap();
        let y = s.next().unwrap().parse::<i64>().unwrap();

        g[Coord::new(x, dimension as i64 - y - 1)] = '#';
    }
    g
}

impl TaskCompleter for Task18 {
    fn do_task_1(&self) -> String {
        let mut g = create_grid(include_str!("../input/day_18/input"), 71, 1024);
        let start = Coord::new(0, 70);
        let finish = Coord::new(70, 0);
        let path = astar(
            &start,
            |x| {
                x.get_bounded_neighbours(&g, false)
                    .iter()
                    .filter(|x| g[**x] != '#')
                    .map(|x| (*x, 1))
                    .collect_vec()
            },
            |x| x.non_diagnal_distance(&finish),
            |x| *x == finish,
        );
        path.unwrap().1.to_string()
    }

    fn do_task_2(&self) -> String {
        let dimension = 71;
        let bytes_to_use = 1024; // use the right line so we can immediately return to cut down on computation
        let s = include_str!("../input/day_18/input");
        let mut g = create_grid(s, dimension, bytes_to_use);
        let start = Coord::new(0, dimension as i64 - 1);
        let finish = Coord::new(dimension as i64 - 1, 0);
        for l in s.lines().skip(1024) {
            let mut s = l.split(",");
            let x = s.next().unwrap().parse::<i64>().unwrap();
            let y = s.next().unwrap().parse::<i64>().unwrap();
            let c = Coord::new(x, dimension as i64 - y - 1);
            g[c] = '#';
            if astar(
                &start,
                |x| {
                    x.get_bounded_neighbours(&g, false)
                        .iter()
                        .filter(|x| g[**x] != '#')
                        .map(|x| (*x, 1))
                        .collect_vec()
                },
                |x| x.non_diagnal_distance(&finish),
                |x| *x == finish,
            ) == None
            {
                return l.to_string();
            }
        }
        "Error".to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("326".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        None
    }
}
