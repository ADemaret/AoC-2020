use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 13 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day13_input_demo1.txt");
    let input = include_str!("../assets/day13_input.txt");

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
    let (str_time, str_bus) = input.split_once("\n").unwrap();
    let time = str_time.parse::<usize>().unwrap();
    let mut result = 0;
    let mut min_wait = usize::MAX;
    for b in str_bus.split(",").filter(|b| *b != "x").collect::<Vec<_>>() {
        let bus = b.parse::<usize>().unwrap();
        let wait = bus - time % bus;
        if wait < min_wait {
            min_wait = wait;
            result = bus * wait;
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day13_input_demo1.txt")),
            Some(295)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day13_input.txt")),
            Some(3997)
        );
    }
}
