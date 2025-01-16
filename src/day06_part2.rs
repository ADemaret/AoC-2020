use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 06 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day06_input_demo1.txt");
    let input = include_str!("../assets/day06_input.txt");

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
    Some(
        input
            .split("\n\n")
            .map(|group| {
                let group_len = group.split("\n").collect::<Vec<_>>().len();
                let mut questions = HashSet::new();
                let mut stats = HashMap::new();
                group.chars().for_each(|c| {
                    if c != '\n' {
                        *stats.entry(c).or_insert(0) += 1;
                        questions.insert(c);
                    }
                });
                let mut everybody = 0;
                for q in questions {
                    if let Some(nbr) = stats.get(&q) {
                        if *nbr == group_len {
                            everybody += 1;
                        }
                    }
                }
                everybody
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day06_input_demo1.txt")),
            Some(6)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day06_input.txt")),
            Some(3435)
        );
    }
}
