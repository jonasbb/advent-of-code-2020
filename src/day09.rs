use crate::prelude::*;
use std::{cmp::Ordering, collections::VecDeque};

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<u64> {
    input
        .split('\n')
        .map(|line| line.parse().unwrap())
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[u64]) -> u64 {
    find_number(input, 25)
}

fn find_number(input: &[u64], preamble_size: usize) -> u64 {
    let mut iter = input.iter().cloned();
    let mut last_x: VecDeque<_> = (&mut iter).take(preamble_size).collect();

    for value in iter {
        // Check if value can be computed from the last 25
        let has_pair = last_x
            .iter()
            .enumerate()
            .flat_map(|(idx, &i)| last_x.iter().skip(idx + 1).map(move |&j| i + j))
            .any(|x| x == value);
        if !has_pair {
            return value;
        }

        last_x.push_back(value);
        if last_x.len() > preamble_size {
            last_x.pop_front();
        }
    }
    panic!("Should have found a number which cannot be computed.")
}

#[aoc(day9, part2)]
fn part2(input: &[u64]) -> u64 {
    find_range(input, 69316178)
}

fn find_range(input: &[u64], target: u64) -> u64 {
    'outer: for start_idx in 0..input.len() {
        for length in 2.. {
            match input[start_idx..][..length]
                .iter()
                .copied()
                .sum::<u64>()
                .cmp(&target)
            {
                // Do nothing, we need to add more numbers
                Ordering::Less => {}
                // Found our target
                Ordering::Equal => {
                    let (min, max) = input[start_idx..][..length]
                        .iter()
                        .copied()
                        .minmax()
                        .into_option()
                        .unwrap();
                    return min + max;
                }
                // Went too far, continue outer loop
                Ordering::Greater => continue 'outer,
            }
        }
    }
    panic!("Should have found a number which cannot be computed.")
}

#[test]
fn test_part1() {
    let values = input_generator(PUZZLE);
    assert_eq!(127, find_number(&values, 5));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day9.txt").trim());
    assert_eq!(69316178, part1(&values));
}

#[test]
fn test_part2() {
    let values = input_generator(PUZZLE);
    assert_eq!(62, find_range(&values, 127));
}

#[test]
fn test_part2_solution() {
    let values = input_generator(include_str!("../input/2020/day9.txt").trim());
    assert_eq!(9351526, part2(&values));
}

#[cfg(test)]
static PUZZLE: &str = r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#;
