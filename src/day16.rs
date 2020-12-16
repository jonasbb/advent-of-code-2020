use crate::prelude::*;
use std::ops::RangeInclusive;

#[derive(Debug, Deserialize, PartialEq, Recap)]
#[recap(
    regex = r#"^(?P<name>[^:]+): (?P<range1_min>\d+)-(?P<range1_max>\d+) or (?P<range2_min>\d+)-(?P<range2_max>\d+)$"#
)]
struct Category {
    name: String,
    range1_min: u32,
    range1_max: u32,
    range2_min: u32,
    range2_max: u32,
}
#[derive(Debug)]
struct Input {
    categories: Map<String, (RangeInclusive<u32>, RangeInclusive<u32>)>,
    own_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

#[aoc_generator(day16)]
fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();

    let mut categories = Map::new();
    for line in &mut lines {
        if line == "" {
            break;
        }
        let category: Category = line.parse().unwrap();
        categories.insert(
            category.name,
            (
                category.range1_min..=category.range1_max,
                category.range2_min..=category.range2_max,
            ),
        );
    }

    // your ticket:
    lines.next();
    let own_ticket = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    // empty line
    lines.next();
    // nearby tickets:
    lines.next();

    let nearby_tickets = lines
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    Input {
        categories,
        own_ticket,
        nearby_tickets,
    }
}

#[aoc(day16, part1)]
fn part1(input: &Input) -> u32 {
    let mut ticket_scanning_error_rate = 0;
    for nearby_ticket in &input.nearby_tickets {
        'value: for value in nearby_ticket {
            for (range1, range2) in input.categories.values() {
                // Check if value is valid in any range
                if range1.contains(&value) || range2.contains(&value) {
                    continue 'value;
                }
            }
            ticket_scanning_error_rate += value;
        }
    }
    ticket_scanning_error_rate
}

#[aoc(day16, part2)]
fn part2(input: &Input) -> u64 {
    // Filter out invalid tickets
    let nearby_tickets = input
        .nearby_tickets
        .iter()
        .cloned()
        .filter(|nearby_ticket| {
            'value: for value in nearby_ticket {
                for (range1, range2) in input.categories.values() {
                    // Check if value is valid in any range
                    if range1.contains(&value) || range2.contains(&value) {
                        continue 'value;
                    }
                }
                return false;
            }
            true
        })
        .collect_vec();

    let mut assigned_columns = Set::new();
    let mut category_mapping = Map::new();
    for cat in &input.categories {
        category_mapping.insert(cat.0, (0..input.categories.len()).collect::<Set<_>>());
    }

    // Mark all incompatible columns
    for col in 0..input.categories.len() {
        // Already assigned this column
        if assigned_columns.get(&col).is_some() {
            continue;
        }

        'category: for (cat_name, (range1, range2)) in &input.categories {
            if category_mapping[cat_name].len() == 1 || !category_mapping[cat_name].contains(&col) {
                // Category only has 1 column left, skip
                continue;
            }

            // Try to find any mismatches between columns and categories
            for nearby_ticket in &nearby_tickets {
                let val = nearby_ticket[col];
                if !range1.contains(&val) && !range2.contains(&val) {
                    println!(
                        "Value {} in Col {} is not compatible with Cat {} ({:?}, {:?})",
                        val, col, cat_name, range1, range2
                    );
                    category_mapping.get_mut(cat_name).unwrap().remove(&col);
                    continue 'category;
                }
            }
        }
    }

    while assigned_columns.len() != input.categories.len() {
        println!("\nAssigned Column: {:?}", assigned_columns);
        println!("Possible Assignments:");
        for (cat_name, cols) in &category_mapping {
            println!("  {}: {:?}", cat_name, cols);
        }

        // Update set of assigned columns
        assigned_columns = category_mapping
            .iter()
            .filter(|(_, cm)| cm.len() == 1)
            .map(|(name, cm)| {
                let col = *cm.iter().next().unwrap();
                println!("{}: {}", name, col);
                col
            })
            .collect();
        // Remove all assigned columns from the category mapping
        category_mapping.values_mut().for_each(|cols| {
            // Need to ignore all categories which are already assigned, i.e., only have one col left.
            if cols.len() > 1 {
                for col in &assigned_columns {
                    cols.remove(col);
                }
            }
        });
    }

    println!("\nAssignments:");
    for (cat_name, cols) in &category_mapping {
        println!("  {}: {:?}", cat_name, cols);
    }

    category_mapping
        .iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(_, cols)| input.own_ticket[*cols.iter().next().unwrap()] as u64)
        .product()
}

#[test]
fn test_part1() {
    let values = input_generator(PUZZLE);
    assert_eq!(71, part1(&values));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day16.txt").trim());
    assert_eq!(21980, part1(&values));
}

#[test]
fn test_part2() {
    let values = input_generator(PUZZLE_PART2);
    assert_eq!(1, part2(&values));
}

#[test]
fn test_part2_solution() {
    let values = input_generator(include_str!("../input/2020/day16.txt").trim());
    assert_eq!(1439429522627, part2(&values));
}

#[cfg(test)]
static PUZZLE: &str = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#;

#[cfg(test)]
static PUZZLE_PART2: &str = r#"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"#;
