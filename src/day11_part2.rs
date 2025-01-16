use std::time::Instant;

use crate::utils::grid2d::Grid2D;

// personal functions
//use crate::utils::grid2d;
// use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 11 - Part 2 --");
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
                    .get_visible(l, c)
                    .iter()
                    .filter(|x| x.2 == '#')
                    .collect::<Vec<_>>()
                    .len();
                if grid.get_at((l, c)) == 'L' && adj_occ == 0 {
                    new_grid.set_at((l, c), '#');
                    change += 1;
                } else if grid.get_at((l, c)) == '#' && adj_occ >= 5 {
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

impl Grid2D {
    pub fn get_visible(&self, l: usize, c: usize) -> Vec<(usize, usize, char)> {
        let mut ret = Vec::new();
        let adj: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (dl, dc) in adj {
            let mut l2 = l;
            let mut c2 = c;
            while let Some((new_l, new_c)) = self.is_next_valid((l2, c2), (dl, dc)) {
                if self.get_at((new_l, new_c)) != '.' {
                    ret.push((new_l, new_c, self.grid[new_l][new_c]));
                    break;
                }
                l2 = new_l;
                c2 = new_c;
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_visible() {
        let input = ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....";
        let grid = Grid2D::new(input);
        assert_eq!(grid.get_visible(4, 3).len(), 8);

        let input = ".............
.L.L.#.#.#.#.
.............";
        let grid = Grid2D::new(input);
        assert_eq!(grid.get_visible(1, 1), vec![(1, 3, 'L')]);

        let input = ".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.";
        let grid = Grid2D::new(input);
        assert_eq!(grid.get_visible(3, 3), vec![]);
   }

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day11_input_demo1.txt")),
            Some(26)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day11_input.txt")),
            Some(2059)
        );
    }
}
