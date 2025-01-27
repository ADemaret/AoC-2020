use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

const SIZE: usize = 1_000_000;
const CYCLES: usize = 10_000_000;
const DEBUG: bool = false;

pub fn main() {
    println!("-- Advent of Code - Day 23 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day23_input_demo1.txt");
    let input = include_str!("../assets/day23_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

type Cup = usize;

fn get_answer(input: &str) -> Option<u64> {
    // if we have the cups 8,4,... => Next[8] = 4
    let mut next: Vec<Cup> = vec![0; SIZE + 1];

    // get input
    let input_vec = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    for i in 0..input_vec.len() - 1 {
        next[input_vec[i]] = input_vec[i + 1];
    }
    next[input_vec[input_vec.len() - 1]] = input_vec.len() + 1;
    for i in input_vec.len() + 1..SIZE {
        next[i] = i + 1;
    }
    next[SIZE] = input_vec[0];
    // if DEBUG {
    //     for (i, n) in next.iter().enumerate() {
    //         println!("{i} est suivi de {n}");
    //     }
    // }

    let mut current = input_vec[0];

    for m in 1..=CYCLES {
        if DEBUG {
            println!("-- move {m} --");
            print_cups(&next, current);
        }

        // pick
        let mut pick = [0, 0, 0];
        pick[0] = next[current];
        pick[1] = next[pick[0]];
        pick[2] = next[pick[1]];
        if DEBUG {
            print_pick(&pick);
        }
        // let old_current_next = next[current];
        next[current] = next[pick[2]];

        // dest
        let dest = find_dest(&pick, current - 1);
        if DEBUG {
            println!("destination: {}", dest);
            println!();
        }

        // reinsert picks
        let old_dest_next = next[dest];
        next[dest] = pick[0];
        next[pick[2]] = old_dest_next;

        current = next[current];
    }

    let cup1 = next[1];
    let cup2 = next[cup1];
    let result: u64 = cup1 as u64 * cup2 as u64;
    Some(result)
}

fn find_dest(pick: &[Cup; 3], val: usize) -> usize {
    let min = 1;
    let max = SIZE;
    let mut dest = val;
    loop {
        if dest < min {
            dest = max;
        }
        if !pick.contains(&dest) {
            return dest;
        }
        dest -= 1;
    }
}

fn print_pick(pick: &[Cup]) {
    if DEBUG {
        let picks: Vec<String> = pick.iter().map(|num| num.to_string()).collect();
        let joined: String = picks.join(", ");
        println!("pick up: {}", joined);
    }
}

fn print_cups(next: &[Cup], current: usize) {
    if DEBUG {
        let mut n = current;
        print!("cups: ");
        print!("({}) ", n);
        for _ in 0..next.len() - 2 {
            n = next[n];
            print!("{}  ", n);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day23_input_demo1.txt")),
            Some(149245887792)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day23_input.txt")),
            Some(72772522064)
        );
    }
}
