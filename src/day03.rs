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
