use crate::prelude::*;

struct Recipe {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

#[aoc_generator(day21)]
fn input_generator(input: &str) -> Vec<Recipe> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.strip_suffix(')').unwrap().split(" (contains ");
            let ingredients = parts
                .next()
                .unwrap()
                .split(' ')
                .map(ToString::to_string)
                .collect();
            let allergens = parts
                .next()
                .unwrap()
                .split(", ")
                .map(ToString::to_string)
                .collect();
            Recipe {
                ingredients,
                allergens,
            }
        })
        .collect()
}

#[aoc(day21, part1)]
fn part1(input: &[Recipe]) -> usize {
    let all_ingredients: Set<&str> = input
        .iter()
        .flat_map(|r| r.ingredients.iter())
        .map(|s| s.as_ref())
        .collect();
    let all_allergens: Set<&str> = input
        .iter()
        .flat_map(|r| r.allergens.iter())
        .map(|s| s.as_ref())
        .collect();

    // Count how often the ingredients without allergens appear
    cannot_have_allergens(input, &all_ingredients, &all_allergens)
        .flat_map(|ing| {
            input
                .iter()
                .filter(move |r| r.ingredients.iter().any(|i| i == ing))
        })
        .count()
}

fn cannot_have_allergens<'a>(
    input: &'a [Recipe],
    all_ingredients: &'a Set<&'a str>,
    all_allergens: &'a Set<&'a str>,
) -> impl Iterator<Item = &'a str> {
    // Check which ingredients definitly do no contain allergens
    // If an allergen is listed, the ingredient must be listed too.
    all_ingredients.iter().cloned().filter(move |&ing| {
        for &allergen in all_allergens {
            let ing_has_not_allergen = input
                .iter()
                .filter(|r| r.allergens.iter().any(|a| a == allergen))
                .any(|r| !r.ingredients.iter().any(|i| i == ing));
            if !ing_has_not_allergen {
                // ing could have allergen
                return false;
            }
        }
        true
    })
}

#[aoc(day21, part2)]
fn part2(input: &[Recipe]) -> String {
    let all_ingredients: Set<&str> = input
        .iter()
        .flat_map(|r| r.ingredients.iter())
        .map(|s| s.as_ref())
        .collect();
    let all_allergens: Set<&str> = input
        .iter()
        .flat_map(|r| r.allergens.iter())
        .map(|s| s.as_ref())
        .collect();

    // Count how often the ingredients without allergens appear
    let mut could_contain_allergens = all_ingredients.clone();
    for ing in cannot_have_allergens(input, &all_ingredients, &all_allergens) {
        could_contain_allergens.remove(ing);
    }

    let mut ingredient_allergen_mapping: Map<&str, Set<&str>> = could_contain_allergens
        .into_iter()
        .map(|ing| (ing, all_allergens.clone()))
        .collect();

    // Remove allergens which cannot be the case
    ingredient_allergen_mapping
        .iter_mut()
        .for_each(|(&ing, allergens)| {
            for &all in &all_allergens {
                let ing_not_allergen = input
                    .iter()
                    .filter(|r| r.allergens.iter().any(|a| a == all))
                    .any(|r| !r.ingredients.iter().any(|i| i == ing));
                if ing_not_allergen {
                    allergens.remove(all);
                }
            }
        });

    let mut assigned_allergens: Set<&str> = Set::new();
    for _ in 0..10 {
        println!("\nAssigned Allergend: {:?}", assigned_allergens);
        println!("Possible Assignments:");
        for (ingredient, allergens) in &ingredient_allergen_mapping {
            println!("  {}: {:?}", ingredient, allergens);
        }

        // while assigned_allergens.len() != all_allergens.len() {
        // Collect all allergens which are already assigned
        assigned_allergens = ingredient_allergen_mapping
            .values()
            .filter(|all| all.len() == 1)
            .map(|all| *all.iter().next().unwrap())
            .collect();
        // Remove them from any mapping which still contain assigned allergens
        ingredient_allergen_mapping
            .values_mut()
            .filter(|all| all.len() > 1)
            .for_each(|all| {
                for &a in &assigned_allergens {
                    all.remove(a);
                }
            });
    }

    // Sort output and produce output string
    ingredient_allergen_mapping
        .into_iter()
        .map(|(ing, all)| (ing, all.into_iter().next().unwrap()))
        .sorted_by_key(|x| x.1)
        .map(|x| x.0)
        .intersperse(",")
        .collect()
}

#[test]
fn test_part1() {
    let values = input_generator(PUZZLE);
    assert_eq!(5, part1(&values));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2020/day21.txt").trim());
    assert_eq!(1930, part1(&values));
}

#[test]
fn test_part2() {
    let values = input_generator(PUZZLE);
    assert_eq!("mxmxvkd,sqjhc,fvjkl", part2(&values));
}

#[test]
fn test_part2_solution() {
    let values = input_generator(include_str!("../input/2020/day21.txt").trim());
    assert_eq!("spcqmzfg,rpf,dzqlq,pflk,bltrbvz,xbdh,spql,bltzkxx", part2(&values));
}

#[cfg(test)]
static PUZZLE: &str = r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"#;
