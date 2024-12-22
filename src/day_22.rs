use std::{i64, iter::repeat};

use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::TaskCompleter;

#[cfg(test)]
mod tests;

pub struct Task22;

//    Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
//    Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
//    Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.
fn get_next_secret(secret: i64) -> i64 {
    let step1 = ((secret * 64) ^ secret) % 16777216;
    let step2 = ((step1 / 32) ^ step1) % 16777216;
    ((step2 * 2048) ^ step2) % 16777216
}

struct SecretIterator {
    secret: i64,
}

impl Iterator for SecretIterator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.secret = get_next_secret(self.secret);
        Some(self.secret)
    }
}

impl SecretIterator {
    fn new(secret: i64) -> Self {
        Self { secret }
    }
}

struct PriceChangeIterator {
    secret: i64,
}

impl PriceChangeIterator {
    fn new(secret: i64) -> Self {
        Self { secret }
    }
    fn from_string(i: &str) -> Self {
        Self {
            secret: i.parse::<i64>().unwrap(),
        }
    }

    fn highest_value_given_price_changes(
        &mut self,
        price_changes_to_look_for: (i64, i64, i64, i64),
        max_iter: i64,
    ) -> i64 {
        let mut i = max_iter - 4;
        let mut curr_price_changes = (
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
            self.next().unwrap(),
        );

        while i > 0 {
            if price_changes_to_look_for == curr_price_changes {
                return self.secret % 10;
            }
            curr_price_changes.0 = curr_price_changes.1;
            curr_price_changes.1 = curr_price_changes.2;
            curr_price_changes.2 = curr_price_changes.3;
            curr_price_changes.3 = self.next().unwrap();
            i -= 1;
        }
        0
    }
}

impl Iterator for PriceChangeIterator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let old_price = self.secret % 10;
        self.secret = get_next_secret(self.secret);
        let new_price = self.secret % 10;
        Some(new_price - old_price)
    }
}

impl TaskCompleter for Task22 {
    fn do_task_1(&self) -> String {
        let lines = include_str!("../input/day_22/input").lines();
        lines
            .map(|x| {
                SecretIterator::new(x.parse::<i64>().unwrap())
                    .nth(1999)
                    .unwrap()
            })
            .sum::<i64>()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let lines = include_str!("../input/day_22/input").lines();
        // let (a, b, c, d) = itertools::iproduct!(-9..9, -9..9, -9..9, -9..9)
        //     .par_bridge()
        //     .map(|x| {
        //         (
        //             x,
        //             lines
        //                 .clone()
        //                 .map(move |i| {
        //                     PriceChangeIterator::from_string(i)
        //                         .highest_value_given_price_changes(x, 2000)
        //                 })
        //                 .sum::<i64>(),
        //         )
        //     })
        //     .max_by_key(|(_, x)| *x)
        //     .unwrap()
        //     .0;
        lines
            .map(|i| {
                PriceChangeIterator::from_string(i)
                    .highest_value_given_price_changes((0, 2, -2, 3), 2000)
            })
            .sum::<i64>()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("20441185092".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        None
    }
}
