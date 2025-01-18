use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 15 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day15_input_demo1.txt");
    let input = include_str!("../assets/day15_input.txt");

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
    let mut last_spoken = 0;
    let mut game = input
        .split(',')
        .enumerate()
        .map(|(i, x)| {
            let val = x.parse::<usize>().unwrap();
            last_spoken = val;
            (val, (0, i + 1))
        })
        .collect::<HashMap<usize, (usize, usize)>>();
    let init_len = game.len();
    // dbg!(&game);

    let mut new_spoken = 0;
    for t in init_len..30000000 {
        if let Some(last) = game.get(&last_spoken) {
            if last.1 == t {
                // println!("just said it");
                if last.0 == 0 {
                    // println!("and was first time => 0");
                    new_spoken = 0;
                } else {
                    // println!("but also did at {}", last.0);
                    new_spoken = t - last.0;
                }
            } else {
                // println!("said it at {}", last.1);
                new_spoken = t - last.1;
            }
        } else {
            // println!("and was first time => 0");
            new_spoken = 0;
        }
        if let Some(spoken) = game.get(&new_spoken) {
            game.insert(new_spoken, (spoken.1, t + 1));
        } else {
            game.insert(new_spoken, (0, t + 1));
        }
        last_spoken = new_spoken;
        // println!("at turn {}, says {new_spoken}", t + 1);
        // dbg!(&game);
    }

    Some(new_spoken)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer(include_str!("../assets/day15_input_demo1.txt")),Some(175594));
        assert_eq!(get_answer("1,3,2"),Some(2578));
        assert_eq!(get_answer("2,1,3"),Some(3544142));
        assert_eq!(get_answer("1,2,3"),Some(261214));
        assert_eq!(get_answer("2,3,1"),Some(6895259));
        assert_eq!(get_answer("3,2,1"),Some(18));
        assert_eq!(get_answer("3,1,2"),Some(362));
        assert_eq!(get_answer(include_str!("../assets/day15_input.txt")), Some(37385));
    }
}
