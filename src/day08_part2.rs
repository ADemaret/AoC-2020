use std::{collections::HashSet, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 08 - Part 2 --");
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
    let mut prog = input
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

    for idx in 0..prog.len() {
        if prog[idx].0 == "jmp" {
            prog[idx].0 = "nop";
            if let Some(result) = run_prog(&prog) {
                return Some(result);
            }
            prog[idx].0 = "jmp";
        }
    }
    for idx in 0..prog.len() {
        if prog[idx].0 == "nop" {
            prog[idx].0 = "jmp";
            if let Some(result) = run_prog(&prog) {
                return Some(result);
            }
            prog[idx].0 = "nop";
        }
    }

    None
}

fn run_prog(prog: &[(&str, isize)]) -> Option<isize> {
    let mut ptr = 0;
    let mut acc = 0;
    let mut done = HashSet::new();
    loop {
        if done.contains(&ptr) {
            return None;
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
        if ptr >= prog.len() {
            return Some(acc);
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
            Some(8)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day08_input.txt")),
            Some(1607)
        );
    }
}
