use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 01 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day01_input_demo1.txt");
    let input = include_str!("../assets/day01_input.txt");

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
    let v = input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    for a in &v {
        for b in &v {
            for c in &v {
                if *a != *b && *a != *c && *a + *b + *c == 2020 {
                    return Some(*a * *b * *c);
                }
            }
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
            get_answer(include_str!("../assets/day01_input_demo1.txt")),
            Some(241861950)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day01_input.txt")),
            Some(79734368)
        );
    }
}
