#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn vector(&self) -> (i32, i32) {
        match *self {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        }
    }

    fn left(&mut self) {
        *self = match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
    }

    fn right(&mut self) {
        *self = match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }
}

#[derive(Debug)]
enum Action {
    North,
    East,
    South,
    West,
    Left,
    Right,
    Forward,
}

#[aoc_generator(day12)]
fn input_generator(input: &str) -> Vec<(Action, i32)> {
    input
        .split('\n')
        .map(|line| {
            let action = &line[0..1];
            let value = &line[1..];
            let action = match action {
                "N" => Action::North,
                "E" => Action::East,
                "S" => Action::South,
                "W" => Action::West,
                "L" => Action::Left,
                "R" => Action::Right,
                "F" => Action::Forward,
                _ => panic!("Unknown action {}", action),
            };
            let value = value.parse().unwrap();
            (action, value)
        })
        .collect()
}

#[aoc(day12, part1)]
fn part1(input: &[(Action, i32)]) -> i32 {
    let mut direction = Direction::East;
    // (east, north)
    let mut position: (i32, i32) = (0, 0);

    for (action, value) in input {
        match *action {
            Action::North => position.1 += value,
            Action::East => position.0 += value,
            Action::South => position.1 -= value,
            Action::West => position.0 -= value,
            Action::Left => {
                for _ in 0..(value / 90) {
                    direction.left();
                }
            }
            Action::Right => {
                for _ in 0..(value / 90) {
                    direction.right()
                }
            }
            Action::Forward => {
                let dir = direction.vector();
                position.0 += dir.0 * value;
                position.1 += dir.1 * value;
            }
        }
    }
    position.0.abs() + position.1.abs()
}

#[aoc(day12, part2)]
fn part2(input: &[(Action, i32)]) -> i32 {
    // (east, north)
    let mut waypoint_offset = (10, 1);
    let mut position: (i32, i32) = (0, 0);

    for (action, value) in input {
        // println!("east: {} north {}", position.0, position.1);
        // println!("wp east: {} wp north {}", waypoint_offset.0, waypoint_offset.1);
        match *action {
            Action::North => waypoint_offset.1 += value,
            Action::East => waypoint_offset.0 += value,
            Action::South => waypoint_offset.1 -= value,
            Action::West => waypoint_offset.0 -= value,
            Action::Left => {
                for _ in 0..(value / 90) {
                    std::mem::swap(&mut waypoint_offset.0, &mut waypoint_offset.1);
                    waypoint_offset.0 = -waypoint_offset.0;
                }
            }
            Action::Right => {
                for _ in 0..(value / 90) {
                    std::mem::swap(&mut waypoint_offset.0, &mut waypoint_offset.1);
                    waypoint_offset.1 = -waypoint_offset.1;
                }
            }
            Action::Forward => {
                position.0 += waypoint_offset.0 * value;
                position.1 += waypoint_offset.1 * value;
            }
        }
    }
    position.0.abs() + position.1.abs()
}

#[test]
fn test_part1() {
    let values = input_generator(PUZZLE);
    assert_eq!(25, part1(&values));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day12.txt").trim());
    assert_eq!(1565, part1(&values));
}

#[test]
fn test_part2() {
    let values = input_generator(PUZZLE);
    assert_eq!(286, part2(&values));
}

#[test]
fn test_part2_solution() {
    let values = input_generator(include_str!("../input/2020/day12.txt").trim());
    assert_eq!(78883, part2(&values));
}

#[cfg(test)]
static PUZZLE: &str = r#"F10
N3
F7
R90
F11"#;
