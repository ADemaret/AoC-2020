use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 10 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day10_input_demo2.txt");
    let input = include_str!("../assets/day10_input.txt");

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
    let mut adapters = input
        .lines()
        .map(|line| (line.parse::<usize>().unwrap(), false))
        .collect::<Vec<_>>();

    adapters.sort();
    
    // tag mandatory adapters
    let len = adapters.len();
    for idx in 0..len - 1 {
        if adapters[idx + 1].0 - adapters[idx].0 == 3 {
            adapters[idx].1 = true;
            adapters[idx + 1].1 = true;
        }
    }
    adapters[len - 1].1 = true;

    // for a in &adapters {
    //     println!("{:?}", a);
    // }

    // measure gaps
    let mut v_true = Vec::new();
    let mut idx = 0;
    loop {
        if (idx == 0 || adapters[idx - 1].1) && !adapters[idx].1 {
            let min = idx;
            loop {
                idx += 1;
                if idx == adapters.len() || adapters[idx].1 {
                    break;
                }
            }
            let max = idx;
            match max - min {
                1 => v_true.push(2), // a gap of 1 = 2 possibilities
                2 => v_true.push(4), // a gap of 2 = 4 possibilities
                3 => v_true.push(7), // a gap of 3 = 7 possibilities
                _ => {}
            }
        }
        idx += 1;
        if idx >= adapters.len() {
            break;
        }
    }
    // dbg!(&v_true);

    Some(v_true.iter().product())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day10_input_demo1.txt")),
            Some(8)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day10_input_demo2.txt")),
            Some(19208)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day10_input.txt")),
            Some(32396521357312)
        );
    }
}
