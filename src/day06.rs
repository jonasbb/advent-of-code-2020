//! [Day 6: Custom Customs](https://adventofcode.com/2020/day/6)
//!
//! # Part 1
//!
//! As your flight approaches the regional airport where you'll switch to a much larger plane, customs declaration forms are distributed to the passengers.
//!
//! The form asks a series of 26 yes-or-no questions marked `a` through `z`.
//! All you need to do is identify the questions for which **anyone in your group** answers "yes".
//! Since your group is just you, this doesn't take very long.
//!
//! However, the person sitting next to you seems to be experiencing a language barrier and asks if you can help.
//! For each of the people in their group, you write down the questions for which they answer "yes", one per line.
//! For example:
//!
//! ```text
//! abcx
//! abcy
//! abcz
//! ```
//!
//! In this group, there are **`6`** questions to which anyone answered "yes": `a`, `b`, `c`, `x`, `y`, and `z`.
//! (Duplicate answers to the same question don't count extra; each question counts at most once.)
//!
//! Another group asks for your help, then another, and eventually you've collected answers from every group on the plane (your puzzle input). Each group's answers are separated by a blank line, and within each group, each person's answers are on a single line. For example:
//!
//! ```text
//! abc
//!
//! a
//! b
//! c
//!
//! ab
//! ac
//!
//! a
//! a
//! a
//! a
//!
//! b
//! ```
//!
//! This list represents answers from five groups:
//!
//! - The first group contains one person who answered "yes" to **`3`** questions: `a`, `b`, and `c`.
//! - The second group contains three people; combined, they answered "yes" to **`3`** questions: `a`, `b`, and `c`.
//! - The third group contains two people; combined, they answered "yes" to **`3`** questions: `a`, `b`, and `c`.
//! - The fourth group contains four people; combined, they answered "yes" to only **`1`** question, `a`.
//! - The last group contains one person who answered "yes" to only **`1`** question, `b`.
//!
//! In this example, the sum of these counts is `3 + 3 + 3 + 1 + 1` = **`11`**.
//!
//! For each group, count the number of questions to which anyone answered "yes".
//! **What is the sum of those counts?**
//!
//! # Part 2
//!
//! As you finish the last group's customs declaration, you notice that you misread one word in the instructions:
//!
//! You don't need to identify the questions to which **anyone** answered "yes"; you need to identify the questions to which **everyone** answered "yes"!
//!
//! Using the same example as above:
//!
//! ```text
//! abc
//!
//! a
//! b
//! c
//!
//! ab
//! ac
//!
//! a
//! a
//! a
//! a
//!
//! b
//! ```
//!
//! This list represents answers from five groups:
//!
//! - In the first group, everyone (all 1 person) answered "yes" to **`3`** questions: `a`, `b`, and `c`.
//! - In the second group, there is **no** question to which everyone answered "yes".
//! - In the third group, everyone answered yes to only **`1`** question, `a`. Since some people did not answer "yes" to `b` or `c`, they don't count.
//! - In the fourth group, everyone answered yes to only **`1`** question, `a`.
//! - In the fifth group, everyone (all 1 person) answered "yes" to **`1`** question, `b`.
//!
//! In this example, the sum of these counts is `3 + 0 + 1 + 1 + 1` = **`6`**.
//!
//! For each group, count the number of questions to which **everyone** answered "yes".
//! **What is the sum of those counts?**

use crate::prelude::*;

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<Vec<Set<char>>> {
    input
        .split("\n\n")
        .map(|group_answers| {
            group_answers
                .split('\n')
                .map(|answers| answers.chars().collect())
                .collect()
        })
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &[Vec<Set<char>>]) -> usize {
    input
        .iter()
        .map(|group| {
            group
                .iter()
                .fold(Set::<char>::new(), |mut accu, x| {
                    accu.extend(x);
                    accu
                })
                .len()
        })
        .sum()
}

#[aoc(day6, part2)]
fn part2(input: &[Vec<Set<char>>]) -> usize {
    let all_char: Set<char> = ('a'..='z').collect();
    input
        .iter()
        .map(|group| {
            group
                .iter()
                .fold(all_char.clone(), |accu, x| {
                    accu.intersection(x).copied().collect()
                })
                .len()
        })
        .sum()
}

#[test]
fn test_part1() {
    let values = input_generator(PUZZLE);
    assert_eq!(11, part1(&values));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day6.txt").trim());
    assert_eq!(6259, part1(&values));
}

#[test]
fn test_part2() {
    let values = input_generator(PUZZLE);
    assert_eq!(6, part2(&values));
}

#[test]
fn test_part2_solution() {
    let values = input_generator(include_str!("../input/2020/day6.txt").trim());
    assert_eq!(3178, part2(&values));
}

#[cfg(test)]
static PUZZLE: &str = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;
