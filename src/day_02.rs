use crate::TaskCompleter;
use std::fmt::Debug;

pub struct Task2;

/// Returns true if safe
fn check_for_safety(input_str: &&str) -> bool {
    let mut s = input_str.split_whitespace().into_iter();
    let first_val = s.next().unwrap().parse::<u32>().unwrap();
    let mut curr_val = s.next().unwrap().parse::<u32>().unwrap();
    let comparer = if first_val < curr_val {
        u32::lt
    } else {
        u32::gt
    };
    let diff = first_val.abs_diff(curr_val);
    if diff > 3 || diff == 0 {
        return false;
    }
    for v in s {
        let val = v.parse::<u32>().unwrap();
        let diff = curr_val.abs_diff(val);
        if !comparer(&curr_val, &val) || diff > 3 || diff == 0 {
            return false;
        }
        curr_val = val;
    }
    true
}

fn check_for_safety_with_error_area<'a, T: Iterator<Item = &'a str>>(mut s: T) -> Option<usize>
where
    T: Debug + Clone,
{
    // dbg!(s.clone().collect::<Vec<&'a str>>());
    let first_val = s.next().unwrap().parse::<u32>().unwrap();
    let mut curr_val = s.next().unwrap().parse::<u32>().unwrap();
    let comparer = if first_val < curr_val {
        u32::lt
    } else {
        u32::gt
    };
    let diff = first_val.abs_diff(curr_val);
    if diff > 3 || diff == 0 {
        return Some(0);
    }
    for (i, v) in s.enumerate() {
        let val = v.parse::<u32>().unwrap();
        let diff = curr_val.abs_diff(val);
        if !comparer(&curr_val, &val) || diff > 3 || diff == 0 {
            return Some(i + 1);
        }
        curr_val = val;
    }
    None
}

fn check_for_permissive_safety(input_str: &&str) -> bool {
    if let Some(_) = check_for_safety_with_error_area(input_str.split_whitespace()) {
        (0..input_str.split_whitespace().count())
            .into_iter()
            .map(|x| {
                check_for_safety_with_error_area(
                    input_str
                        .split_whitespace()
                        .take(x)
                        .chain(input_str.split_whitespace().skip(x + 1)),
                )
            })
            .any(|y| y.is_none())
        // let first_err = check_for_safety_with_error_area(
        //     input_str
        //         .split_whitespace()
        //         .take(error_place)
        //         .chain(input_str.split_whitespace().skip(error_place + 1)),
        // );
        // let second_err = check_for_safety_with_error_area(
        //     input_str
        //         .split_whitespace()
        //         .take(error_place + 1)
        //         .chain(input_str.split_whitespace().skip(error_place + 2)),
        // );
        // let res = first_err.is_none() || second_err.is_none();
        // if !res {
        //     println!(
        //     "On input {input_str:23} Determined error to be at {error_place} and other errors: {first_err:?}, {second_err:?}"
        // );
        // }
        // res
    } else {
        true
    }
}

impl TaskCompleter for Task2 {
    fn do_task_1(&self) -> String {
        include_str!("../input/day_02/input")
            .lines()
            .filter(check_for_safety)
            .count()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        include_str!("../input/day_02/input")
            .lines()
            .filter(check_for_permissive_safety)
            .count()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("299".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("364".to_string())
    }
}
