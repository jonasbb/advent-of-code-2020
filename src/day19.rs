use crate::prelude::*;

struct Input {
    rules: Map<u32, String>,
    messages: Vec<String>,
}

#[aoc_generator(day19)]
fn input_generator(input: &str) -> Input {
    let mut input = input.split("\n\n");
    let rules: Map<u32, String> = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut i = line.split(": ");
            let id: u32 = i.next().unwrap().parse().unwrap();
            let pattern = i.next().unwrap().to_string();
            (id, pattern)
        })
        .collect();
    let messages = input
        .next()
        .unwrap()
        .lines()
        .map(ToString::to_string)
        .collect();

    Input { rules, messages }
}

fn convert_rule(
    id: u32,
    rules: &mut Map<u32, Set<String>>,
    tmp_rules: &Map<u32, String>,
) -> Set<String> {
    if let Some(res) = rules.get(&id) {
        return res.clone();
    }

    let pattern = &tmp_rules[&id];
    let pattern: BTreeSet<String> = if pattern == "\"a\"" {
        vec!["a".to_string()].into_iter().collect()
    } else if pattern == "\"b\"" {
        vec!["b".to_string()].into_iter().collect()
    } else {
        pattern
            .split(" | ")
            .flat_map(|p| {
                let mut patterns = Set::new();
                patterns.insert("".to_string());
                for other_pattern in p
                    .split(' ')
                    .map(|id| convert_rule(id.parse().unwrap(), rules, tmp_rules))
                {
                    patterns = merge_patterns(&patterns, &other_pattern);
                }
                patterns
            })
            .collect()
    };
    rules.insert(id, pattern.clone());
    pattern
}

fn merge_patterns(a: &Set<String>, b: &Set<String>) -> Set<String> {
    let mut res = Set::new();
    for string_a in a {
        for string_b in b {
            let mut new_str = String::with_capacity(string_a.len() + string_b.len());
            new_str.push_str(&string_a);
            new_str.push_str(&string_b);
            res.insert(new_str);
            // res.insert(format!("{}{}", string_a, string_b));
        }
    }
    res
}

#[aoc(day19, part1)]
fn part1(input: &Input) -> usize {
    let mut rules = Map::new();
    convert_rule(0, &mut rules, &input.rules);
    let rule = &input.rules[&0];
    input
        .messages
        .iter()
        .filter(|&msg| rule.contains(msg))
        .count()
}

#[aoc(day19, part2)]
fn part2(input: &Input) -> usize {
    let mut tmp_rules = input.rules.clone();
    tmp_rules.insert(8, "42 | 42 8".to_string());
    tmp_rules.insert(11, "42 31 | 42 11 31".to_string());
    let mut rules = Map::new();
    convert_rule(0, &mut rules, &input.rules);
    let rule = &input.rules[&0];
    input
        .messages
        .iter()
        .filter(|&msg| rule.contains(msg))
        .count()
}

#[test]
fn test_part1() {
    let values = input_generator(PUZZLE);
    assert_eq!(2, part1(&values));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day19.txt").trim());
    assert_eq!(122, part1(&values));
}

// #[test]
// fn test_part2() {
//     let values = input_generator(PUZZLE);
//     assert_eq!(46 + 1445 + 669060 + 23340, part2(&values));
// }

// #[test]
// fn test_part2_solution() {
//     let values = input_generator(include_str!("../input/2020/day19.txt").trim());
//     assert_eq!(88534268715686, part2(&values));
// }

#[cfg(test)]
static PUZZLE: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;
