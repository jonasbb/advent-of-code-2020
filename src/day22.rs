use crate::prelude::*;
use std::collections::VecDeque;

#[aoc_generator(day22)]
fn input_generator(input: &str) -> (VecDeque<u32>, VecDeque<u32>) {
    fn parse_deck(i: &str) -> VecDeque<u32> {
        i.lines()
            .skip(1)
            .map(|line| line.parse().unwrap())
            .collect()
    };

    let mut parts = input.split("\n\n");
    (
        parse_deck(parts.next().unwrap()),
        parse_deck(parts.next().unwrap()),
    )
}

#[aoc(day22, part1)]
fn part1((player1, player2): &(VecDeque<u32>, VecDeque<u32>)) -> usize {
    let mut player1 = player1.clone();
    let mut player2 = player2.clone();

    while !player1.is_empty() && !player2.is_empty() {
        let c1 = player1.pop_front().unwrap();
        let c2 = player2.pop_front().unwrap();
        let max = c1.max(c2);
        let min = c1.min(c2);
        if c1 > c2 {
            player1.push_back(max);
            player1.push_back(min);
        } else {
            player2.push_back(max);
            player2.push_back(min);
        }
    }

    let winner = if player1.len() > player2.len() {
        player1
    } else {
        player2
    };
    score(&winner)
}

fn score(cards: &VecDeque<u32>) -> usize {
    cards
        .iter()
        .zip((1..=cards.len()).rev())
        .map(|(&v, i)| v as usize * i)
        .sum()
}

#[aoc(day22, part2)]
fn part2((player1, player2): &(VecDeque<u32>, VecDeque<u32>)) -> usize {
    play_recursive_combat(player1.clone(), player2.clone(), 1).1
}

enum Winner {
    Player1,
    Player2,
}

fn play_recursive_combat(
    mut player1: VecDeque<u32>,
    mut player2: VecDeque<u32>,
    game: u32,
) -> (Winner, usize) {
    let mut states = Set::<(_, _)>::new();
    // println!("=== Game {} ===\n", game);

    while !player1.is_empty() && !player2.is_empty() {
        // println!("-- Round {} (Game {}) --", states.len() + 1, game);
        // println!("Player 1's deck: {:?}", player1);
        // println!("Player 2's deck: {:?}", player2);

        if !states.insert((player1.clone(), player2.clone())) {
            // state already contains this value
            return (Winner::Player1, score(&player1));
        }

        let c1 = player1.pop_front().unwrap();
        let c2 = player2.pop_front().unwrap();
        // println!("Player 1 plays: {}", c1);
        // println!("Player 2 plays: {}", c2);

        let winner = if player1.len() >= c1 as _ && player2.len() >= c2 as _ {
            play_recursive_combat(
                player1.iter().take(c1 as _).copied().collect(),
                player2.iter().take(c2 as _).copied().collect(),
                game + 1,
            )
            .0
        } else if c1 > c2 {
            Winner::Player1
        } else {
            Winner::Player2
        };

        match winner {
            Winner::Player1 => {
                // println!("Player 1 wins round {} of game {}!\n", states.len(), game);
                player1.push_back(c1);
                player1.push_back(c2);
            }
            Winner::Player2 => {
                // println!("Player 2 wins round {} of game {}!\n", states.len(), game);
                player2.push_back(c2);
                player2.push_back(c1);
            }
        }
    }

    if player1.len() > player2.len() {
        (Winner::Player1, score(&player1))
    } else {
        (Winner::Player2, score(&player2))
    }
}

#[test]
fn test_part1() {
    let values = input_generator(PUZZLE);
    assert_eq!(306, part1(&values));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day22.txt").trim());
    assert_eq!(34566, part1(&values));
}

#[test]
fn test_part2() {
    let values = input_generator(PUZZLE);
    assert_eq!(291, part2(&values));
}

#[test]
fn test_part2_solution() {
    let values = input_generator(include_str!("../input/2020/day22.txt").trim());
    assert_eq!(31854, part2(&values));
}

#[cfg(test)]
static PUZZLE: &str = r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"#;
