use std::{
    collections::HashMap,time::Instant,
};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 14 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day14_input_demo2.txt");
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

fn get_answer(input: &str) -> Option<usize> {
    let mut full_memory = HashMap::new();
    let default_memory = ['0'; 36];
    let mut mask = ['0'; 36];
    input.lines().for_each(|line| {
        if line.starts_with("mask") {
            let m = line.chars().skip(7).collect::<Vec<char>>();
            mask = m.try_into().unwrap();
            // dbg!(&mask);
        } else if line.starts_with("mem") {
            // get input values
            let chunks = line.split(['[', ']', ' ']).collect::<Vec<_>>();
            let mem_id_str = format!("{:b}", chunks[1].parse::<usize>().unwrap());
            let mem_id = mem_id_str.chars().collect::<Vec<char>>();
            let mem_value = chunks[4].parse::<usize>().unwrap();

            // set binary mem_id
            let mut str = default_memory;
            for idx in 0..mem_id.len() {
                str[36 - mem_id.len() + idx] = mem_id[idx];
            }

            // apply mask
            for idx in 0..36 {
                match mask[idx] {
                    '0' => {}
                    '1' => str[idx] = '1',
                    'X' => str[idx] = 'X',
                    _ => panic!(),
                }
            }

            // => str is the mem_id with wildcards

            // find all mem_id's
            let mut vecs: Vec<[char; 36]> = Vec::new();
            vecs.push(default_memory);
            for idx in 0..36 {
                if str[idx] != 'X' {
                    for v in &mut vecs {
                        v[idx] = str[idx];
                    }
                } else {
                    let mut new_vecs = vecs.clone();
                    for v in &mut vecs {
                        v[idx] = '1';
                    }
                    for v in &mut new_vecs {
                        v[idx] = '0';
                    }
                    vecs.append(&mut new_vecs);
                }
            }
            // dbg!(&vecs);

            // write in memory
            for v in vecs {
                let binary_string: String = v.iter().collect();
                full_memory.insert(u64::from_str_radix(&binary_string, 2).unwrap(), mem_value);
            }
        }
    });
    Some(full_memory.iter().map(|x| *x.1).sum::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day14_input_demo2.txt")),
            Some(208)
        );
        assert_eq!(get_answer(include_str!("../assets/day14_input.txt")), Some(3817372618036));
    }
}
