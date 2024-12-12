use std::collections::HashMap;

use count_digits::CountDigits;
use itertools::Either;

use crate::TaskCompleter;

pub struct Task11;

enum Rule {
    Number(i64),
    Split(Box<Rule>, Box<Rule>),
}

impl Rule {
    fn apply_rule(&mut self) {
        match self {
            Rule::Number(a) => match apply_rule(*a) {
                Either::Left(x) => *a = x,
                Either::Right((x, y)) => {
                    *self = Rule::Split(Box::new(Rule::Number(x)), Box::new(Rule::Number(y)))
                }
            },
            Rule::Split(a, b) => {
                a.apply_rule();
                b.apply_rule();
            }
        }
    }

    fn count_numbers(&self) -> i64 {
        match self {
            Rule::Number(_) => 1,
            Rule::Split(a, b) => a.count_numbers() + b.count_numbers(),
        }
    }
}

fn apply_rule(n: i64) -> Either<i64, (i64, i64)> {
    if n == 0 {
        Either::Left(1)
    } else if n.count_digits() % 2 == 0 {
        let point_of_split = 10_i64.pow((n.count_digits() / 2).try_into().unwrap());
        let c = n / point_of_split;
        let d = n % point_of_split;
        Either::Right((c, d))
    } else {
        Either::Left(n * 2024)
    }
}

fn count_numbers_after_applying_rule(
    starting_number: i64,
    number_of_times_to_apply_rule: i64,
    cache: &mut HashMap<(i64, i64), i64>,
) -> i64 {
    if let Some(i) = cache.get(&(starting_number, number_of_times_to_apply_rule)) {
        return *i;
    }
    if number_of_times_to_apply_rule <= 0 {
        return 1;
    }
    if number_of_times_to_apply_rule <= 6 {
        return count_numbers_after_applying_rule_no_cache(
            starting_number,
            number_of_times_to_apply_rule,
        );
    }
    match apply_rule(starting_number) {
        Either::Left(a) => {
            let v = count_numbers_after_applying_rule(a, number_of_times_to_apply_rule - 1, cache);
            cache.insert((starting_number, number_of_times_to_apply_rule), v);
            v
        }
        Either::Right((a, b)) => {
            let v = count_numbers_after_applying_rule(a, number_of_times_to_apply_rule - 1, cache)
                + count_numbers_after_applying_rule(b, number_of_times_to_apply_rule - 1, cache);
            cache.insert((starting_number, number_of_times_to_apply_rule), v);
            v
        }
    }
}

fn count_numbers_after_applying_rule_no_cache(
    starting_number: i64,
    number_of_times_to_apply_rule: i64,
) -> i64 {
    if number_of_times_to_apply_rule <= 0 {
        return 1;
    }
    match apply_rule(starting_number) {
        Either::Left(a) => {
            let v =
                count_numbers_after_applying_rule_no_cache(a, number_of_times_to_apply_rule - 1);
            v
        }
        Either::Right((a, b)) => {
            let v =
                count_numbers_after_applying_rule_no_cache(a, number_of_times_to_apply_rule - 1)
                    + count_numbers_after_applying_rule_no_cache(
                        b,
                        number_of_times_to_apply_rule - 1,
                    );
            v
        }
    }
}

impl TaskCompleter for Task11 {
    fn do_task_1(&self) -> String {
        let mut cache = HashMap::new();
        include_str!("../input/day_11/input")
            .split_whitespace()
            .map(|x| count_numbers_after_applying_rule(x.parse::<i64>().unwrap(), 25, &mut cache))
            .sum::<i64>()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let mut cache = HashMap::new();
        include_str!("../input/day_11/input")
            .split_whitespace()
            .map(|x| count_numbers_after_applying_rule(x.parse::<i64>().unwrap(), 75, &mut cache))
            .sum::<i64>()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("202019".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("239321955280205".to_string())
    }
}
