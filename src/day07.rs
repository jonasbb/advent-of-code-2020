//! [Day 7: Handy Haversacks](https://adventofcode.com/2020/day/7)
//!
//! # Part 1
//!
//! You land at the regional airport in time for your next flight.
//! In fact, it looks like you'll even have time to grab some food: all flights are currently delayed due to **issues in luggage processing**.
//!
//! Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and their contents; bags must be color-coded and must contain specific quantities of other color-coded bags.
//! Apparently, nobody responsible for these regulations considered how long they would take to enforce!
//!
//! For example, consider the following rules:
//!
//! ```text
//! light red bags contain 1 bright white bag, 2 muted yellow bags.
//! dark orange bags contain 3 bright white bags, 4 muted yellow bags.
//! bright white bags contain 1 shiny gold bag.
//! muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
//! shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
//! dark olive bags contain 3 faded blue bags, 4 dotted black bags.
//! vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
//! faded blue bags contain no other bags.
//! dotted black bags contain no other bags.
//! ```
//!
//! These rules specify the required contents for 9 bag types.
//! In this example, every `faded blue` bag is empty, every `vibrant plum` bag contains 11 bags (5 `faded blue` and 6 `dotted black`), and so on.
//!
//! You have a **`shiny gold`** bag.
//! If you wanted to carry it in at least one other bag, how many different bag colors would be valid for the outermost bag?
//! (In other words: how many colors can, eventually, contain at least one `shiny gold` bag?)
//!
//! In the above rules, the following options would be available to you:
//!
//! - A `bright white` bag, which can hold your `shiny gold` bag directly.
//! - A `muted yellow` bag, which can hold your `shiny gold` bag directly, plus some other bags.
//! - A `dark orange` bag, which can hold `bright white` and `muted yellow` bags, either of which could then hold your `shiny gold` bag.
//! - A `light red` bag, which can hold `bright white` and `muted yellow` bags, either of which could then hold your `shiny gold` bag.
//!
//! So, in this example, the number of bag colors that can eventually contain at least one shiny gold bag is **`4`**.
//!
//! **How many bag colors can eventually contain at least one shiny gold bag?**
//! (The list of rules is quite long; make sure you get all of it.)
//!
//! # Part 2
//!
//! It's getting pretty expensive to fly these days - not because of ticket prices, but because of the ridiculous number of bags you need to buy!
//!
//! Consider again your `shiny gold` bag and the rules from the above example:
//!
//! - `faded blue` bags contain `0` other bags.
//! - `dotted black` bags contain `0` other bags.
//! - `vibrant plum` bags contain `11` other bags: 5 `faded blue` bags and 6 `dotted black` bags.
//! - `dark olive` bags contain `7` other bags: 3 `faded blue` bags and 4 `dotted black` bags.
//!
//! So, a single `shiny gold` bag must contain 1 `dark olive` bag (and the 7 bags within it) plus 2 `vibrant plum` bags (and the 11 bags within **each** of those): `1 + 1*7 + 2 + 2*11` = **`32`** bags!
//!
//! Of course, the actual rules have a small chance of going several levels deeper than this example; be sure to count all of the bags, even if the nesting becomes topologically impractical!
//!
//! Here's another example:
//!
//! ```text
//! shiny gold bags contain 2 dark red bags.
//! dark red bags contain 2 dark orange bags.
//! dark orange bags contain 2 dark yellow bags.
//! dark yellow bags contain 2 dark green bags.
//! dark green bags contain 2 dark blue bags.
//! dark blue bags contain 2 dark violet bags.
//! dark violet bags contain no other bags.
//! ```
//!
//! In this example, a `single shiny` gold bag must contain **`126`** other bags.
//!
//! **How many individual bags are required inside your single shiny gold bag?**

use crate::prelude::*;

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Map<String, Vec<(usize, String)>> {
    let re_bags = Regex::new(r"(?P<count>\d+) (?P<color>\w+ \w+) bag").unwrap();

    let mut res: Map<String, Vec<(usize, String)>> = Map::new();
    input.split('\n').for_each(|line| {
        let outer = line.split(" bag").next().unwrap();
        let entry = res.entry(outer.to_string()).or_default();
        for capture in re_bags.captures_iter(line) {
            let count = capture["count"].parse().unwrap();
            let inner = capture["color"].to_string();
            entry.push((count, inner))
        }
    });
    res
}

#[aoc(day7, part1)]
fn part1(input: &Map<String, Vec<(usize, String)>>) -> usize {
    // Invert map, i.e., if we have A->B and A->C we now get
    // C->A and B->A
    let mut inverted_map: Map<&str, Vec<&str>> = Map::new();
    input.iter().for_each(|(outer, inners)| {
        for (_, inner) in inners {
            inverted_map.entry(inner).or_default().push(outer);
        }
    });

    let mut unexplored = vec!["shiny gold"];
    let mut can_contain: Set<&str> = Set::new();
    while let Some(color) = unexplored.pop() {
        if let Some(colors) = inverted_map.get(color) {
            can_contain.extend(colors);
            unexplored.extend(colors);
        }
    }
    // dbg!(&can_contain);
    can_contain.len()
}

#[aoc(day7, part2, naive)]
fn part2_naive(input: &Map<String, Vec<(usize, String)>>) -> usize {
    fn get_color_contains_bags<'a>(
        color: &'a str,
        input: &'a Map<String, Vec<(usize, String)>>,
    ) -> usize {
        let inner_bag_count = input[color]
            .iter()
            .map(|(count, color)| (get_color_contains_bags(color, input) + 1) * count)
            .sum();
        inner_bag_count
    }
    get_color_contains_bags("shiny gold", input)
}

#[aoc(day7, part2, memoization)]
fn part2_memoization(input: &Map<String, Vec<(usize, String)>>) -> usize {
    fn get_color_contains_bags<'a>(
        color: &'a str,
        input: &'a Map<String, Vec<(usize, String)>>,
        cache: &mut Map<&'a str, usize>,
    ) -> usize {
        if let Some(&res) = cache.get(color) {
            return res;
        }
        // Need to compute the value
        let inner_bag_count = input[color]
            .iter()
            .map(|(count, color)| (get_color_contains_bags(color, input, cache) + 1) * count)
            .sum();
        cache.insert(color, inner_bag_count);
        inner_bag_count
    }

    let mut cache: Map<&str, usize> = Map::new();
    get_color_contains_bags("shiny gold", input, &mut cache)
}

#[test]
fn test_part1() {
    let values = input_generator(PUZZLE);
    assert_eq!(4, part1(&values));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day7.txt").trim());
    assert_eq!(300, part1(&values));
}

#[test]
fn test_part2() {
    let values = input_generator(PUZZLE);
    assert_eq!(32, part2_memoization(&values));
}

#[test]
fn test_part2_example2() {
    let values = input_generator(PUZZLE2);
    assert_eq!(126, part2_memoization(&values));
}

#[test]
fn test_part2_solution_naive() {
    let values = input_generator(include_str!("../input/2020/day7.txt").trim());
    assert_eq!(8030, part2_naive(&values));
}

#[test]
fn test_part2_solution_memoization() {
    let values = input_generator(include_str!("../input/2020/day7.txt").trim());
    assert_eq!(8030, part2_memoization(&values));
}

#[cfg(test)]
static PUZZLE: &str = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;

#[cfg(test)]
static PUZZLE2: &str = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#;
