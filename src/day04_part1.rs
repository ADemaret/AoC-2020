use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 04 - Part 1 --");
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
                let mut valid = 0;
                let fields = psp.split(['\n', ' ']).collect::<Vec<_>>();
                let mut found = 0;
                for n in &needed {
                    for f in &fields {
                        if f.starts_with(n) {
                            found += 1;
                            break;
                        }
                    }
                }
                if nbr_needed == found {
                    valid = 1;
                }
                valid
            })
            .sum::<usize>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day04_input_demo1.txt")),
            Some(2)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day04_input.txt")),
            Some(222)
        );
    }
}
