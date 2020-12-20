use crate::prelude::*;
use std::convert::TryInto;

struct Tile {
    id: u64,
    pixels: [[bool; 10]; 10],
}

impl Tile {
    fn borders(&self) -> [[bool; 10]; 8] {
        let tmp = [
            self.pixels[0],
            {
                (0..10)
                    .map(|i| self.pixels[i][0])
                    .collect_vec()
                    .try_into()
                    .unwrap()
            },
            {
                let mut tmp = self.pixels[9];
                tmp.reverse();
                tmp
            },
            {
                (0..10)
                    .rev()
                    .map(|i| self.pixels[i][9])
                    .collect_vec()
                    .try_into()
                    .unwrap()
            },
        ];
        [
            tmp[0],
            tmp[1],
            tmp[2],
            tmp[3],
            {
                let mut x = tmp[0];
                x.reverse();
                x
            },
            {
                let mut x = tmp[1];
                x.reverse();
                x
            },
            {
                let mut x = tmp[2];
                x.reverse();
                x
            },
            {
                let mut x = tmp[3];
                x.reverse();
                x
            },
        ]
    }
}

#[aoc_generator(day20)]
fn input_generator(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .map(|tile| {
            let mut lines = tile.lines();
            let id = lines
                .next()
                .unwrap()
                .strip_prefix("Tile ")
                .unwrap()
                .strip_suffix(":")
                .unwrap()
                .parse()
                .unwrap();
            let pixels = lines
                .map(|line| {
                    line.chars()
                        .map(|c| c == '#')
                        .collect_vec()
                        .try_into()
                        .unwrap()
                })
                .collect_vec()
                .try_into()
                .unwrap();
            Tile { id, pixels }
        })
        .collect()
}

#[aoc(day20, part1)]
fn part1(input: &[Tile]) -> u64 {
    // Store all tile borders
    let mut borders = Map::<_, Set<u64>>::new();
    for tile in input {
        for &border in &tile.borders() {
            borders.entry(border).or_default().insert(tile.id);
        }
    }
    // Count the unique borders
    let mut id_count = Map::<_, u32>::new();
    for (border, ids) in &borders {
        if ids.len() == 1 {
            println!("{:?} => {:?}", border, ids);
            *id_count
                .entry(ids.iter().next().cloned().unwrap())
                .or_default() += 1;
        } else if ids.len() > 2 {
            println!("{:?} => {:?}", border, ids);
        }
    }
    // Count those tiles with 4 unique borders, these are corners
    // Actually, only 2 unique, but each side is also unique if flipped, thus 4
    id_count
        .into_iter()
        .filter(|(_, count)| *count == 4)
        .map(|(id, _)| id)
        .product()
}

// #[aoc(day20, part2)]
// fn part2(input: &[Tile]) -> u64 {
//     todo!()
// }

#[test]
fn test_part1() {
    let values = input_generator(PUZZLE);
    assert_eq!(20899048083289, part1(&values));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day20.txt").trim());
    assert_eq!(47213728755493, part1(&values));
}

// #[test]
// fn test_part2() {
//     let values = input_generator(PUZZLE);
//     assert_eq!(46 + 1445 + 669060 + 23340, part2(&values));
// }

// #[test]
// fn test_part2_solution() {
//     let values = input_generator(include_str!("../input/2020/day19.txt").trim());
//     assert_eq!(88534268715686, part2(&values));
// }

#[cfg(test)]
static PUZZLE: &str = r#"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."#;
