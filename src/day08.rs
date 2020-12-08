use crate::prelude::*;

#[derive(Copy, Clone)]
struct Instruction {
    kind: InstructionKind,
    value: i32,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum InstructionKind {
    Acc,
    Nop,
    Jmp,
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .split('\n')
        .map(|line| {
            let mut iter = line.split(' ');
            let kind = match iter.next().unwrap() {
                "acc" => InstructionKind::Acc,
                "nop" => InstructionKind::Nop,
                "jmp" => InstructionKind::Jmp,
                _ => unreachable!("No other instructions exist"),
            };
            let value = iter.next().unwrap().parse().unwrap();
            Instruction { kind, value }
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(input: &[Instruction]) -> i32 {
    if let RunResult::Loop(value) = run(input) {
        value
    } else {
        panic!("The code should loop but it didn't")
    }
}

#[aoc(day8, part2)]
fn part2(input: &[Instruction]) -> i32 {
    // Simply try all modifications for nop->jmp and jmp->nop to see which terminates
    for i in 0..input.len() {
        let mut input = input.to_vec();
        input[i].kind = match input[i].kind {
            InstructionKind::Acc => continue,
            InstructionKind::Nop => InstructionKind::Jmp,
            InstructionKind::Jmp => InstructionKind::Nop,
        };

        if let RunResult::Terminate(value) = run(&input) {
            println!("{}", i);
            return value;
        }
    }
    panic!("Should have found a modification which makes the program terminate")
}

enum RunResult {
    Loop(i32),
    Terminate(i32),
    Error,
}

fn run(input: &[Instruction]) -> RunResult {
    let mut executed_instructions = Set::<usize>::new();
    let mut instruction_pointer = 0;
    let mut accumulator = 0;

    loop {
        if !executed_instructions.insert(instruction_pointer) {
            // Value was already included in the set
            return RunResult::Loop(accumulator);
        } else if instruction_pointer == input.len() {
            return RunResult::Terminate(accumulator);
        } else if instruction_pointer > input.len() {
            println!("Found error");
            return RunResult::Error;
        }
        let Instruction { kind, value } = &input[instruction_pointer];
        match kind {
            InstructionKind::Acc => accumulator += value,
            InstructionKind::Nop => {}
            InstructionKind::Jmp => {
                instruction_pointer = (instruction_pointer as i32 + value) as usize;
                continue;
            }
        };
        instruction_pointer += 1;
    }
}

#[test]
fn test_part1() {
    let values = input_generator(PUZZLE);
    assert_eq!(5, part1(&values));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day8.txt").trim());
    assert_eq!(1137, part1(&values));
}

#[test]
fn test_part2() {
    let values = input_generator(PUZZLE);
    assert_eq!(8, part2(&values));
}

#[test]
fn test_part2_solution() {
    let values = input_generator(include_str!("../input/2020/day8.txt").trim());
    assert_eq!(1125, part2(&values));
}

#[cfg(test)]
static PUZZLE: &str = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;
