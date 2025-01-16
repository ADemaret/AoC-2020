use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 09 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day09_input_demo1.txt");
    let input = include_str!("../assets/day09_input.txt");

    if let Some(answer) = get_answer(input, 25) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str, preamble: usize) -> Option<usize> {
    let mut step1_value = 0;
    let mut vals = Vec::new();
    'lines: for (i, line) in input.lines().enumerate() {
        if let Ok(val) = line.parse::<usize>() {
            vals.push(val);
            if i >= preamble {
                for a in i - preamble..vals.len() {
                    for b in i - preamble..vals.len() {
                        if vals[a] + vals[b] == val {
                            // println!("{val} is {a} + {b}");
                            continue 'lines;
                        }
                    }
                }
                step1_value = val;
                break;
            }
        }
    }

    // dbg!(step1_value);

    let mut start = 0;
    let mut idx = start;
    let mut sum = 0;
    loop {
        sum += vals[idx];
        match sum.cmp(&step1_value) {
            std::cmp::Ordering::Equal => {
                let min = vals.iter().skip(start).take(idx - start).min().unwrap();
                let max = vals.iter().skip(start).take(idx - start).max().unwrap();
                // println!("{}, {}",min, max);
                return Some(min + max);
            }
            std::cmp::Ordering::Greater => {
                start += 1;
                idx = start;
                sum = 0;
            }
            std::cmp::Ordering::Less => {
                idx += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day09_input_demo1.txt"), 5),
            Some(62)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day09_input.txt"), 25),
            Some(5388976)
        );
    }
}
