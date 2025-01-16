use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 10 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day10_input_demo1.txt");
    let input = include_str!("../assets/day10_input.txt");

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
    let mut adapters = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    adapters.sort();
    // dbg!(&adapters);

    let mut diff = [0, 0, 0];
    let mut eff = 0;
    for a in &adapters {
        // println!("{a}, {eff}");
        diff[a - eff - 1] += 1;
        eff = *a;
    }
    diff[2] += 1;
    // dbg!(diff);

    Some(diff[0] * diff[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day10_input_demo1.txt")),
            Some(35)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day10_input_demo2.txt")),
            Some(220)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day10_input.txt")),
            Some(2450)
        );
    }
}
