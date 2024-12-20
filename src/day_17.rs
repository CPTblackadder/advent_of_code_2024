use std::fmt::Display;

use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::TaskCompleter;

pub struct Task17;
#[cfg(test)]
mod tests;

#[derive(Clone)]
struct Program {
    instruction_pointer: usize,
    register_a: i64,
    register_b: i64,
    register_c: i64,
    program: Vec<u8>,
    output: Vec<u8>,
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Instruction Pointer: {}", self.instruction_pointer)?;
        writeln!(f, "Register A: {}", self.register_a)?;
        writeln!(f, "Register B: {}", self.register_b)?;
        writeln!(f, "Register C: {}", self.register_c)?;
        write!(f, "Program Even: ")?;
        for i in 0..self.program.len() / 2 {
            let literal_operand = self.program[i * 2 + 1].to_string();
            let combo_operand = match self.program[i * 2 + 1] {
                0 | 1 | 2 | 3 => self.program[i * 2 + 1].to_string(),
                4 => "A".to_string(),
                5 => "B".to_string(),
                6 => "C".to_string(),
                7 => {
                    // Shouldn't ever be in output
                    "N".to_string()
                }
                _ => panic!("Invalid combo operand"),
            };
            let (instruction, operand) = match self.program[i * 2] {
                0 => ("adv".to_string(), combo_operand),
                1 => ("bxl".to_string(), literal_operand),
                2 => ("bst".to_string(), combo_operand),
                3 => ("jnz".to_string(), literal_operand),
                4 => ("bxc".to_string(), " ".to_string()),
                5 => ("out".to_string(), combo_operand),
                6 => ("bdv".to_string(), combo_operand),
                7 => ("cdv".to_string(), combo_operand),
                _ => panic!("Invalid program instruction"),
            };
            write!(f, "{} {}, ", instruction, operand)?;
        }
        writeln!(f, "")?;
        writeln!(f, "Output: {}", self.read_output())
    }
}

impl Program {
    fn new(register_a: i64, register_b: i64, register_c: i64, program: Vec<u8>) -> Self {
        Self {
            instruction_pointer: 0,
            register_a,
            register_b,
            register_c,
            program,
            output: vec![],
        }
    }

    fn from_string(input: &str) -> Self {
        let mut l = input.lines();
        let register_a = l.next().unwrap()[12..].parse::<i64>().unwrap();
        let register_b = l.next().unwrap()[12..].parse::<i64>().unwrap();
        let register_c = l.next().unwrap()[12..].parse::<i64>().unwrap();
        l.next();
        let program = l.next().unwrap()[9..]
            .split(",")
            .map(|x| x.parse::<u8>().unwrap())
            .collect_vec();
        Self::new(register_a, register_b, register_c, program)
    }

    fn perform_instruction(&mut self, instruction: u8, operand: u8) {
        match instruction {
            0 => {
                // adv
                let numberator = self.register_a;
                let denominator = 2_i64.pow(self.get_combo(operand) as u32);
                self.register_a = numberator / denominator;
                self.instruction_pointer += 2;
            }
            1 => {
                // bxl
                self.register_b = self.register_b ^ operand as i64;
                self.instruction_pointer += 2;
            }
            2 => {
                // bst
                self.register_b = self.get_combo(operand) % 8;
                self.instruction_pointer += 2;
            }
            3 => {
                // jnz
                if self.register_a != 0 {
                    if operand % 2 == 1 {
                        println!("Setting instruction pointer to odd value");
                    }
                    self.instruction_pointer = operand as usize;
                } else {
                    self.instruction_pointer += 2;
                }
            }
            4 => {
                // bxc
                self.register_b = self.register_b ^ self.register_c;
                self.instruction_pointer += 2;
            }
            5 => {
                // out
                self.output.push((self.get_combo(operand) % 8) as u8);
                self.instruction_pointer += 2;
            }
            6 => {
                // bdv
                let numberator = self.register_a;
                let denominator = 2_i64.pow(self.get_combo(operand) as u32);
                self.register_b = numberator / denominator;
                self.instruction_pointer += 2;
            }
            7 => {
                // cdv
                let numberator = self.register_a;
                let denominator = 2_i64.pow(self.get_combo(operand) as u32);
                self.register_c = numberator / denominator;
                self.instruction_pointer += 2;
            }
            _ => panic!("Invalid program instruction"),
        }
    }

