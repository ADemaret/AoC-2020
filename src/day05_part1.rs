use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 05 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day05_input_demo1.txt");
    let input = include_str!("../assets/day05_input.txt");

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
   let v = input.lines().map(|line| {
        let mut min = 0;
        let mut max = 127;
        line.chars().take(7).for_each(|c| {            
            match c {
                'F' => max = ((max+min) as f32 /2.0).floor() as usize,
                'B' => min = ((max+min) as f32 /2.0).ceil() as usize,
                _ => panic!()
            }
            // println!("row {c} => {min} - {max}");
        });
        let row = if min == max {min} else {panic!()};
        let mut min = 0;
        let mut max = 7;
        line.chars().skip(7).take(3).for_each(|c| {            
            match c {
                'L' => max = ((max+min) as f32 /2.0).floor() as usize,
                'R' => min = ((max+min) as f32 /2.0).ceil() as usize,
                _ => panic!()
            }
            // println!("col {c} => {min} - {max}");
        });
        let col = if min == max {min} else {panic!()};
        row*8 + col
    }).collect::<Vec<usize>>();
    v.iter().max().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("FBFBBFFRLR"), Some(357));
        assert_eq!(get_answer("BFFFBBFRRR"), Some(567));
        assert_eq!(get_answer("FFFBBBFRRR"), Some(119));
        assert_eq!(get_answer("BBFFBBFRLL"), Some(820));
        assert_eq!(get_answer(include_str!("../assets/day05_input.txt")), Some(980));
    }
}
