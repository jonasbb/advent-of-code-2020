use crate::prelude::*;
use std::fmt::Display;

#[derive(Clone)]
struct Cups {
    cursor: usize,
    values: Vec<u8>,
}

impl Cups {
    fn insert(&mut self, pos: usize, values: [u8; 3]) {
        self.values.insert(pos + 1, values[2]);
        self.values.insert(pos + 1, values[1]);
        self.values.insert(pos + 1, values[0]);
        if pos < self.cursor {
            self.cursor += 3;
        }
    }

    fn pick_up(&mut self) -> [u8; 3] {
        [self.take_next(), self.take_next(), self.take_next()]
    }

    fn take_next(&mut self) -> u8 {
        let remove_pos = (self.cursor + 1) % self.values.len();
        let res = self.values.remove(remove_pos);
        if remove_pos < self.cursor {
            self.cursor -= 1;
        }
        res
    }

    fn cups_order(&self) -> String {
        self.values
            .iter()
            .cycle()
            // search for the 1
            .skip_while(|&&v| v != 1)
            // skip the 1
            .skip(1)
            // Take all other values and create string
            .take(8)
            .map(|&v| (v + b'0') as char)
            .collect()
    }

    fn destination(&mut self) -> (usize, u8) {
        let mut cursor_value = self.values[self.cursor];
        loop {
            cursor_value = cursor_value
                .checked_sub(1)
                .unwrap_or_else(|| self.values.iter().copied().max().unwrap());
            match self.values.iter().find_position(|&&v| v == cursor_value) {
                None => continue,
                Some((position, &destination)) => {
                    return (position, destination);
                }
            }
        }
    }

    fn advance(&mut self) {
        self.cursor = (self.cursor + 1) % self.values.len();
    }
}

impl Display for Cups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, v) in self.values.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            if i == self.cursor {
                write!(f, "(")?;
            }
            v.fmt(f)?;
            if i == self.cursor {
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}

#[aoc_generator(day23)]
fn input_generator(input: &str) -> Cups {
    let values = input.chars().map(|c| c as u8 - b'0').collect();
    Cups { cursor: 0, values }
}

#[aoc(day23, part1)]
fn part1(cups: &Cups) -> String {
    run(100, cups.clone())
}

fn run(moves: u8, mut cups: Cups) -> String {
    for m in 0..moves {
        println!("-- move {} --\ncups: {}", m + 1, cups);
        let pick_up = cups.pick_up();
        println!("pick up: {}, {}, {}", pick_up[0], pick_up[1], pick_up[2]);
        let (pos, dest) = cups.destination();
        println!("destination: {}\n", dest);
        cups.insert(pos, pick_up);
        cups.advance();
    }
    cups.cups_order()
}

// #[aoc(day23, part2)]
// fn part2((player1, player2): &(VecDeque<u32>, VecDeque<u32>)) -> usize {
//     todo!()
// }

#[test]
fn test_part1_small() {
    let values = input_generator(PUZZLE);
    assert_eq!("92658374", run(10, values));
}

#[test]
fn test_part1() {
    let values = input_generator(PUZZLE);
    assert_eq!("67384529", part1(&values));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day23.txt").trim());
    assert_eq!("69473825", part1(&values));
}

// #[test]
// fn test_part2() {
//     let values = input_generator(PUZZLE);
//     assert_eq!(291, part2(&values));
// }

// #[test]
// fn test_part2_solution() {
//     let values = input_generator(include_str!("../input/2020/day23.txt").trim());
//     assert_eq!(31854, part2(&values));
// }

#[cfg(test)]
static PUZZLE: &str = r#"389125467"#;
