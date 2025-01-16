use std::{collections::HashSet, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 08 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day08_input_demo1.txt");
    let input = include_str!("../assets/day08_input.txt");

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
    let prog = input
        .lines()
        .map(|line| {
            let chunks = line
                .split([' ', '+'])
                .filter(|ch| !ch.is_empty())
                .collect::<Vec<_>>();
            let arg = chunks[1].parse::<isize>().unwrap();
            (chunks[0], arg)
        })
        .collect::<Vec<_>>();
    // dbg!(&prog);

    let mut ptr = 0;
    let mut acc = 0;
    let mut done = HashSet::new();
    loop {
        if done.contains(&ptr) {
            return Some(acc);
        }
        done.insert(ptr);
        match prog[ptr].0 {
            "acc" => {
                acc += prog[ptr].1;
                ptr += 1;
            }
            "jmp" => {
                ptr = (ptr as isize + prog[ptr].1) as usize;
            }
            "nop" => {
                ptr += 1;
            }
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day08_input_demo1.txt")),
            Some(5)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day08_input.txt")),
            Some(1487)
        );
    }
}
