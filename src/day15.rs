use crate::prelude::*;

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(|value| value.parse().unwrap())
        .collect()
}

#[aoc(day15, part1)]
fn part1(input: &[u32]) -> u32 {
    numbers(input).nth(2020 - 1).unwrap()
}

#[aoc(day15, part2)]
fn part2(input: &[u32]) -> u32 {
    numbers(&input).nth(30000000 - 1).unwrap()
}

fn numbers<'a>(input: &'a [u32]) -> impl Iterator<Item = u32> + 'a {
    let mut last_seen = Map::<u32, u32>::new();
    let mut last_value = input[0];
    let mut input = input.iter().cloned().fuse();
    let mut round = 0;
    std::iter::from_fn(move || {
        let value = if let Some(value) = input.next() {
            value
        } else {
            match last_seen.get(&last_value) {
                None => 0,
                Some(&last_round) => round - last_round,
            }
        };
        last_seen.insert(last_value, round);
        last_value = value;
        round += 1;
        Some(value)
    })
}

#[test]
fn test_part1() {
    let values = input_generator(PUZZLE);
    assert_eq!(
        vec![0, 3, 6, 0, 3, 3, 1, 0, 4, 0],
        numbers(&values).take(10).collect::<Vec<_>>()
    );
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day15.txt").trim());
    assert_eq!(1522, part1(&values));
}

#[test]
fn test_part2() {
    let values = input_generator(PUZZLE);
    assert_eq!(175594, numbers(&values).nth(30000000 - 1).unwrap());
}

#[test]
fn test_part2_1_3_2() {
    let values = input_generator("1,3,2");
    assert_eq!(2578, numbers(&values).nth(30000000 - 1).unwrap());
}

#[test]
fn test_part2_2_1_3() {
    let values = input_generator("2,1,3");
    assert_eq!(3544142, numbers(&values).nth(30000000 - 1).unwrap());
}

#[test]
fn test_part2_1_2_3() {
    let values = input_generator("1,2,3");
    assert_eq!(261214, numbers(&values).nth(30000000 - 1).unwrap());
}

#[test]
fn test_part2_2_3_1() {
    let values = input_generator("2,3,1");
    assert_eq!(6895259, numbers(&values).nth(30000000 - 1).unwrap());
}

#[test]
fn test_part2_3_2_1() {
    let values = input_generator("3,2,1");
    assert_eq!(18, numbers(&values).nth(30000000 - 1).unwrap());
}

#[test]
fn test_part2_3_1_2() {
    let values = input_generator("3,1,2");
    assert_eq!(362, numbers(&values).nth(30000000 - 1).unwrap());
}

#[test]
fn test_part2_solution() {
    let values = input_generator(include_str!("../input/2020/day15.txt").trim());
    assert_eq!(18234, part2(&values));
}

#[cfg(test)]
static PUZZLE: &str = r#"0,3,6"#;
