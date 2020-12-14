use crate::prelude::*;
use std::convert::TryInto;

enum Instruction {
    Mask([char; 36]),
    Store { addr: usize, value: usize },
}

#[aoc_generator(day14)]
fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .split('\n')
        .map(|line| {
            if line.starts_with("mask") {
                Instruction::Mask(
                    line.split(" = ")
                        .nth(1)
                        .unwrap()
                        .chars()
                        .collect_vec()
                        .try_into()
                        .unwrap(),
                )
            } else {
                let mut iter = line.split("] = ");
                let addr = (&iter.next().unwrap()[4..]).parse().unwrap();
                let value = iter.next().unwrap().parse().unwrap();
                Instruction::Store { addr, value }
            }
        })
        .collect()
}

#[aoc(day14, part1)]
fn part1(input: &[Instruction]) -> usize {
    let mut mask = ['X'; 36];
    let mut memory = Map::<usize, usize>::new();
    for inst in input {
        match inst {
            Instruction::Mask(m) => {
                mask = *m;
            }
            Instruction::Store { addr, value } => {
                let value_bin = format!("{:0>36b}", value);
                let value = usize::from_str_radix(
                    &value_bin
                        .chars()
                        .zip(mask.iter().cloned())
                        .map(|(v, m)| match m {
                            'X' => v,
                            '1' => '1',
                            '0' => '0',
                            _ => panic!("Unknown mask char: {}", m),
                        })
                        .collect::<String>(),
                    2,
                )
                .unwrap();
                memory.insert(*addr, value);
            }
        }
    }
    memory.values().sum()
}

#[aoc(day14, part2)]
fn part2(input: &[Instruction]) -> usize {
    let mut mask = ['X'; 36];
    let mut memory = Map::<usize, usize>::new();
    for inst in input {
        match inst {
            Instruction::Mask(m) => {
                mask = *m;
            }
            Instruction::Store { addr, value } => {
                // Count floating bits
                let floating_bits = mask.iter().filter(|&&c| c == 'X').count();
                for num in 0..2usize.pow(floating_bits as u32) {
                    let floating_addr_bits = format!("{:0>width$b}", num, width = floating_bits);
                    let mut floating_addr_bits = floating_addr_bits.chars();
                    let addr = format!("{:0>36b}", addr);
                    let addr = usize::from_str_radix(
                        &addr
                            .chars()
                            .zip(mask.iter().cloned())
                            .map(|(a, m)| match m {
                                'X' => floating_addr_bits.next().unwrap(),
                                '1' => '1',
                                '0' => a,
                                _ => panic!("Unknown mask char: {}", m),
                            })
                            .collect::<String>(),
                        2,
                    )
                    .unwrap();
                    memory.insert(addr, *value);
                }
            }
        }
    }
    memory.values().sum()
}

#[test]
fn test_part1() {
    let values = input_generator(PUZZLE);
    assert_eq!(165, part1(&values));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day14.txt").trim());
    assert_eq!(7817357407588, part1(&values));
}

#[test]
fn test_part2() {
    let values = input_generator(PUZZLE_PART2);
    assert_eq!(208, part2(&values));
}

#[test]
fn test_part2_solution() {
    let values = input_generator(include_str!("../input/2020/day14.txt").trim());
    assert_eq!(4335927555692, part2(&values));
}

#[cfg(test)]
static PUZZLE: &str = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"#;

#[cfg(test)]
static PUZZLE_PART2: &str = r#"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"#;
