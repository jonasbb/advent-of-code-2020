//! [Day 5: Binary Boarding](https://adventofcode.com/2020/day/5)
//!
//! # Part 1
//!
//! You board your plane only to discover a new problem: you dropped your boarding pass!
//! You aren't sure which seat is yours, and all of the flight attendants are busy with the flood of people that suddenly made it through passport control.
//!
//! You write a quick program to use your phone's camera to scan all of the nearby boarding passes (your puzzle input); perhaps you can find your seat through process of elimination.
//!
//! Instead of [zones or groups](https://www.youtube.com/watch?v=oAHbLRjF0vo), this airline uses **binary space partitioning** to seat people.
//! A seat might be specified like `FBFBBFFRLR`, where `F` means "front", `B` means "back", `L` means "left", and `R` means "right".
//!
//! The first 7 characters will either be `F` or `B`; these specify exactly one of the **128 rows** on the plane (numbered `0` through `127`).
//! Each letter tells you which half of a region the given seat is in.
//! Start with the whole list of rows; the first letter indicates whether the seat is in the **front** (`0` through `63`) or the **back** (64 through 127).
//! The next letter indicates which half of that region the seat is in, and so on until you're left with exactly one row.
//!
//! For example, consider just the first seven characters of `FBFBBFFRLR`:
//!
//! - Start by considering the whole range, rows `0` through `127`.
//! - `F` means to take the **lower half**, keeping rows `0` through `63`.
//! - `B` means to take the **upper half**, keeping rows `32` through `63`.
//! - `F` means to take the **lower half**, keeping rows `32` through `47`.
//! - `B` means to take the **upper half**, keeping rows `40` through `47`.
//! - `B` keeps rows `44` through `47`.
//! - `F` keeps rows `44` through `45`.
//! - The final `F` keeps the lower of the two, **row `44`.**
//!
//! The last three characters will be either `L` or `R`; these specify exactly one of the **8 columns** of seats on the plane (numbered `0` through `7`).
//! The same process as above proceeds again, this time with only three steps.
//! `L` means to keep the **lower half**, while `R` means to keep the **upper half**.
//!
//! For example, consider just the last 3 characters of `FBFBBFFRLR`:
//!
//! - Start by considering the whole range, columns `0` through `7`.
//! - `R` means to take the **upper half**, keeping columns `4` through `7`.
//! - `L` means to take the **lower half**, keeping columns `4` through `5`.
//! - The final `R` keeps the upper of the two, column `5`.
//!
//! So, decoding `FBFBBFFRLR` reveals that it is the seat at **`row 44, column 5`.**
//!
//! Every seat also has a unique **seat ID**: multiply the row by 8, then add the column.
//! In this example, the seat has ID `44 * 8 + 5 = 357`.
//!
//! Here are some other boarding passes:
//!
//! - `BFFFBBFRRR`: row `70`, column `7`, seat ID `567`.
//! - `FFFBBBFRRR`: row `14`, column `7`, seat ID `119`.
//! - `BBFFBBFRLL`: row `102`, column `4`, seat ID `820`.
//!
//! As a sanity check, look through your list of boarding passes.
//! **What is the highest seat ID on a boarding pass?**
//!
//! # Part 2
//!
//! **Ding!**
//! The "fasten seat belt" signs have turned on.
//! Time to find your seat.
//!
//! It's a completely full flight, so your seat should be the only missing boarding pass in your list.
//! However, there's a catch: some of the seats at the very front and back of the plane don't exist on this aircraft, so they'll be missing from your list as well.
//!
//! Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours will be in your list.
//!
//! **What is the ID of your seat?**

use crate::prelude::*;

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<(u32, u32)> {
    input.split('\n').map(parse_seating).collect()
}

fn parse_seating(s: &str) -> (u32, u32) {
    let s = s
        .replace("F", "0")
        .replace("B", "1")
        .replace("L", "0")
        .replace("R", "1");
    let row = u32::from_str_radix(&s[0..7], 2).unwrap();
    let col = u32::from_str_radix(&s[7..10], 2).unwrap();
    (row, col)
}

#[aoc(day5, part1)]
fn part1(input: &[(u32, u32)]) -> u32 {
    input
        .iter()
        .map(|(row, col)| *row * 8 + *col)
        .max()
        .unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &[(u32, u32)]) -> u32 {
    let set: Set<(_, _)> = input.iter().copied().collect();
    for row in 12..123 {
        for col in 0..8 {
            if set.get(&(row, col)).is_none() {
                // println!("{} - {}" ,row, col)
                return row * 8 + col;
            }
        }
    }
    0
}

#[test]
fn test_parse_seating() {
    assert_eq!((70, 7), parse_seating("BFFFBBFRRR"));
    assert_eq!((14, 7), parse_seating("FFFBBBFRRR"));
    assert_eq!((102, 4), parse_seating("BBFFBBFRLL"));
}

#[test]
fn test_part1() {
    let values = input_generator(PUZZLE);
    assert_eq!(820, part1(&values));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day5.txt").trim());
    assert_eq!(989, part1(&values));
}

#[test]
fn test_part2_solution() {
    let values = input_generator(include_str!("../input/2020/day5.txt").trim());
    assert_eq!(548, part2(&values));
}

#[cfg(test)]
static PUZZLE: &str = r#"BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL"#;
