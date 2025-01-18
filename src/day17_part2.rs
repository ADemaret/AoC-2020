use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 17 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day17_input_demo1.txt");
    let input = include_str!("../assets/day17_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

fn get_answer(input: &str) -> Option<usize> {
    let mut grid = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                grid.insert(
                    Point {
                        x: x as isize,
                        y: y as isize,
                        z: 0,
                        w: 0,
                    },
                    c,
                );
            }
        })
    });
    // print(&grid);

    for _cycle in 1..=6 {
        let mut new_grid = HashMap::new();
        let x_min = grid.iter().min_by_key(|(pt, _)| pt.x).unwrap().0.x - 1;
        let x_max = grid.iter().max_by_key(|(pt, _)| pt.x).unwrap().0.x + 1;
        let y_min = grid.iter().min_by_key(|(pt, _)| pt.y).unwrap().0.y - 1;
        let y_max = grid.iter().max_by_key(|(pt, _)| pt.y).unwrap().0.y + 1;
        let z_min = grid.iter().min_by_key(|(pt, _)| pt.z).unwrap().0.z - 1;
        let z_max = grid.iter().max_by_key(|(pt, _)| pt.z).unwrap().0.z + 1;
        let w_min = grid.iter().min_by_key(|(pt, _)| pt.w).unwrap().0.w - 1;
        let w_max = grid.iter().max_by_key(|(pt, _)| pt.w).unwrap().0.w + 1;
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                for z in z_min..=z_max {
                    for w in w_min..=w_max {
                        let an = count_active_neigbours(&grid, x, y, z, w);
                        // println!(
                        //     "at {x},{y},{z} ({}): {an} active neigbours",
                        //     is_inactive(&grid, x, y, z)
                        // );
                        if is_inactive(&grid, x, y, z, w) && an == 3
                            || !is_inactive(&grid, x, y, z, w) && [2, 3].contains(&an)
                        {
                            // println!("add #");
                            new_grid.insert(Point { x, y, z, w }, '#');
                        }
                    }
                }
            }
        }
        // dbg!(&new_grid);
        grid = new_grid.clone();
        // println!("after {cycle} cycle");
        // print(&grid);
    }

    // count
    Some(grid.len())
}

fn count_active_neigbours(
    grid: &HashMap<Point, char>,
    x: isize,
    y: isize,
    z: isize,
    w: isize,
) -> usize {
    let mut result = 0;
    for xi in x - 1..=x + 1 {
        for yi in y - 1..=y + 1 {
            for zi in z - 1..=z + 1 {
                for wi in w - 1..=w + 1 {
                    if !(is_inactive(grid, xi, yi, zi, wi)
                        || xi == x && yi == y && zi == z && wi == w)
                    {
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

fn is_inactive(grid: &HashMap<Point, char>, x: isize, y: isize, z: isize, w: isize) -> bool {
    if let Some(pt) = grid.get(&Point { x, y, z, w }) {
        if *pt == '#' {
            return false;
        }
    }
    true
}

fn _print(grid: &HashMap<Point, char>) {
    let x_min = grid.iter().min_by_key(|(pt, _)| pt.x).unwrap().0.x;
    let x_max = grid.iter().max_by_key(|(pt, _)| pt.x).unwrap().0.x;
    let y_min = grid.iter().min_by_key(|(pt, _)| pt.y).unwrap().0.y;
    let y_max = grid.iter().max_by_key(|(pt, _)| pt.y).unwrap().0.y;
    let z_min = grid.iter().min_by_key(|(pt, _)| pt.z).unwrap().0.z;
    let z_max = grid.iter().max_by_key(|(pt, _)| pt.z).unwrap().0.z;
    let w_min = grid.iter().min_by_key(|(pt, _)| pt.w).unwrap().0.w;
    let w_max = grid.iter().max_by_key(|(pt, _)| pt.w).unwrap().0.w;

    for w in w_min..=w_max {
        for z in z_min..=z_max {
            println!("z={z}");
            for y in y_min..=y_max {
                for x in x_min..=x_max {
                    if grid.get(&Point { x, y, z, w }).is_some() {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            println!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day17_input_demo1.txt")),
            Some(848)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day17_input.txt")),
            Some(960)
        );
    }
}
