use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 12 - Part 1 --");
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
    let mut dir = 0;

    input.lines().for_each(|line| {
        step(line, &mut pos, &mut dir);
    });
    Some(pos.0.abs() + pos.1.abs())
}

fn step(line: &str, pos: &mut (isize, isize), dir: &mut usize) {
    let directions = [(0, 1, 'E'), (1, 0, 'S'), (0, -1, 'W'), (-1, 0, 'N')];
    let (instr, val) = line.split_at(1);
    if let Ok(dist) = val.parse::<isize>() {
        match instr {
            "N" => {
                pos.0 -= dist;
            }
            "S" => {
                pos.0 += dist;
            }
            "W" => {
                pos.1 -= dist;
            }
            "E" => {
                pos.1 += dist;
            }
            "L" => {
                *dir = (*dir + (3 * dist as usize / 90)) % 4;
            }
            "R" => {
                *dir = (*dir + (dist as usize / 90)) % 4;
            }
            "F" => {
                pos.0 += directions[*dir].0 * dist;
                pos.1 += directions[*dir].1 * dist;
            }
            _ => panic!(),
        }
    } else {
        panic!();
    }
    // println!("{line} => {:?},{}", curpos, dir[*curdir].2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir() {
        let mut curpos = (0, 0);
        let mut curdir = 0;
        step("N10", &mut curpos, &mut curdir);
        assert_eq!(curpos, (-10, 0));
        step("S10", &mut curpos, &mut curdir);
        assert_eq!(curpos, (0, 0));
        step("W10", &mut curpos, &mut curdir);
        assert_eq!(curpos, (0, -10));
        step("E10", &mut curpos, &mut curdir);
        assert_eq!(curpos, (0, 0));
        step("F10", &mut curpos, &mut curdir);
        assert_eq!(curpos, (0, 10));
        step("R90", &mut curpos, &mut curdir);
        assert_eq!(curdir, 1); // S
        step("F10", &mut curpos, &mut curdir);
        assert_eq!(curpos, (10, 10));
        step("R90", &mut curpos, &mut curdir);
        assert_eq!(curdir, 2); // W
        step("F10", &mut curpos, &mut curdir);
        assert_eq!(curpos, (10, 0));
        step("R90", &mut curpos, &mut curdir);
        assert_eq!(curdir, 3); // N
        step("F10", &mut curpos, &mut curdir);
        assert_eq!(curpos, (0, 0));
        step("R90", &mut curpos, &mut curdir);
        assert_eq!(curdir, 0); // E
        step("L90", &mut curpos, &mut curdir);
        assert_eq!(curdir, 3); // N
        step("L90", &mut curpos, &mut curdir);
        assert_eq!(curdir, 2); // W
        step("L90", &mut curpos, &mut curdir);
        assert_eq!(curdir, 1); // S
        step("L90", &mut curpos, &mut curdir);
        assert_eq!(curdir, 0); // E
        step("R180", &mut curpos, &mut curdir);
        assert_eq!(curdir, 2); // W
    }

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day12_input_demo1.txt")),
            Some(25)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day12_input.txt")),
            Some(962)
        );
    }
}
