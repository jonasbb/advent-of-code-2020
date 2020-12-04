#[cfg(test)]
static PUZZLE: &str = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Vec<(String, String)>> {
    input
        .split("\n\n")
        .map(|passport| {
            passport
                .replace("\n", " ")
                .split(" ")
                .map(|fields| {
                    let mut fields = fields.split(":");
                    (
                        fields.next().unwrap().to_string(),
                        fields.next().unwrap().to_string(),
                    )
                })
                .collect()
        })
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

#[aoc(day4, part1)]
fn part1(input: &[Vec<(String, String)>]) -> usize {
    input
        .iter()
        .filter(|passport| has_all_parts(passport))
        .count()
}

fn has_all_parts(fields: &[(String, String)]) -> bool {
    let mut byr = false;
    let mut iyr = false;
    let mut eyr = false;
    let mut hgt = false;
    let mut hcl = false;
    let mut ecl = false;
    let mut pid = false;
    for (key, _) in fields {
        match &**key {
            "byr" => byr = true,
            "iyr" => iyr = true,
            "eyr" => eyr = true,
            "hgt" => hgt = true,
            "hcl" => hcl = true,
            "ecl" => ecl = true,
            "pid" => pid = true,
            _ => {}
        }
    }
    byr && iyr && eyr && hgt && hcl && ecl && pid
}

#[test]
fn test_part1() {
    let values = input_generator(PUZZLE);
    assert_eq!(2, part1(&values));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day4.txt").trim());
    assert_eq!(250, part1(&values));
}

// #[test]
// fn test_part2() {
//     let values = input_generator(PUZZLE);
//     assert_eq!(336, part2(&values));
// }

// #[test]
// fn test_part2_solution() {
//     let values = input_generator(include_str!("../input/2020/day4.txt").trim());
//     assert_eq!(2224913600, part2(&values));
// }