    fn run_instruction(&mut self) -> Option<()> {
        let instruction = *self.program.get(self.instruction_pointer)?;
        let operand = *self.program.get(self.instruction_pointer + 1)?;
        self.perform_instruction(instruction, operand);
        Some(())
    }

    fn run_instruction_check_output_matches_program(&mut self) -> Option<()> {
        let instruction = *self.program.get(self.instruction_pointer)?;
        let operand = *self.program.get(self.instruction_pointer + 1)?;
        self.perform_instruction(instruction, operand);
        if instruction == 5 {
            // Check added output is correct
            let i = self.output.len() - 1;
            if i >= self.program.len() || self.output[i] != self.program[i] {
                return None;
            }
        }
        Some(())
    }

    fn read_output(&self) -> String {
        self.output.iter().map(|x| x.to_string()).join(",")
    }

    fn run_till_halting(&mut self) -> String {
        while let Some(()) = self.run_instruction() {}
        self.read_output()
    }

    fn run_till_halting_then_check_if_output_is_program(&mut self) -> bool {
        while let Some(()) = self.run_instruction_check_output_matches_program() {}
        self.program == self.output
    }

    fn get_combo(&self, value: u8) -> i64 {
        match value {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Invalid value"),
        }
    }
}

fn search_for_task_2() -> String {
    let p = Program::from_string(include_str!("../input/day_17/input"));
    // let mut program = Program {
    //     register_a: 281474976710657,
    //     ..p.clone()
    // };
    // println!("{}", program);
    // let res = program.run_till_halting();
    // println!("{}", program);
    // return res.to_string();
    // let starting_value =  281474976710656;
    let ending_value = 2251799813685248;
    let result = (281475720000000..ending_value)
        .par_bridge()
        .find_first(|i| {
            if i % 10000000 == 0 {
                println!("Got to {i}");
            }
            let mut program = p.clone();
            program.register_a = *i;
            program.run_till_halting_then_check_if_output_is_program()
        });
    result.map_or("Not Found".to_string(), |x| x.to_string())
}

fn get_task_2_working_backwards() -> String {
    let p = Program::from_string(include_str!("../input/day_17/input"));
    let mut a = 0;
    let mut v = vec![];
    for o in p.program.iter().rev() {
        v.insert(0, *o);
        let mut found = false;
        for i in 0..8 {
            let possible_a = (a << 3) + i;
            println!("possible_a: {possible_a:b}");
            let mut program = p.clone();
            program.register_a = possible_a;
            program.run_till_halting();
            println!("{}", program);
            if program.output == v {
                found = true;
                a = possible_a;
                break;
            }
        }
        if !found {
            dbg!(a);
            dbg!(v);
            panic!("Expected a bloody thingy here")
        }
    }
    a.to_string()
}

fn get_task_2_working_backwards_rec(required_output: Vec<u8>, a: i64) -> Option<i64> {
    let p = Program::from_string(include_str!("../input/day_17/input"));
    let mut r = required_output.clone();
    let to_find = r.pop().unwrap();
    for i in 0..8 {
        let possible_a = (a << 3) + i;
        // println!("possible_a: {possible_a:b}");
        let mut program = p.clone();
        program.register_a = possible_a;
        program.run_till_halting();
        // println!("{}", program);
        if program.output[0] == to_find {
            if r.len() > 0 {
                if let Some(x) = get_task_2_working_backwards_rec(r.clone(), possible_a) {
                    return Some(x);
                }
            } else {
                return Some(possible_a);
            }
        }
    }
    None
}

impl TaskCompleter for Task17 {
    fn do_task_1(&self) -> String {
        let mut p = Program::from_string(include_str!("../input/day_17/input"));
        p.run_till_halting()
    }

    fn do_task_2(&self) -> String {
        get_task_2_working_backwards_rec(vec![2, 4, 1, 5, 7, 5, 0, 3, 1, 6, 4, 3, 5, 5, 3, 0], 0)
            .unwrap()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("6,5,7,4,5,7,3,1,0".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("105875099912602".to_string())
    }
}
