use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 21 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day21_input_demo1.txt");
    let input = include_str!("../assets/day21_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> Option<String> {
    let mut recipes = input
        .lines()
        .map(|line| {
            let (f, a) = line.split_once("contains").unwrap();
            (
                f.split(['(', ')', ' ', ','])
                    .filter(|str| !str.is_empty())
                    .map(|item| item.to_string())
                    .collect::<HashSet<_>>(),
                a.split(['(', ')', ' ', ','])
                    .filter(|str| !str.is_empty())
                    .map(|item| item.to_string())
                    .collect::<HashSet<_>>(),
            )
        })
        .collect::<Vec<_>>();

    // list ingredients and freq
    let mut ingredients = HashMap::new();
    let rec2 = recipes.clone();
    for (i, _) in rec2.iter().enumerate() {
        for ing in &rec2[i].0 {
            let nbr = ingredients.entry(ing).or_insert(0);
            *nbr += 1;
        }
    }
    let mut allergens = HashSet::new();
    for (i, _) in recipes.iter().enumerate() {
        for al in &recipes[i].1 {
            allergens.insert(al.clone());
        }
    }

    let mut ingr_allerg = Vec::new();
    while let Some((ing, all)) = find_one_common_allergen(recipes.clone(), allergens.clone()) {
        update_recipes(&mut recipes, ing.clone(), all.clone());
        ingr_allerg.push((all, ing));
    }

    ingr_allerg.sort();

    Some(
        ingr_allerg
            .iter()
            .map(|(_, v)| v.as_str())
            .collect::<Vec<_>>()
            .join(","),
    )
}

fn update_recipes(recipes: &mut Vec<(HashSet<String>, HashSet<String>)>, ing: String, all: String) {
    let mut recipes_to_remove = Vec::new();

    for (i, r) in recipes.iter_mut().enumerate() {
        r.0.remove(&ing);
        r.1.remove(&all);
        if r.0.is_empty() {
            recipes_to_remove.push(i);
        }
    }
    recipes_to_remove.reverse();
    for i in recipes_to_remove {
        recipes.remove(i);
    }
}

fn find_one_common_allergen(
    recipes: Vec<(HashSet<String>, HashSet<String>)>,
    allergens: HashSet<String>,
) -> Option<(String, String)> {
    for all in &allergens {
        let mut common_ing: Vec<&String>;
        for (l1, _) in recipes.iter().enumerate() {
            if recipes[l1].1.contains(all) {
                common_ing = recipes[l1].0.iter().collect();
                for (l2, _) in recipes.iter().enumerate() {
                    if l1 != l2 && recipes[l2].1.contains(all) {
                        // only keep common ingredients
                        let l2_ing = recipes[l2].0.iter().collect::<Vec<_>>();
                        common_ing = common_ing
                            .iter()
                            .filter(|item| l2_ing.contains(item))
                            .copied()
                            .collect::<Vec<_>>();

                        if common_ing.len() == 1 {
                            // println!("{} contains {}", common_ing[0], all);
                            return Some((common_ing[0].to_string(), all.to_string()));
                        }
                    }
                }
            }
        }
    }
    // search a recipe containing the allergen
    for r in &recipes {
        // if the recipe has only one ingredient, it is the allergen
        if r.0.len() == 1 && r.1.len() == 1 {
            let ing = r.0.iter().next().unwrap().to_string();
            let all = r.1.iter().next().unwrap().to_string();
            // println!("{ing} contains {all}");
            return Some((ing, all));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day21_input_demo1.txt")),
            Some("mxmxvkd,sqjhc,fvjkl".to_string())
        );
        assert_eq!(
            get_answer(include_str!("../assets/day21_input.txt")),
            Some("fbtqkzc,jbbsjh,cpttmnv,ccrbr,tdmqcl,vnjxjg,nlph,mzqjxq".to_string())
        );
    }
}
