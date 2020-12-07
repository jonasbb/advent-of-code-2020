use crate::prelude::*;

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Map<String, Vec<String>> {
    // light red bags contain 1 bright white bag, 2 muted yellow bags.

    let mut res: Map<String, Vec<String>> = Map::new();
    input.split('\n').for_each(|line| {
        let mut bags = line.split(" bag").filter_map(|part| {
            let color: String = part.rsplit(' ').take(2).collect();
            match &*color {
                "s." | "." | "otherno" => None,
                _ => Some(color),
            }
        });
        let outer = bags.next().unwrap();
        res.entry(outer).or_default().extend(bags);
    });
    res
}

#[aoc(day7, part1)]
fn part1(input: &Map<String, Vec<String>>) -> usize {
    // Invert map, i.e., if we have A->B and A->C we now get
    // C->A and B->A
    let mut inverted_map: Map<&str, Vec<&str>> = Map::new();
    input.iter().for_each(|(outer, inners)| {
        for inner in inners {
            inverted_map.entry(inner).or_default().push(outer);
        }
    });

    let mut unexplored = vec!["goldshiny"];
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

// #[aoc(day7, part2)]
// fn part2(input: &[Vec<Set<char>>]) -> usize {
//     let all_char: Set<char> = ('a'..='z').collect();
//     input
//         .iter()
//         .map(|group| {
//             group
//                 .iter()
//                 .fold(all_char.clone(), |accu, x| {
//                     accu.intersection(x).copied().collect()
//                 })
//                 .len()
//         })
//         .sum()
// }

#[test]
fn test_part1() {
    let values = input_generator(PUZZLE);
    assert_eq!(4, part1(&values));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day7.txt").trim());
    assert_eq!(4, part1(&values));
}

// #[test]
// fn test_part2() {
//     let values = input_generator(PUZZLE);
//     assert_eq!(6, part2(&values));
// }

// #[test]
// fn test_part2_solution() {
//     let values = input_generator(include_str!("../input/2020/day7.txt").trim());
//     assert_eq!(3178, part2(&values));
// }

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
