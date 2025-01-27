use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 23 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day23_input_demo1.txt");
    let input = include_str!("../assets/day23_input.txt");

    if let Some(answer) = get_answer(input, 100) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

type Cup = usize;

fn get_answer(input: &str, cycles: usize) -> Option<String> {
    let mut cups: Vec<Cup> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as Cup)
        .collect();
    let len = cups.len();

    let mut cur_idx = 0;
    let mut cur_val = cups[cur_idx];

    for _ in 1..=cycles {
        // println!("-- move {m} --");
        // print_cups(&cups, cur_idx);

        // pick
        let pick: Vec<Cup> = cups
            .iter()
            .cycle()
            .skip(cur_idx + 1)
            .take(3)
            .copied()
            .collect();
        let mut pick_idx = [
            (cur_idx + 1) % len,
            (cur_idx + 2) % len,
            (cur_idx + 3) % len,
        ];
        pick_idx.sort();
        pick_idx.reverse();
        for p in pick_idx {
            cups.remove(p);
        }
        // print_pick(&pick);

        // dest
        let mut dest = find_dest(&cups, cur_val - 1);
        // println!("destination: {}", cups[dest]);
        // println!();

        // reinsert picks
        dest = (dest + 1) % len;
        cups.insert(dest, pick[2]);
        cups.insert(dest, pick[1]);
        cups.insert(dest, pick[0]);

        // rotate so that current is at the same position
        let new_cur_idx = cups.iter().position(|&cup| cup == cur_val).unwrap();
        for _ in 0..new_cur_idx - cur_idx {
            let first = cups.remove(0);
            cups.push(first);
        }

        cur_idx = (cur_idx + 1) % len;
        cur_val = cups[cur_idx];
    }

    let pos = cups.iter().position(|&cup| cup == 1).unwrap();
    let vec_result: Vec<String> = cups
        .iter()
        .cycle()
        .skip(pos + 1)
        .take(len - 1)
        .map(|&num| num.to_string())
        .collect();
    let str_result = vec_result.join("");

    Some(str_result)
}

fn find_dest(cups: &[Cup], val: usize) -> usize {
    let mut val = val;
    let min = cups.iter().min().unwrap();
    let max = cups.iter().max().unwrap();
    loop {
        if let Some(pos) = cups.iter().position(|&cup| cup == val) {
            return pos;
        }
        if val < *min {
            val = *max + 1;
        }
        val -= 1;
    }
}

fn _print_pick(pick: &[Cup]) {
    let picks: Vec<String> = pick.iter().map(|&num| num.to_string()).collect();
    let joined: String = picks.join(", ");
    println!("pick up: {}", joined);
}

fn _print_cups(cups: &[Cup], cur_idx: usize) {
    print!("cups: ");
    for (i, c) in cups.iter().enumerate() {
        if i == cur_idx {
            print!("({}) ", c);
        } else {
            print!("{}  ", c);
        }
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day23_input_demo1.txt"), 10),
            Some("92658374".to_string())
        );
        assert_eq!(
            get_answer(include_str!("../assets/day23_input_demo1.txt"), 100),
            Some("67384529".to_string())
        );
        assert_eq!(
            get_answer(include_str!("../assets/day23_input.txt")),
            Some("76952348".to_string())
        );
    }
}
