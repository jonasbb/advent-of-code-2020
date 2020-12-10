//! [Day 8: Handheld Halting](https://adventofcode.com/2020/day/8)
//!
//! # Part 1
//!
//! Your flight to the major airline hub reaches cruising altitude without incident.
//! While you consider checking the in-flight menu for one of those drinks that come with a little umbrella, you are interrupted by the kid sitting next to you.
//!
//! Their [handheld game console](https://en.wikipedia.org/wiki/Handheld_game_console) won't turn on!
//! They ask if you can take a look.
//!
//! You narrow the problem down to a strange **infinite loop** in the boot code (your puzzle input) of the device.
//! You should be able to fix it, but first you need to be able to run the code in isolation.
//!
//! The boot code is represented as a text file with one **instruction** per line of text.
//! Each instruction consists of an **operation** (`acc`, `jmp`, or `nop`) and an **argument** (a signed number like `+4` or `-20`).
//!
//! - `acc` increases or decreases a single global value called the **accumulator** by the value given in the argument.For example, `acc +7` would increase the accumulator by 7.
//!     The accumulator starts at `0`.
//!     After an `acc` instruction, the instruction immediately below it is executed next.
//! - `jmp` **jumps** to a new instruction relative to itself.
//!     The next instruction to execute is found using the argument as an **offset** from the `jmp` instruction; for example, `jmp +2` would skip the next instruction, `jmp +1` would continue to the instruction immediately below it, and `jmp -20` would cause the instruction 20 lines above to be executed next.
//! - `nop` stands for **No OPeration** - it does nothing.
//!     The instruction immediately below it is executed next.
//!
//! For example, consider the following program:
//!
//! ```text
//! nop +0
//! acc +1
//! jmp +4
//! acc +3
//! jmp -3
//! acc -99
//! acc +1
//! jmp -4
//! acc +6
//! ```
//!
//! These instructions are visited in this order:
//!
//! ```text
//! nop +0  | 1
//! acc +1  | 2, 8(!)
//! jmp +4  | 3
//! acc +3  | 6
//! jmp -3  | 7
//! acc -99 |
//! acc +1  | 4
//! jmp -4  | 5
//! acc +6  |
//! ```
//!
//! First, the `nop +0` does nothing.
//! Then, the accumulator is increased from 0 to 1 (`acc +1`) and `jmp +4` sets the next instruction to the other `acc +1` near the bottom.
//! After it increases the accumulator from 1 to 2, `jmp -4` executes, setting the next instruction to the only `acc +3`.
//! It sets the accumulator to 5, and `jmp -3` causes the program to continue back at the first `acc +1`.
//!
//! This is an **infinite loop**: with this sequence of jumps, the program will run forever.
//! The moment the program tries to run any instruction a second time, you know it will never terminate.
//!
//! Immediately **before** the program would run an instruction a second time, the value in the accumulator is **`5`**.
//!
//! Run your copy of the boot code.
//! Immediately before any instruction is executed a second time, **what value is in the accumulator?**
//!
//! # Part 2
//!
//! After some careful analysis, you believe that **exactly one instruction is corrupted**.
//!
//! Somewhere in the program, **either** a `jmp` is supposed to be a `nop`, or a `nop` is supposed to be a `jmp`.
//! (No `acc` instructions were harmed in the corruption of this boot code.)
//!
//! The program is supposed to terminate by **attempting to execute an instruction immediately after the last instruction in the file**.
//! By changing exactly one jmp or nop, you can repair the boot code and make it terminate correctly.
//!
//! For example, consider the same program from above:
//!
//! ```text
//! nop +0
//! acc +1
//! jmp +4
//! acc +3
//! jmp -3
//! acc -99
//! acc +1
//! jmp -4
//! acc +6
//! ```
//!
//! If you change the first instruction from `nop +0` to `jmp +0`, it would create a single-instruction infinite loop, never leaving that instruction.
//! If you change almost any of the `jmp` instructions, the program will still eventually find another `jmp` instruction and loop forever.
//!
//! However, if you change the second-to-last instruction (from `jmp -4` to `nop -4`), the program terminates!
//! The instructions are visited in this order:
//!
//! ```text
//! nop +0  | 1
//! acc +1  | 2
//! jmp +4  | 3
//! acc +3  |
//! jmp -3  |
//! acc -99 |
//! acc +1  | 4
//! nop -4  | 5
//! acc +6  | 6
//! ```
//!
//! After the last instruction (`acc +6`), the program terminates by attempting to run the instruction below the last instruction in the file.
//! With this change, after the program terminates, the accumulator contains the value **`8`** (`acc +1`, `acc +1`, `acc +6`).
//!
//! Fix the program so that it terminates normally by changing exactly one `jmp` (to `nop`) or `nop` (to `jmp`).
//! **What is the value of the accumulator after the program terminates?**

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
