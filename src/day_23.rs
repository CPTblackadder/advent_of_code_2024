use core::num;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use bit_set::BitSet;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::TaskCompleter;

pub struct Task23;

struct Graph<'a>(HashMap<&'a str, HashSet<&'a str>>);

impl<'a> Graph<'a> {
    fn from_str(input: &'a str) -> Self {
        let mut h = HashMap::new();

        fn add_to_map<'a>(a: &'a str, b: &'a str, h: &mut HashMap<&'a str, HashSet<&'a str>>) {
            if let Some(a) = h.get_mut(a) {
                if !a.contains(&b) {
                    a.insert(b);
                }
            } else {
                h.insert(a, HashSet::from([b]));
            }
        }
        for l in input.lines() {
            let node_1 = &l[0..2];
            let node_2 = &l[3..5];
            add_to_map(node_1, node_2, &mut h);
            add_to_map(node_2, node_1, &mut h);
        }
        Self(h)
    }
}

struct BitGraph<'a> {
    map: HashMap<usize, BitSet>,
    string_mapping: HashMap<&'a str, usize>,
    number_mapping: Vec<&'a str>,
}

impl<'a> BitGraph<'a> {
    fn from_string(input: &'a str) -> Self {
        let mut map = HashMap::new();
        let mut string_mapping = HashMap::new();
        let mut number_mapping = vec![];
        fn add_to_map<'a>(a: usize, b: usize, h: &mut HashMap<usize, BitSet>) {
            if let Some(a) = h.get_mut(&a) {
                if !a.contains(b) {
                    a.insert(b);
                }
            } else {
                let mut x = BitSet::new();
                x.insert(b);
                h.insert(a, x);
            }
        }
        fn get_number<'a>(
            str_map: &mut HashMap<&'a str, usize>,
            i: &'a str,
            number_mapping: &mut Vec<&'a str>,
        ) -> usize {
            if let Some(s) = str_map.get(i) {
                *s
            } else {
                let l = number_mapping.len();
                number_mapping.push(i);
                str_map.insert(i, l);
                l
            }
        }
        for l in input.lines() {
            let node_1 = &l[0..2];
            let node_2 = &l[3..5];
            let node_1 = get_number(&mut string_mapping, node_1, &mut number_mapping);
            let node_2 = get_number(&mut string_mapping, node_2, &mut number_mapping);
            add_to_map(node_1, node_2, &mut map);
            add_to_map(node_2, node_1, &mut map);
        }

        println!(
            "Graph created with {} nodes and the largest number of neighbours is {}\nAverage number of neighbours is {}",
            string_mapping.len(),
            map.iter().max_by_key(|(_, z)| z.len()).unwrap().1.len(),
            map.iter().map(|(_,y)|y.len()).sum::<usize>() / string_mapping.len()
        );

        Self {
            map,
            string_mapping,
            number_mapping,
        }
    }
}

fn is_valid_set(graph: &BitGraph, set: &BitSet) -> bool {
    set.iter().all(|x| graph.map[&x].is_superset(set))
}

fn get_largest_set_for_set(graph: &BitGraph, in_set: BitSet, to_check: BitSet) -> BitSet {
    if let Some(s) = to_check
        .iter()
        .filter_map(|i| {
            if graph.map[&i].is_superset(&in_set) {
                let mut new = in_set.clone();
                new.insert(i);
                let mut to_ch = to_check.clone();
                to_ch.remove(i);
                Some(get_largest_set_for_set(graph, new, to_ch))
            } else {
                None
            }
        })
        .max_by_key(|x| x.len())
    {
        s
    } else {
        in_set
    }
}

fn find_largest_valid_set(graph: &BitGraph, set: BitSet) -> BitSet {
    if is_valid_set(graph, &set) {
        if set.len() >= 1 {
            dbg!(&set);
        }
        set
    } else {
        set.iter()
            .map(|i| {
                let mut new_set = set.clone();
                new_set.remove(i);
                find_largest_valid_set(graph, new_set)
            })
            .max_by_key(|x| x.len())
            .unwrap()
    }
}

fn get_largest_set_for_item(graph: &BitGraph, item: usize) -> BitSet {
    println!("{item}, has {} neighbours", graph.map[&item].len());
    dbg!(find_largest_valid_set(graph, graph.map[&item].clone()))
}

fn get_largest_set<'a>(graph: BitGraph<'a>) -> HashSet<&'a str> {
    let set = graph
        .map
        .iter()
        .map(|(x, _)| get_largest_set_for_item(&graph, *x))
        .max_by_key(|x| x.len())
        .unwrap();
    HashSet::from_iter(set.iter().map(|x| graph.number_mapping[x]))
}

fn get_largest_set_two<'a>(graph: BitGraph<'a>) -> HashSet<&'a str> {
    let mut pq = PriorityQueue::new();
    for (i, q) in graph.map.iter() {
        let items: BitSet = BitSet::from_iter([*i]);
        let p = items.len() + q.len();
        pq.push((items, q.clone()), p);
    }

    let mut biggest_set = BitSet::new();
    while let Some(((set, neighbours), p)) = pq.pop() {
        if p <= biggest_set.len() {
            break;
        }
        for n in neighbours.iter() {
            let mut new_set = set.clone();
            let mut new_neighbours = neighbours.clone();
            new_set.insert(n);
            new_neighbours.intersect_with(&graph.map[&n]);
            if new_set.len() > biggest_set.len() {
                biggest_set = new_set.clone();
            }
            if new_neighbours.len() > 0 {
                let p = new_set.len() + new_neighbours.len();
                pq.push((new_set, new_neighbours), p);
            }
        }
    }

    HashSet::from_iter(biggest_set.iter().map(|x| graph.number_mapping[x]))
}

impl TaskCompleter for Task23 {
    fn do_task_1(&self) -> String {
        let g = Graph::from_str(include_str!("../input/day_23/input"));
        let mut sets_of_3 = HashSet::new();
        let filter = |x: &[&&str; 3]| x.iter().any(|y| y.starts_with("t"));
        for (node, neighbours) in g.0.iter() {
            for neighbour in neighbours {
                let shared = neighbours.intersection(&g.0[neighbour]);
                for neighbour2 in shared {
                    let mut set = [node, neighbour, neighbour2];
                    if filter(&set) {
                        set.sort();
                        sets_of_3.insert(set);
                    }
                }
            }
        }

        sets_of_3.len().to_string()
    }

    fn do_task_2(&self) -> String {
        let g = BitGraph::from_string(include_str!("../input/day_23/input"));

        let mut res = get_largest_set_two(g).into_iter().collect_vec();
        res.sort();
        res.join(",")
        // "".to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        None
    }

    fn task_2_result(&self) -> Option<String> {
        None
    }
}
