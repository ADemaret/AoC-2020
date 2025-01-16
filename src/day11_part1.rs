use std::time::Instant;

use crate::utils::grid2d::Grid2D;

// personal functions
//use crate::utils::grid2d;
// use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 11 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day11_input_demo1.txt");
    let input = include_str!("../assets/day11_input.txt");

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
    let mut grid = Grid2D::new(input);

    let mut new_grid = grid.clone();
    loop {
        let mut change = 0;
        for l in 0..grid.max_l {
            for c in 0..grid.max_c {
                let adj_occ = grid
                    .get_adjacents(l, c)
                    .iter()
                    .filter(|x| x.2 == '#')
                    .collect::<Vec<_>>()
                    .len();
                if grid.get_at((l, c)) == 'L' && adj_occ == 0 {
                    new_grid.set_at((l, c), '#');
                    change += 1;
                } else if grid.get_at((l, c)) == '#' && adj_occ >= 4 {
                    new_grid.set_at((l, c), 'L');
                    change += 1;
                }
            }
        }
        grid = new_grid.clone();

        // new_grid.print();
        // pause::pause();

        if change == 0 {
            return Some(grid.count_occurences('#'));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day11_input_demo1.txt")),
            Some(37)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day11_input.txt")),
            Some(2289)
        );
    }
}
