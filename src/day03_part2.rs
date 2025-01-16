use std::time::Instant;

use crate::utils::grid2d::Grid2D;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 03 - Part 2 --");
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
    let nbr = [
        count_trees(&grid, 1, 1),
        count_trees(&grid, 1, 3),
        count_trees(&grid, 1, 5),
        count_trees(&grid, 1, 7),
        count_trees(&grid, 2, 1),
    ];

    Some(nbr.iter().product())
}

fn count_trees(grid: &Grid2D, dl: usize, dc: usize) -> usize {
    let mut current = (0, 0);
    let mut nbr = 0;
    while current.0 < grid.max_l {
        if grid.get_at(current) == '#' {
            nbr += 1;
        }
        current.0 += dl;
        current.1 = (current.1 + dc) % (grid.max_c);
    }
    nbr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day03_input_demo1.txt")),
            Some(336)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day03_input.txt")),
            Some(3638606400)
        );
    }
}
