//! [Day 1: Report Repair](https://adventofcode.com/2020/day/1)
//!
//! # Part 1
//!
//! After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island.
//! Surely, Christmas will go on without you.
//!
//! The tropical island has its own currency and is entirely cash-only.
//! The gold coins used there have a little picture of a starfish; the locals just call them stars.
//! None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.
//!
//! To save your vacation, you need to get all fifty stars by December 25th.
//!
//! Collect stars by solving puzzles.
//! Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star.
//! Good luck!
//!
//! Before you leave, the Elves in accounting just need you to fix your **expense report** (your puzzle input); apparently, something isn't quite adding up.
//!
//! Specifically, they need you to **find the two entries that sum to `2020`** and then multiply those two numbers together.
//!
//! For example, suppose your expense report contained the following:
//!
//! ```text
//! 1721
//! 979
//! 366
//! 299
//! 675
//! 1456
//! ```
//!
//! In this list, the two entries that sum to `2020` are `1721` and `299`.
//! Multiplying them together produces `1721 * 299 = 514579`, so the correct answer is `514579`.
//!
//! Of course, your expense report is much larger. **Find the two entries that sum to 2020; what do you get if you multiply them together?
//!
//! # Part 2
//!
//! The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from a past vacation.
//! They offer you a second one if you can find **three** numbers in your expense report that meet the same criteria.
//!
//! Using the above example again, the three entries that sum to `2020` are `979`, `366`, and `675`.
//! Multiplying them together produces the answer, `241861950`.
//!
//! In your expense report, **what is the product of the three entries that sum to 2020**?

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<u32> {
    input.split('\n').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> u32 {
    for i in input {
        for j in input {
            if i + j == 2020 {
                return i * j;
            }
        }
    }

    panic!("Could not find entries which match the requirements")
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> u32 {
    for i in input {
        for j in input {
            for k in input {
                if i + j + k == 2020 {
                    return i * j * k;
                }
            }
        }
    }

    panic!("Could not find entries which match the requirements")
}

#[test]
fn test_part1() {
    let input = r#"1721
979
366
299
675
1456"#;
    let values = input_generator(input);
    assert_eq!(514579, part1(&values));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day1.txt").trim());
    assert_eq!(545379, part1(&values));
}

#[test]
fn test_part2() {
    let input = r#"1721
979
366
299
675
1456"#;
    let values = input_generator(input);
    assert_eq!(241861950, part2(&values));
}

#[test]
fn test_part2_solution() {
    let values = input_generator(include_str!("../input/2020/day1.txt").trim());
    assert_eq!(257778836, part2(&values));
}
