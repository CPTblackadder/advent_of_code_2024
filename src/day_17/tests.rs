use crate::day_17::Program;

#[test]
/// If register C contains 9, the program 2,6 would set register B to 1.
fn test_1() {
    let mut program = Program::new(0, 0, 9, vec![2, 6]);
    assert_eq!(program.run_instruction(), Some(()));
    assert_eq!(program.register_b, 1)
}

#[test]
/// If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
fn test_2() {
    let mut program = Program::new(10, 0, 0, vec![5, 0, 5, 1, 5, 4]);
    assert_eq!(program.run_till_halting(), "0,1,2");
}

#[test]
/// If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
fn test_3() {
    let mut program = Program::new(2024, 0, 0, vec![0, 1, 5, 4, 3, 0]);
    assert_eq!(program.run_till_halting(), "4,2,5,6,7,7,7,7,3,1,0");
    assert_eq!(program.register_a, 0);
}
#[test]
/// If register B contains 29, the program 1,7 would set register B to 26.
fn test_4() {
    let mut program = Program::new(0, 29, 0, vec![1, 7]);
    program.run_instruction();
    assert_eq!(program.register_b, 26);
}
#[test]
/// If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
fn test_5() {
    let mut program = Program::new(0, 2024, 43690, vec![4, 0]);
    program.run_instruction();
    assert_eq!(program.register_b, 44354);
}

#[test]
fn test_adv() {
    let mut program = Program::new(10, 0, 0, vec![0, 5]);
    program.run_instruction();
    assert_eq!(program.register_a, 10);
    program.register_b = 1;
    program.instruction_pointer = 0;
    program.run_instruction();
    assert_eq!(program.register_a, 5);
    program.instruction_pointer = 0;
    program.run_instruction();
    assert_eq!(program.register_a, 2);
}

#[test]
fn test_part_two() {
    let instruction = vec![0, 3, 5, 4, 3, 0];
    let mut program = Program::new(117440, 0, 0, instruction);
    assert!(program.run_till_halting_then_check_if_output_is_program())
}
