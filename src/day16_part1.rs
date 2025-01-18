use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 16 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day16_input_demo1.txt");
    let input = include_str!("../assets/day16_input.txt");

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
    let parts = input.split("\n\n").collect::<Vec<_>>();

    // ranges
    let rules = parts[0]
        .lines()
        .map(|line| {
            let values = line
                .split([' ', '-'])
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<_>>();
            values
                .chunks(2)
                .map(|chunk| (chunk[0], chunk[1]))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<_>>();
    // dbg!(&rules);

    // my ticket

    // other tickets
    let tickets = parts[2]
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<usize>>>();
    // dbg!(&tickets);

    Some(
        tickets
            .iter()
            .map(|fields| {
                fields
                    .iter()
                    .map(|f| if not_valid(&rules, *f) { *f } else { 0 })
                    .sum::<usize>()
            })
            .sum::<usize>(),
    )
}

fn not_valid(rules: &Vec<Vec<(usize, usize)>>, f: usize) -> bool {
    let mut valid = false;
    for range in rules {
        for (min, max) in range {
            if f >= *min && f <= *max {
                valid = true;
                break;
            }
        }
    }
    // println!("{f} is {valid}");
    !valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day16_input_demo1.txt")),
            Some(71)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day16_input.txt")),
            Some(26988)
        );
    }
}
