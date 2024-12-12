use crate::TaskCompleter;

pub struct Task1;

impl TaskCompleter for Task1 {
    fn do_task_1(&self) -> String {
        let (mut v1, mut v2): (Vec<u32>, Vec<u32>) = include_str!("../input/day_01/input")
            .lines()
            .map(|x| {
                let mut s = x.split("   ");
                (
                    s.next().unwrap().parse::<u32>().unwrap(),
                    s.next().unwrap().parse::<u32>().unwrap(),
                )
            })
            .unzip();
        v1.sort();
        v2.sort();
        (0..v1.len())
            .into_iter()
            .map(|i| v1[i].abs_diff(v2[i]))
            .sum::<u32>()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let (v1, v2): (Vec<u32>, Vec<u32>) = include_str!("../input/day_01/input")
            .lines()
            .map(|x| {
                let mut s = x.split("   ");
                (
                    s.next().unwrap().parse::<u32>().unwrap(),
                    s.next().unwrap().parse::<u32>().unwrap(),
                )
            })
            .unzip();
        v1.iter()
            .map(|x| *x as usize * v2.iter().filter(|y| *y == x).count())
            .sum::<usize>()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("2031679".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("19678534".to_string())
    }
}
