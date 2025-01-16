use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 02 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day02_input_demo1.txt");
    let input = include_str!("../assets/day02_input.txt");

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
            .lines()
            .map(|line| {
                let chunks = line
                    .split(['-', ' ', ':'])
                    .filter(|ch| !ch.is_empty())
                    .collect::<Vec<_>>();
                let min = chunks[0].parse::<usize>().unwrap();
                let max = chunks[1].parse::<usize>().unwrap();
                let nbr = chunks[3]
                    .chars()
                    .filter(|&c| c == chunks[2].chars().next().unwrap())
                    .collect::<Vec<_>>()
                    .len();
                if nbr >= min && nbr <= max {
                    1
                } else {
                    0
                }
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
            get_answer(include_str!("../assets/day02_input_demo1.txt")),
            Some(2)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day02_input.txt")),
            Some(643)
        );
    }
}
