use std::time::Instant;

use crate::utils::grid2d::Grid2D;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 20 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day20_input_demo1.txt");
    let input = include_str!("../assets/day20_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

struct Zone {
    id: usize,
    // grid: Grid2D,
    borders: Vec<Vec<char>>,
    // flip_hor: bool,
    // flip_vert: bool,
    // rotate: usize,
}

fn get_answer(input: &str) -> Option<usize> {
    // get zones
    let mut zones = Vec::new();
    let parts = input.split("\n\n").collect::<Vec<_>>();
    for p in parts {
        let id = p
            .lines()
            .take(1)
            .map(|line| {
                line.split([' ', ':'])
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect::<Vec<usize>>()[0]
            })
            .collect::<Vec<_>>()[0];
        let grid_str = p.lines().skip(1).collect::<Vec<_>>().join("\n");
        let grid = Grid2D::new(&grid_str);

        // grid.print();
        let borders = vec![
            grid.get_line(0),              // top
            grid.get_line(grid.max_l - 1), // bottom
            grid.get_column(0),            // left
            grid.get_column(grid.max_c - 1),
        ]; // right
        zones.push(Zone {
            id,
            // grid,
            borders,
            // flip_hor: false,
            // flip_vert: false,
            // rotate: 0,
        })
    }

    // find possible neighbours
    let mut corners = Vec::new();
    // println!("there are {} zones", zones.len());
    for z in &zones {
        let mut neighb = 0;
        // println!("zone {}, {:?}",z.id, z.borders);
        for b in &z.borders {
            for z2 in &zones {
                if z.id != z2.id {
                    let mut count = 0;
                    for b2 in &z2.borders {
                        if b == b2 {
                            // println!("border {i} of {} = border {i2} of {}",z.id,z2.id);
                            count += 1;
                        }
                        let mut b_rev = b.clone();
                        b_rev.reverse();
                        if b_rev == *b2 {
                            // println!("border {i} of {} = REV border {i2} of {}",z.id,z2.id);
                            count += 1;
                        }
                    }
                    match count.cmp(&1) {
                        std::cmp::Ordering::Greater => {
                            println!("** unsure neighbour !!");
                            return None;
                        }
                        std::cmp::Ordering::Equal => {
                            neighb += 1;
                        }
                        std::cmp::Ordering::Less => {}
                    }
                }
            }
        }
        match neighb {
            1 => {
                // panic!("invalid tile");
            }
            2 => {
                // println!("zone {} is a corner", z.id);
                corners.push(z.id);
            }
            3 => {}
            4 => {
                // println!("zone {} can be inside", z.id);
            }
            _ => {
                // println!("zone {} has {neighb} neighbours", z.id);
            }
        }
    }
    if corners.len() == 4 {
        return Some(corners.iter().product());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day20_input_demo1.txt")),
            Some(20899048083289)
        );
        assert_eq!(get_answer(include_str!("../assets/day20_input.txt")), Some(60145080587029));
    }
}
