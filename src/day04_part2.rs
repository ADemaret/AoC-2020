use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 04 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day04_input_demo1.txt");
    let input = include_str!("../assets/day04_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> Option<usize> {
    let needed = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let nbr_needed = needed.len();

    Some(
        input
            .split("\n\n")
            .map(|psp| {
                let fields = psp.split(['\n', ' ']).collect::<Vec<_>>();
                let mut valid = 0;
                for n in &needed {
                    for f in &fields {
                        if f.starts_with(n) {
                            if is_valid(n, f) {
                                // println!("{f} is valid");
                                valid += 1;
                            } else {
                                // println!("{f} is NOT valid");
                            }
                            break;
                        }
                    }
                }
                if valid < nbr_needed {
                    0
                } else {
                    1
                }
            })
            .sum::<usize>(),
    )
}

fn is_valid(keyword: &str, field: &str) -> bool {
    match keyword {
        "byr" => {
            let (_, str) = field.split_once(":").unwrap();
            if let Ok(nbr) = str.parse::<usize>() {
                if (1920..=2002).contains(&nbr) {
                    return true;
                }
            }
        }
        "iyr" => {
            let (_, str) = field.split_once(":").unwrap();
            if let Ok(nbr) = str.parse::<usize>() {
                if (2010..=2020).contains(&nbr) {
                    return true;
                }
            }
        }
        "eyr" => {
            let (_, str) = field.split_once(":").unwrap();
            if let Ok(nbr) = str.parse::<usize>() {
                if (2020..=2030).contains(&nbr) {
                    return true;
                }
            }
        }
        "hgt" => {
            let (_, str) = field.split_once(":").unwrap();
            if str.ends_with("in") {
                if let Ok(nbr) = str.strip_suffix("in").unwrap().parse::<usize>() {
                    if (59..=76).contains(&nbr) {
                        return true;
                    }
                }
            } else if str.ends_with("cm") {
                if let Ok(nbr) = str.strip_suffix("cm").unwrap().parse::<usize>() {
                    if (150..=193).contains(&nbr) {
                        return true;
                    }
                }
            }
        }
        "hcl" => {
            let (_, str) = field.split_once(":").unwrap();
            if str.starts_with('#') {
                let str2 = str
                    .chars()
                    .skip(1)
                    .filter(|c| ('a'..='f').contains(c) || c.is_ascii_digit())
                    .collect::<Vec<_>>();
                if str2.len() == 6 {
                    return true;
                }
            }
        }
        "ecl" => {
            let (_, str) = field.split_once(":").unwrap();
            if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&str) {
                return true;
            }
        }
        "pid" => {
            let (_, str) = field.split_once(":").unwrap();
            let str2 = str
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<_>>();
            if str2.len() == 9 {
                return true;
            }
        }
        _ => panic!(),
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day04_input_demo2.txt")),
            Some(0)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day04_input_demo3.txt")),
            Some(4)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day04_input.txt")),
            Some(140)
        );
    }
}
