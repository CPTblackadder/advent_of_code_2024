use crate::{grid::Coord, TaskCompleter};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub struct Task13;

const COST_A: i64 = 3;
const COST_B: i64 = 1;

#[derive(Default, Debug)]
struct ClawMachine {
    a_press: Coord,
    b_press: Coord,
    prize: Coord,
}

fn determinant(a: i64, b: i64, c: i64, d: i64) -> i64 {
    return a * d - b * c;
}

impl ClawMachine {
    fn get_min_cost_for_prize_custom(&self) -> Option<i64> {
        let mut v = Coord::default();
        let mut c = 0;
        let mut possible_answers = vec![];
        while v <= self.prize {
            if let Some(i) = self.b_press.divides(self.prize - v) {
                possible_answers.push(c + (COST_B * i));
            }
            v += self.a_press;
            c += COST_A;
        }
        possible_answers.into_iter().min()
    }

    /**
     *  Couple of possibilities:
     *  Either this is a system of linear equations with one solution
     *  
     *  Or both button presses are multiples of each other (this also doesn't happen in the input data)
     *
     *  Or a button press moves only X or Y (this doesn't happen in input data so we ignore that)
     */
    fn get_min_cost_for_prize(&self) -> Option<i64> {
        let t = (self.prize.x() * self.a_press.y()) - (self.prize.y() * self.a_press.x());
        let b = (self.b_press.x() * self.a_press.y()) - (self.b_press.y() * self.a_press.x());
        if t % b == 0 {
            let b_presses = t / b;
            let t = self.prize.y() - (b_presses * self.b_press.y());
            let b = self.a_press.y();
            if t % b == 0 {
                let a_presses = t / b;
                Some((a_presses * COST_A) + (b_presses * COST_B))
            } else {
                None
            }
        } else {
            None
        }
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
            .map(|x| x.get_min_cost_for_prize().unwrap_or_default())
            .sum::<i64>()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let mut machines = parse_input(include_str!("../input/day_13/input"));
        for m in machines.iter_mut() {
            m.prize = m.prize + Coord::new(10000000000000, 10000000000000);
        }
        machines
            .par_iter()
            .map(|x| x.get_min_cost_for_prize().unwrap_or_default())
            .sum::<i64>()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("37297".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("83197086729371".to_string())
    }
}
