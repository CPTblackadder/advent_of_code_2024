use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{grid::Coord, TaskCompleter};

pub struct Task13;

const COST_A: i64 = 3;
const COST_B: i64 = 1;

#[derive(Default, Debug)]
struct ClawMachine {
    a_press: Coord,
    b_press: Coord,
    prize: Coord,
}

impl ClawMachine {
    fn get_min_cost_for_prize_a_star(&self) -> Option<i64> {
        pathfinding::directed::astar::astar(
            &Coord::default(),
            |x| {
                vec![(*x + self.a_press, COST_A), (*x + self.b_press, COST_B)]
                    .into_iter()
                    .filter(|(c, _)| c <= &self.prize)
            },
            |_| 0,
            |x| x == &self.prize,
        )
        .map(|(_, x)| x)
    }

    fn get_min_cost_for_prize_custom(&self) -> Option<i64> {
        let mut v = Coord::default();
        let mut c = 0;

        while v <= self.prize {
            
        }

        None
    }
}

fn parse_numbers(i: &str) -> Coord {
    let mut split = i.split(", ");
    let x = split.next().unwrap()[2..].parse::<i64>().unwrap();
    let y = split.next().unwrap()[2..].parse::<i64>().unwrap();
    Coord::new(x, y)
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    let mut claw_machines = vec![];
    let mut lines = input.lines();

    loop {
        let button_a = lines.next().unwrap();
        assert!(button_a.starts_with("Button A: "), "{:?}", button_a);
        let button_b = lines.next().unwrap();
        assert!(button_b.starts_with("Button B: "), "{:?}", button_b);
        let prize = lines.next().unwrap();
        assert!(prize.starts_with("Prize: "), "{:?}", prize);
        let empty_line = lines.next();
        claw_machines.push(ClawMachine {
            a_press: parse_numbers(&button_a[10..]),
            b_press: parse_numbers(&button_b[10..]),
            prize: parse_numbers(&prize[7..]),
        });

        if empty_line.is_none() {
            break;
        }
    }
    claw_machines
}

impl TaskCompleter for Task13 {
    fn do_task_1(&self) -> String {
        let machines = parse_input(include_str!("../input/day_13/input"));
        machines
            .par_iter()
            .map(|x| x.get_min_cost_for_prize_a_star().unwrap_or_default())
            .sum::<i64>()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let mut machines = parse_input(include_str!("../input/day_13/example"));
        for m in machines.iter_mut() {
            m.prize = m.prize + Coord::new(10000000000000, 10000000000000);
        }
        machines
            .par_iter()
            .map(|x| x.get_min_cost_for_prize_a_star().unwrap_or_default())
            .sum::<i64>()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("37297".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        None
    }
}
