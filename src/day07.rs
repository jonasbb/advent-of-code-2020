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
