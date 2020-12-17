use crate::prelude::*;
use misc_utils::{Max, Min};

#[aoc_generator(day17)]
fn input_generator(input: &str) -> Set<(i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(line_idx, line)| {
            line.chars().enumerate().filter_map(move |(char_idx, c)| {
                if c == '#' {
                    Some((line_idx as _, char_idx as _))
                } else {
                    None
                }
            })
        })
        .collect()
}

#[aoc(day17, part1)]
fn part1(input: &Set<(i32, i32)>) -> usize {
    let mut state: Set<(_, _, _)> = input.iter().map(|&(x, y)| (x, y, 0)).collect();
    let mut new_state = state.clone();

    fn neighbors3((xp, yp, zp): (i32, i32, i32)) -> impl Iterator<Item = (i32, i32, i32)> {
        [-1, 0, 1]
            .iter()
            .flat_map(move |&x| {
                [-1, 0, 1]
                    .iter()
                    .flat_map(move |&y| [-1, 0, 1].iter().map(move |&z| (xp + x, yp + y, zp + z)))
            })
            .filter(move |p| p != &(xp, yp, zp))
    }

    for _ in 0..6 {
        // Find min and max values in all dimensions
        let mut min_x = Min::new();
        let mut max_x = Max::new();
        let mut min_y = Min::new();
        let mut max_y = Max::new();
        let mut min_z = Min::new();
        let mut max_z = Max::new();

        for (x, y, z) in &state {
            min_x.update(*x);
            max_x.update(*x);
            min_y.update(*y);
            max_y.update(*y);
            min_z.update(*z);
            max_z.update(*z);
        }

        let min_x = min_x.get_min_extreme() - 1;
        let min_y = min_y.get_min_extreme() - 1;
        let min_z = min_z.get_min_extreme() - 1;
        let max_x = max_x.get_max_extreme() + 1;
        let max_y = max_y.get_max_extreme() + 1;
        let max_z = max_z.get_max_extreme() + 1;

        new_state.clone_from(&state);

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    let actives = neighbors3((x, y, z)).filter(|p| state.contains(p)).count();

                    match (state.contains(&(x, y, z)), actives) {
                        (true, 2) | (true, 3) => {}
                        (true, _) => {
                            new_state.remove(&(x, y, z));
                        }
                        (false, 3) => {
                            new_state.insert((x, y, z));
                        }
                        _ => {}
                    }
                }
            }
        }

        std::mem::swap(&mut state, &mut new_state);
    }

    state.len()
}

#[aoc(day17, part2)]
fn part2(input: &Set<(i32, i32)>) -> usize {
    let mut state: Set<(_, _, _, _)> = input.iter().map(|&(x, y)| (x, y, 0, 0)).collect();
    let mut new_state = state.clone();

    fn neighbors4(
        (xp, yp, zp, wp): (i32, i32, i32, i32),
    ) -> impl Iterator<Item = (i32, i32, i32, i32)> {
        [-1, 0, 1]
            .iter()
            .flat_map(move |&x| {
                [-1, 0, 1].iter().flat_map(move |&y| {
                    [-1, 0, 1].iter().flat_map(move |&w| {
                        [-1, 0, 1]
                            .iter()
                            .map(move |&z| (xp + x, yp + y, zp + z, wp + w))
                    })
                })
            })
            .filter(move |p| p != &(xp, yp, zp, wp))
    }

    for _ in 0..6 {
        // Find min and max values in all dimensions
        let mut min_x = Min::new();
        let mut max_x = Max::new();
        let mut min_y = Min::new();
        let mut max_y = Max::new();
        let mut min_z = Min::new();
        let mut max_z = Max::new();
        let mut min_w = Min::new();
        let mut max_w = Max::new();

        for (x, y, z, w) in &state {
            min_x.update(*x);
            max_x.update(*x);
            min_y.update(*y);
            max_y.update(*y);
            min_z.update(*z);
            max_z.update(*z);
            min_w.update(*w);
            max_w.update(*w);
        }

        let min_x = min_x.get_min_extreme() - 1;
        let min_y = min_y.get_min_extreme() - 1;
        let min_z = min_z.get_min_extreme() - 1;
        let min_w = min_w.get_min_extreme() - 1;
        let max_x = max_x.get_max_extreme() + 1;
        let max_y = max_y.get_max_extreme() + 1;
        let max_z = max_z.get_max_extreme() + 1;
        let max_w = max_w.get_max_extreme() + 1;

        new_state.clone_from(&state);

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    for w in min_w..=max_w {
                        let p = (x, y, z, w);
                        let actives = neighbors4(p).filter(|p| state.contains(p)).count();

                        match (state.contains(&p), actives) {
                            (true, 2) | (true, 3) => {}
                            (true, _) => {
                                new_state.remove(&p);
                            }
                            (false, 3) => {
                                new_state.insert(p);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        std::mem::swap(&mut state, &mut new_state);
    }

    state.len()
}

#[test]
fn test_part1() {
    let values = input_generator(PUZZLE);
    assert_eq!(112, part1(&values));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day17.txt").trim());
    assert_eq!(267, part1(&values));
}

#[test]
fn test_part2() {
    let values = input_generator(PUZZLE);
    assert_eq!(848, part2(&values));
}

#[test]
fn test_part2_solution() {
    let values = input_generator(include_str!("../input/2020/day17.txt").trim());
    assert_eq!(1812, part2(&values));
}

#[cfg(test)]
static PUZZLE: &str = r#".#.
..#
###"#;
