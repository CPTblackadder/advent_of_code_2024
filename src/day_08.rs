use std::collections::HashMap;

use crate::{grid::*, TaskCompleter};

pub struct Task8;

impl TaskCompleter for Task8 {
    fn do_task_1(&self) -> String {
        let grid = Grid::<char>::from_string(include_str!("../input/day_08/input"), true);
        let mut anti_nodes = Grid::init_with_size('.', grid.width(), grid.height());
        let mut antennas: HashMap<char, Vec<Coord>> = HashMap::new();

        for (coord, char) in grid.iter() {
            if *char != '.' && *char != '#' {
                antennas.entry(*char).or_default().push(coord);
            }
        }
        for (_, v) in antennas.iter() {
            for (c1, c2) in itertools::iproduct!(v.iter(), v.iter()) {
                if c1 != c2 {
                    let c3 = anti_nodes.in_bounds(*c1 + *c1 - *c2);
                    let c4 = anti_nodes.in_bounds(*c2 + *c2 - *c1);
                    if let Some(c) = c3 {
                        anti_nodes[c] = '#'
                    }
                    if let Some(c) = c4 {
                        anti_nodes[c] = '#'
                    }
                }
            }
        }

        anti_nodes
            .iter()
            .filter(|(_, x)| **x == '#')
            .count()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let grid = Grid::<char>::from_string(include_str!("../input/day_08/input"), true);
        let mut anti_nodes = Grid::init_with_size('.', grid.width(), grid.height());
        let mut antennas: HashMap<char, Vec<Coord>> = HashMap::new();

        for (coord, char) in grid.iter() {
            if *char != '.' && *char != '#' {
                antennas.entry(*char).or_default().push(coord);
            }
        }
        for (_, v) in antennas.iter() {
            for (c1, c2) in itertools::iproduct!(v.iter(), v.iter()) {
                if c1 != c2 {
                    let translate_by = *c2 - *c1;
                    let mut c3 = Some(*c1); //anti_nodes.in_bounds(*c1 + *c1 - *c2);
                    while let Some(x) = c3 {
                        anti_nodes[x] = '#';
                        c3 = anti_nodes.in_bounds(x - translate_by);
                    }
                    c3 = Some(*c2); //anti_nodes.in_bounds(*c1 + *c1 - *c2);
                    while let Some(x) = c3 {
                        anti_nodes[x] = '#';
                        c3 = anti_nodes.in_bounds(x + translate_by);
                    }
                }
            }
        }

        anti_nodes
            .iter()
            .filter(|(_, x)| **x == '#')
            .count()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("351".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("1259".to_string())
    }
}
