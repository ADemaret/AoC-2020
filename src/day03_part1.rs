use std::time::Instant;

use crate::utils::grid2d::Grid2D;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 03 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day03_input_demo1.txt");
    let input = include_str!("../assets/day03_input.txt");

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
    let grid = Grid2D::new(input);
    let mut current = (0, 0);
    let mut nbr = 0;
    while current.0 < grid.max_l {
        if grid.get_at(current) == '#' {
            nbr += 1;
        }
        current.0 += 1;
        current.1 = (current.1 + 3) % (grid.max_c);
    }
    Some(nbr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day03_input_demo1.txt")),
            Some(7)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day03_input.txt")),
            Some(286)
        );
    }
}
