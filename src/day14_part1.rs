use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 14 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day14_input_demo1.txt");
    let input = include_str!("../assets/day14_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> Option<u64> {
    let mut memory: HashMap<usize, [char; 36]> = HashMap::new();
    let default_memory = ['0'; 36];
    let mut mask = ['0'; 36];
    input.lines().for_each(|line| {
        if line.starts_with("mask") {
            let m = line.chars().skip(7).collect::<Vec<char>>();
            mask = m.try_into().unwrap();
            // dbg!(&mask);
        } else if line.starts_with("mem") {
            let chunks = line.split(['[', ']', ' ']).collect::<Vec<_>>();
            let mem_id = chunks[1].parse::<usize>().unwrap();
            let val_str = format!("{:b}", chunks[4].parse::<usize>().unwrap());
            let val = val_str.chars().collect::<Vec<char>>();
            // dbg!(&val);

            let mut str = default_memory;
            for idx in 0..val.len() {
                str[36-val.len()+idx] = val[idx];
            }
            for idx in 0..36 {
                if mask[idx] != 'X' {
                    str[idx] = mask[idx];
                }
            }
            // dbg!(&str);
            memory.insert(mem_id, str);
        }
    });
    let mut result = 0;
    for m in memory {
        println!("{:?}",m);
        let binary_string: String = m.1.iter().collect();
        result += u64::from_str_radix(&binary_string, 2).unwrap();
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day14_input_demo1.txt")),
            Some(165)
        );
        assert_eq!(get_answer(include_str!("../assets/day14_input.txt")), None);
    }
}
