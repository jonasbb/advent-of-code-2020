#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum State {
    Floor,
    Empty,
    Occupied,
}

#[aoc_generator(day11)]
fn input_generator(input: &str) -> Vec<Vec<State>> {
    input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => State::Floor,
                    'L' => State::Empty,
                    '#' => State::Occupied,
                    _ => panic!("Unknown state symbol {}", c),
                })
                .collect()
        })
        .collect()
}

#[allow(dead_code)]
fn print_layout(layout: &Vec<Vec<State>>) {
    for line in layout {
        for place in line {
            eprint!(
                "{}",
                match place {
                    State::Floor => '.',
                    State::Empty => 'L',
                    State::Occupied => '#',
                }
            )
        }
        eprintln!();
    }
    eprintln!();
}

#[aoc(day11, part1)]
fn part1(input: &[Vec<State>]) -> usize {
    let size_x = input.len();
    let size_y = input[0].len();

    let mut curr = input.to_vec();
    let mut next = input.to_vec();

    loop {
        // let is_empty_seat = |x: usize, y: usize| if curr[x][y] == State::Empty { 1 } else { 0 };
        let is_occupied_seat =
            |x: usize, y: usize| if curr[x][y] == State::Occupied { 1 } else { 0 };
        for x in 0..size_x {
            for y in 0..size_y {
                // Floors never change
                let field = curr[x][y];
                if field == State::Floor {
                    continue;
                }

                let mut occupied_count = 0;

                // Check left-above
                if x > 0 && y > 0 {
                    occupied_count += is_occupied_seat(x - 1, y - 1)
                }
                // Check above
                if x > 0 {
                    occupied_count += is_occupied_seat(x - 1, y)
                }
                // Check right-above
                if x > 0 && y < size_y - 1 {
                    occupied_count += is_occupied_seat(x - 1, y + 1)
                }
                // Check left
                if y > 0 {
                    occupied_count += is_occupied_seat(x, y - 1)
                }
                // Check right
                if y < size_y - 1 {
                    occupied_count += is_occupied_seat(x, y + 1)
                }
                // Check left-bottom
                if x < size_x - 1 && y > 0 {
                    occupied_count += is_occupied_seat(x + 1, y - 1)
                }
                // Check bottom
                if x < size_x - 1 {
                    occupied_count += is_occupied_seat(x + 1, y)
                }
                // Check right-bottom
                if x < size_x - 1 && y < size_y - 1 {
                    occupied_count += is_occupied_seat(x + 1, y + 1)
                }

                if field == State::Empty && occupied_count == 0 {
                    next[x][y] = State::Occupied
                } else if field == State::Occupied && occupied_count >= 4 {
                    next[x][y] = State::Empty
                } else {
                    next[x][y] = field
                }
            }
        }

        if next == curr {
            break;
        }
        std::mem::swap(&mut next, &mut curr);
    }

    curr.into_iter()
        .flatten()
        .filter(|&s| s == State::Occupied)
        .count()
}

#[aoc(day11, part2)]
fn part2(input: &[Vec<State>]) -> usize {
    let size_x = input.len();
    let size_y = input[0].len();

    let mut curr = input.to_vec();
    let mut next = input.to_vec();

    loop {
        // print_layout(&curr);
        for x in 0..size_x {
            for y in 0..size_y {
                // Floors never change
                let field = curr[x][y];
                if field == State::Floor {
                    continue;
                }

                let mut occupied_count = 0;

                // Check left-above
                {
                    let mut x = x;
                    let mut y = y;
                    while x > 0 && y > 0 {
                        x -= 1;
                        y -= 1;
                        match curr[x][y] {
                            State::Occupied => occupied_count += 1,
                            State::Empty => {}
                            State::Floor => continue,
                        }
                        break;
                    }
                }
                // Check above
                {
                    let mut x = x;
                    while x > 0 {
                        x -= 1;
                        match curr[x][y] {
                            State::Occupied => occupied_count += 1,
                            State::Empty => {}
                            State::Floor => continue,
                        }
                        break;
                    }
                }
                // Check right-above
                {
                    let mut x = x;
                    let mut y = y;
                    while x > 0 && y < size_y - 1 {
                        x -= 1;
                        y += 1;
                        match curr[x][y] {
                            State::Occupied => occupied_count += 1,
                            State::Empty => {}
                            State::Floor => continue,
                        }
                        break;
                    }
                }
                // Check left
                {
                    let mut y = y;
                    while y > 0 {
                        y -= 1;
                        match curr[x][y] {
                            State::Occupied => occupied_count += 1,
                            State::Empty => {}
                            State::Floor => continue,
                        }
                        break;
                    }
                }
                // Check right
                {
                    let mut y = y;
                    while y < size_y - 1 {
                        y += 1;
                        match curr[x][y] {
                            State::Occupied => occupied_count += 1,
                            State::Empty => {}
                            State::Floor => continue,
                        }
                        break;
                    }
                }
                // Check left-bottom
                {
                    let mut x = x;
                    let mut y = y;
                    while x < size_x - 1 && y > 0 {
                        x += 1;
                        y -= 1;
                        match curr[x][y] {
                            State::Occupied => occupied_count += 1,
                            State::Empty => {}
                            State::Floor => continue,
                        }
                        break;
                    }
                }
                // Check bottom
                {
                    let mut x = x;
                    while x < size_x - 1 {
                        x += 1;
                        match curr[x][y] {
                            State::Occupied => occupied_count += 1,
                            State::Empty => {}
                            State::Floor => continue,
                        }
                        break;
                    }
                }
                // Check right-bottom
                {
                    let mut x = x;
                    let mut y = y;
                    while x < size_x - 1 && y < size_y - 1 {
                        x += 1;
                        y += 1;
                        match curr[x][y] {
                            State::Occupied => occupied_count += 1,
                            State::Empty => {}
                            State::Floor => continue,
                        }
                        break;
                    }
                }

                if field == State::Empty && occupied_count == 0 {
                    next[x][y] = State::Occupied
                } else if field == State::Occupied && occupied_count >= 5 {
                    next[x][y] = State::Empty
                } else {
                    next[x][y] = field
                }
            }
        }

        if next == curr {
            break;
        }
        std::mem::swap(&mut next, &mut curr);
    }

    curr.into_iter()
        .flatten()
        .filter(|&s| s == State::Occupied)
        .count()
}

#[test]
fn test_part1() {
    let values = input_generator(PUZZLE);
    assert_eq!(37, part1(&values));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day11.txt").trim());
    assert_eq!(2489, part1(&values));
}

#[test]
fn test_part2() {
    let values = input_generator(PUZZLE);
    assert_eq!(26, part2(&values));
}

#[test]
fn test_part2_solution() {
    let values = input_generator(include_str!("../input/2020/day11.txt").trim());
    assert_eq!(2180, part2(&values));
}

#[cfg(test)]
static PUZZLE: &str = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;
