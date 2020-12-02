//! [Day 2: Password Philosophy](https://adventofcode.com/2020/day/2)
//!
//! # Part 1
//!
//! Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.
//!
//! The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day.
//! "Something's wrong with our computers; we can't log in!"
//! You ask if you can take a look.
//!
//! Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.
//!
//! To try to debug the problem, they have created a list (your puzzle input) of **passwords** (according to the corrupted database) and **the corporate policy when that password was set.**
//!
//! For example, suppose you have the following list:
//!
//! ```text
//! 1-3 a: abcde
//! 1-3 b: cdefg
//! 2-9 c: ccccccccc
//! ```
//!
//! Each line gives the password policy and then the password.
//! The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid.
//! For example, `1-3 a` means that the password must contain a at least `1` time and at most `3` times.
//!
//! In the above example, `2` passwords are valid.
//! The middle password, `cdefg`, is not; it contains no instances of `b`, but needs at least `1`.
//! The first and third passwords are valid: they contain one `a` or nine`c`, both within the limits of their respective policies.
//!
//! **How many passwords are valid** according to their policies?
//!
//! # Part 2
//!
//! While it appears you validated the passwords correctly, they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.
//!
//! The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from his old job at the sled rental place down the street!
//! The Official Toboggan Corporate Policy actually works a little differently.
//!
//! Each policy actually describes two **positions in the password**, where `1` means the first character, `2` means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!)
//! **Exactly one of these positions** must contain the given letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.
//!
//! Given the same example list from above:
//!
//! - `1-3 a: abcde` is **valid**: position `1` contains `a` and position `3` does not.
//! - `1-3 b: cdefg` is **invalid**: neither position `1` nor position `3` contains `b`.
//! - `2-9 c: ccccccccc` is **invalid**: both position `2` and position `9` contain `c`.
//!
//! **How many passwords are valid** according to the new interpretation of the policies?

use crate::prelude::*;

#[derive(Debug, Deserialize, PartialEq, Recap)]
#[recap(regex = r#"^(?P<count_min>\d+)-(?P<count_max>\d+) (?P<char>.): (?P<password>.+)$"#)]
struct PuzzleInput {
    count_min: usize,
    count_max: usize,
    char: char,
    password: String,
}

impl PuzzleInput {
    fn is_valid_part1(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.char).count();
        self.count_min <= count && count <= self.count_max
    }

    fn is_valid_part2(&self) -> bool {
        let first = self.password.chars().nth(self.count_min - 1) == Some(self.char);
        let second = self.password.chars().nth(self.count_max - 1) == Some(self.char);
        first ^ second
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<PuzzleInput> {
    input.split('\n').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
fn part1(input: &[PuzzleInput]) -> usize {
    input.iter().filter(|pi| pi.is_valid_part1()).count()
}

#[aoc(day2, part2)]
fn part2(input: &[PuzzleInput]) -> usize {
    input.iter().filter(|pi| pi.is_valid_part2()).count()
}

#[test]
fn test_part1() {
    let input = r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"#;
    let values = input_generator(input);
    assert_eq!(2, part1(&values));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day2.txt").trim());
    assert_eq!(591, part1(&values));
}

#[test]
fn test_part2() {
    let input = r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"#;
    let values = input_generator(input);
    assert_eq!(1, part2(&values));
}

#[test]
fn test_part2_solution() {
    let values = input_generator(include_str!("../input/2020/day2.txt").trim());
    assert_eq!(335, part2(&values));
}
