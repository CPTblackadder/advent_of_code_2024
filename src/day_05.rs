use std::{cmp::Ordering, collections::HashMap};

use crate::TaskCompleter;

pub struct Task5;

struct OrderingRule {
    before: i64,
    after: i64,
}

// Rules is a hash from a before value, to all of it's after values
fn check_ordering(manual: &Vec<i64>, rules: &HashMap<i64, Vec<i64>>) -> bool {
    let mut seen = vec![];
    for i in manual {
        if rules
            .get(i)
            .is_some_and(|y| y.iter().any(|x| seen.contains(x)))
        {
            return false;
        }
        seen.push(*i);
    }
    true
}

impl TaskCompleter for Task5 {
    fn do_task_1(&self) -> String {
        let mut doing_rules = true;
        let mut rules = vec![];
        let mut afters_hash: HashMap<i64, Vec<i64>> = HashMap::new();
        let mut befores_hash: HashMap<i64, Vec<i64>> = HashMap::new();
        let mut manuals = vec![];
        for l in include_str!("../input/day_05/input").lines() {
            if doing_rules {
                if l == "" {
                    doing_rules = false;
                } else {
                    let mut s = l.split("|");
                    let before = s.next().unwrap().parse::<i64>().unwrap();
                    let after = s.next().unwrap().parse::<i64>().unwrap();
                    afters_hash.entry(after).or_default().push(before);
                    befores_hash.entry(before).or_default().push(after);
                    rules.push(OrderingRule { before, after });
                }
            } else {
                let s = l
                    .split(",")
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
                manuals.push(s);
            }
        }
        manuals
            .iter()
            .filter(|x| check_ordering(x, &befores_hash))
            .map(|x| x[x.len() / 2])
            .sum::<i64>()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let mut doing_rules = true;
        let mut rules = vec![];
        let mut afters_hash: HashMap<i64, Vec<i64>> = HashMap::new();
        let mut befores_hash: HashMap<i64, Vec<i64>> = HashMap::new();
        let mut manuals = vec![];
        for l in include_str!("../input/day_05/input").lines() {
            if doing_rules {
                if l == "" {
                    doing_rules = false;
                } else {
                    let mut s = l.split("|");
                    let before = s.next().unwrap().parse::<i64>().unwrap();
                    let after = s.next().unwrap().parse::<i64>().unwrap();
                    afters_hash.entry(after).or_default().push(before);
                    befores_hash.entry(before).or_default().push(after);
                    rules.push(OrderingRule { before, after });
                }
            } else {
                let s = l
                    .split(",")
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
                manuals.push(s);
            }
        }

        manuals
            .iter_mut()
            .filter(|x| !check_ordering(x, &befores_hash))
            .map(|x| {
                x.sort_by(|x, y| {
                    if befores_hash.get(x).is_some_and(|v| v.contains(y)) {
                        Ordering::Less
                    } else if afters_hash.get(x).is_some_and(|v| v.contains(y)) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                });
                x
            })
            .map(|x| x[x.len() / 2])
            .sum::<i64>()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("4774".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("6004".to_string())
    }
}
