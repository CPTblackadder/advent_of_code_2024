use crate::TaskCompleter;
use cached::proc_macro::cached;
use cached::SizedCache;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub struct Task19;

fn can_make_pattern(line: &str, patterns: &Vec<&str>) -> bool {
    if line.len() == 0 {
        true
    } else {
        patterns
            .iter()
            .any(|x| line.starts_with(x) && can_make_pattern(&line[x.len()..], patterns))
    }
}

#[cached(
    ty = "SizedCache<String, i64>",
    create = "{ SizedCache::with_size(65) }",
    convert = r#"{ format!("{}", line) }"#
)]
fn count_pattern_possibilities(line: &str, patterns: &Vec<&str>) -> i64 {
    if line.len() == 0 {
        1
    } else {
        patterns
            .par_iter()
            .map(|x| {
                if line.starts_with(x) {
                    count_pattern_possibilities(&line[x.len()..], patterns)
                } else {
                    0
                }
            })
            .sum::<i64>()
    }
}

impl TaskCompleter for Task19 {
    fn do_task_1(&self) -> String {
        let mut lines = include_str!("../input/day_19/input").lines();
        let patterns = lines.next().unwrap().split(", ").collect_vec();
        lines.next();

        lines
            .filter_map(|x| {
                if can_make_pattern(x, &patterns) {
                    Some(())
                } else {
                    None
                }
            })
            .count()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let mut lines = include_str!("../input/day_19/input").lines();
        let patterns = lines.next().unwrap().split(", ").collect_vec();
        lines.next();

        lines
            .map(|x| count_pattern_possibilities(x, &patterns))
            .sum::<i64>()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("258".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("632423618484345".to_string())
    }
}
