use crate::prelude::*;

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<u32> {
    let mut input: Vec<_> = input
        .split('\n')
        .map(|line| line.parse().unwrap())
        .collect();
    input.push(0);
    input.sort_unstable();
    input.push(input[input.len() - 1] + 3);
    input
}

#[aoc(day10, part1)]
fn part1(input: &[u32]) -> u32 {
    let (a, b) = joltage_differences(input);
    a * b
}

fn joltage_differences(input: &[u32]) -> (u32, u32) {
    input
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .fold((0, 0), |(diff_1, diff_3), diff| {
            if diff == 1 {
                (diff_1 + 1, diff_3)
            } else {
                (diff_1, diff_3 + 1)
            }
        })
}

#[aoc(day10, part2)]
fn part2(input: &[u32]) -> u64 {
    charger_variations(input)
}

fn charger_variations(input: &[u32]) -> u64 {
    fn charger_variations_inner(mut input: &[u32], cache: &mut Map<usize, u64>) -> u64 {
        let orig_len = input.len();
        if let Some(res) = cache.get(&orig_len) {
            return *res;
        }

        let first = input[0];
        input = &input[1..];

        let mut variations = 0;
        while !input.is_empty() && (input[0] - first) <= 3 {
            variations += charger_variations_inner(input, cache);
            input = &input[1..];
        }
        cache.insert(orig_len, variations);
        variations
    }
    let mut cache: Map<usize, u64> = vec![(1, 1)].into_iter().collect();
    charger_variations_inner(input, &mut cache)
}

#[test]
fn test_part1_small() {
    let values = input_generator(PUZZLE_SMALL);
    assert_eq!((7, 5), joltage_differences(&values));
}

#[test]
fn test_part1() {
    let values = input_generator(PUZZLE);
    assert_eq!((22, 10), joltage_differences(&values));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day10.txt").trim());
    assert_eq!(1755, part1(&values));
}

#[test]
fn test_part2_small() {
    let values = input_generator(PUZZLE_SMALL);
    assert_eq!(8, charger_variations(&values));
}

#[test]
fn test_part2() {
    let values = input_generator(PUZZLE);
    assert_eq!(19208, charger_variations(&values));
}

#[test]
fn test_part2_solution() {
    let values = input_generator(include_str!("../input/2020/day10.txt").trim());
    assert_eq!(4049565169664, part2(&values));
}

#[cfg(test)]
static PUZZLE_SMALL: &str = r#"16
10
15
5
1
11
7
19
6
12
4"#;

#[cfg(test)]
static PUZZLE: &str = r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;
