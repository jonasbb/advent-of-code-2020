#[aoc_generator(day13)]
fn input_generator(input: &str) -> (u32, Vec<Option<u32>>) {
    let mut iter = input.split('\n');
    let earliest = iter.next().unwrap().parse().unwrap();
    let ids = iter
        .next()
        .unwrap()
        .split(',')
        .map(|id| {
            if id == "x" {
                None
            } else {
                Some(id.parse().unwrap())
            }
        })
        .collect();
    (earliest, ids)
}

#[aoc(day13, part1)]
fn part1((earliest, ids): &(u32, Vec<Option<u32>>)) -> u32 {
    let mut earliest_time = u32::max_value();
    let mut earliest_id = 0;

    for &id in ids {
        if let Some(id) = id {
            let time = ((earliest / id) + 1) * id;
            if time < earliest_time {
                earliest_time = time;
                earliest_id = id;
            }
        }
    }

    earliest_id * (earliest_time - earliest)
}

#[aoc(day13, part2)]
fn part2((_, ids): &(u32, Vec<Option<u32>>)) -> u64 {
    let mut time: u64 = 1000;
    // The increment encapsulates all the previously seen ids
    // If the increment is a multiple of the ID, then adding the increment will never destroy the time alignment of those IDs.
    let mut increment = 1;

    for (offset, id) in ids.iter().enumerate() {
        if let Some(id) = *id {
            while (time + (offset as u64)) % u64::from(id) != 0 {
                time += increment;
            }
            increment *= u64::from(id);
        }
    }
    time
}

#[test]
fn test_part1() {
    let values = input_generator(PUZZLE);
    assert_eq!(295, part1(&values));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day13.txt").trim());
    assert_eq!(119, part1(&values));
}

#[test]
fn test_part2() {
    let values = input_generator(PUZZLE);
    assert_eq!(1068781, part2(&values));
}

#[test]
fn test_part2_solution() {
    let values = input_generator(include_str!("../input/2020/day13.txt").trim());
    assert_eq!(1106724616194525, part2(&values));
}

#[cfg(test)]
static PUZZLE: &str = r#"939
7,13,x,x,59,x,31,19"#;
