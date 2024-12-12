use crate::TaskCompleter;
use count_digits::CountDigits;
use rayon::prelude::*;

pub struct Task7;

enum Operation {
    Add,
    Mul,
}

struct EquationChecker {
    correct_value: i64,
    rhs: Vec<i64>,
}

fn check_value(correct_value: i64, value: i64, values: &[i64]) -> Option<i64> {
    if values.is_empty() {
        if value == correct_value {
            Some(value)
        } else {
            None
        }
    } else {
        let next_val = values[0];
        let vals = &values[1..];
        check_value(correct_value, value + next_val, vals).or_else(|| check_value(
            correct_value,
            value * next_val,
            vals,
        ))
    }
}

fn check_value_with_concat(correct_value: i64, value: i64, values: &[i64]) -> Option<i64> {
    if values.is_empty() {
        if value == correct_value {
            Some(value)
        } else {
            None
        }
    } else {
        let next_val = values[0];
        let vals = &values[1..];
        check_value_with_concat(correct_value, value + next_val, vals)
            .or_else(|| check_value_with_concat(correct_value, value * next_val, vals))
            .or_else(|| {
                check_value_with_concat(
                    correct_value,
                    {
                        let digits = next_val.count_digits() as u32;
                        (value * 10_i64.pow(digits)) + next_val
                    },
                    vals,
                )
            })
    }
}

impl EquationChecker {
    fn from_line(line: &str) -> Self {
        let mut s = line.split(":");
        let correct_value = s.next().unwrap().parse::<i64>().unwrap();
        let rhs = s
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        EquationChecker { correct_value, rhs }
    }

    fn get_correct_value_if_possible_with_concat(&self) -> i64 {
        if let Some(r) = check_value_with_concat(self.correct_value, 0, &self.rhs) {
            r
        } else {
            0
        }
    }

    fn get_correct_value_if_possible(&self) -> i64 {
        if let Some(r) = check_value(self.correct_value, 0, &self.rhs) {
            r
        } else {
            0
        }
    }
}

impl TaskCompleter for Task7 {
    fn do_task_1(&self) -> String {
        include_str!("../input/day_07/input")
            .lines()
            .map(|line| EquationChecker::from_line(line).get_correct_value_if_possible())
            .sum::<i64>()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        include_str!("../input/day_07/input")
            .lines()
            .par_bridge()
            .map(|line| {
                EquationChecker::from_line(line).get_correct_value_if_possible_with_concat()
            })
            .sum::<i64>()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("42283209483350".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("1026766857276279".to_string())
    }
}
