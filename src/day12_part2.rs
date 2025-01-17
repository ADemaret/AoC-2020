use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 12 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day12_input_demo1.txt");
    let input = include_str!("../assets/day12_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> Option<isize> {
    let mut pos = (0, 0);
    let mut wp = (-1, 10);

    input.lines().for_each(|line| {
        step(line, &mut pos, &mut wp);
    });
    Some(pos.0.abs() + pos.1.abs())
}

fn step(line: &str, pos: &mut (isize, isize), wp: &mut (isize, isize)) {
    let (instr, val) = line.split_at(1);
    if let Ok(dist) = val.parse::<isize>() {
        match instr {
            "N" => {
                wp.0 -= dist;
            }
            "S" => {
                wp.0 += dist;
            }
            "W" => {
                wp.1 -= dist;
            }
            "E" => {
                wp.1 += dist;
            }
            "L" => {
                let dl = wp.0 - pos.0;
                let dc = wp.1 - pos.1;
                match dist {
                    90 => {
                        wp.0 = wp.0 - dc - dl;
                        wp.1 = wp.1 - dc + dl;
                    }
                    180 => {
                        wp.0 -= 2 * dl;
                        wp.1 -= 2 * dc;
                    }
                    270 => {
                        wp.0 = wp.0 + dc - dl;
                        wp.1 = wp.1 - dc - dl;
                    }
                    _ => panic!(),
                }
            }
            "R" => {
                let dl = wp.0 - pos.0;
                let dc = wp.1 - pos.1;
                match dist {
                    90 => {
                        wp.0 = wp.0 + dc - dl;
                        wp.1 = wp.1 - dc - dl;
                    }
                    180 => {
                        wp.0 -= 2 * dl;
                        wp.1 -= 2 * dc;
                    }
                    270 => {
                        wp.0 = wp.0 - dc - dl;
                        wp.1 = wp.1 - dc + dl;
                    }
                    _ => panic!(),
                }
            }
            "F" => {
                let dl = wp.0 - pos.0;
                let dc = wp.1 - pos.1;
                pos.0 += dl * dist;
                pos.1 += dc * dist;
                wp.0 = pos.0 + dl;
                wp.1 = pos.1 + dc;
            }
            _ => panic!(),
        }
    } else {
        panic!();
    }
    // println!("{line} => {:?},{:?}", curpos, wp);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir() {
        let mut pos = (0, 0);
        let mut wp = (-5, 10);
        step("R90", &mut pos, &mut wp);
        assert_eq!(wp, (10, 5));
        step("R90", &mut pos, &mut wp);
        assert_eq!(wp, (5, -10));
        step("R90", &mut pos, &mut wp);
        assert_eq!(wp, (-10, -5));
        step("R90", &mut pos, &mut wp);
        assert_eq!(wp, (-5, 10));
        let mut pos = (2, 2);
        let mut wp = (-3, 12);
        step("R90", &mut pos, &mut wp);
        assert_eq!(wp, (12, 7));
        step("R90", &mut pos, &mut wp);
        assert_eq!(wp, (7, -8));
        step("R90", &mut pos, &mut wp);
        assert_eq!(wp, (-8, -3));
        step("R90", &mut pos, &mut wp);
        assert_eq!(wp, (-3, 12));
    }

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day12_input_demo1.txt")),
            Some(286)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day12_input.txt")),
            Some(56135)
        );
    }
}
