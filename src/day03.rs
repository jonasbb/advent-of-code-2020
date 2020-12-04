//! [Day 3: Toboggan Trajectory](https://adventofcode.com/2020/day/3)
//!
//! # Part 1
//!
//! With the toboggan login problems resolved, you set off toward the airport.
//! While travel by toboggan might be easy, it's certainly not safe: there's very minimal steering and the area is covered in trees.
//! You'll need to see which angles will take you near the fewest trees.
//!
//! Due to the local geology, trees in this area only grow on exact integer coordinates in a grid.
//! You make a map (your puzzle input) of the open squares (`.`) and trees (`#`) you can see. For example:
//!
//! ```text
//! ..##.......
//! #...#...#..
//! .#....#..#.
//! ..#.#...#.#
//! .#...##..#.
//! ..#.##.....
//! .#.#.#....#
//! .#........#
//! #.##...#...
//! #...##....#
//! .#..#...#.#
//! ```
//!
//! These aren't the only trees, though; due to something you read about once involving arboreal genetics and biome stability, the same pattern repeats to the right many times:
//!
//! ```text
//! ..##.........##.........##.........##.........##.........##.......  --->
//! #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
//! .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! .#........#.#........#.#........#.#........#.#........#.#........#
//! #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
//! #...##....##...##....##...##....##...##....##...##....##...##....#
//! .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//! ```
//!
//! You start on the open square (`.`) in the top-left corner and need to reach the bottom (below the bottom-most row on your map).
//!
//! The toboggan can only follow a few specific slopes (you opted for a cheaper model that prefers rational numbers); start by **counting all the trees** you would encounter for the slope **right 3, down 1:**
//!
//! From your starting position at the top-left, check the position that is right 3 and down 1.
//! Then, check the position that is right 3 and down 1 from there, and so on until you go past the bottom of the map.
//!
//! The locations you'd check in the above example are marked here with `O` where there was an open square and `X` where there was a tree:
//!
//! ```text
//! ..##.........##.........##.........##.........##.........##.......  --->
//! #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
//! .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! .#........#.#........X.#........#.#........#.#........#.#........#
//! #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
//! #...##....##...##....##...#X....##...##....##...##....##...##....#
//! .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//! ```
//!
//! In this example, traversing the map using this slope would cause you to encounter `7` trees.
//!
//! Starting at the top-left corner of your map and following a slope of right 3 and down 1, **how many trees would you encounter?**
//!
//! # Part 2
//!
//! Time to check the rest of the slopes - you need to minimize the probability of a sudden arboreal stop, after all.
//!
//! Determine the number of trees you would encounter if, for each of the following slopes, you start at the top-left corner and traverse the map all the way to the bottom:
//!
//! - Right 1, down 1.
//! - Right 3, down 1. (This is the slope you already checked.)
//! - Right 5, down 1.
//! - Right 7, down 1.
//! - Right 1, down 2.
//!
//! In the above example, these slopes would find `2`, `7`, `3`, `4`, and `2` tree(s) respectively; multiplied together, these produce the answer `336`.
//!
//! **What do you get if you multiply together the number of trees encountered on each of the listed slopes?**

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Vec<bool>> {
    input
        .split('\n')
        .map(|x| x.chars().map(|c| c == '#').collect())
        .collect()
}

fn trees_on_slope(map: &[Vec<bool>], slope_right: usize, slope_down: usize) -> usize {
    let mut down = 0;
    let mut right = 0;
    let mut curr_trees = 0;
    while down < map.len() {
        let row = &map[down];
        if row.iter().cycle().nth(right) == Some(&true) {
            curr_trees += 1;
        }
        down += slope_down;
        right += slope_right;
    }
    curr_trees
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<bool>]) -> usize {
    trees_on_slope(input, 3, 1)
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<bool>]) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(right, down)| trees_on_slope(input, *right, *down))
        .product()
}

#[test]
fn test_part1() {
    let values = input_generator(PUZZLE);
    assert_eq!(7, part1(&values));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day3.txt").trim());
    assert_eq!(259, part1(&values));
}

#[test]
fn test_slopes() {
    let values = input_generator(PUZZLE);
    assert_eq!(2, trees_on_slope(&values, 1, 1));
    assert_eq!(7, trees_on_slope(&values, 3, 1));
    assert_eq!(3, trees_on_slope(&values, 5, 1));
    assert_eq!(4, trees_on_slope(&values, 7, 1));
    assert_eq!(2, trees_on_slope(&values, 1, 2));
}

#[test]
fn test_part2() {
    let values = input_generator(PUZZLE);
    assert_eq!(336, part2(&values));
}

#[test]
fn test_part2_solution() {
    let values = input_generator(include_str!("../input/2020/day3.txt").trim());
    assert_eq!(2224913600, part2(&values));
}

#[cfg(test)]
static PUZZLE: &str = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;
