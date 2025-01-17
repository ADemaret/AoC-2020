use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 13 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day13_input_demo1.txt");
    let input = include_str!("../assets/day13_input.txt");

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
    let (_, str_bus) = input.split_once("\n").unwrap();
    let buses = str_bus
        .split(",")
        .enumerate()
        .filter(|(_, b)| *b != "x")
        .map(|(idx, b)| {
            let bus = b.parse::<usize>().unwrap();
            // avoid negative bi
            let mut b = bus;
            while b < idx {
                b += bus;
            }
            let bi = b - idx;
            (bi, bus)
        })
        .collect::<Vec<_>>();

    // dbg!(&buses);

    Some(chinese_remainder(&buses))
}

fn chinese_remainder(buses: &Vec<(usize, usize)>) -> usize {
    let mut result = 0;
    let big_n: usize = buses.iter().map(|b| b.1).product();
    for b in buses {
        let bi = b.0;
        let big_ni = big_n / b.1;
        let mut xi = 1;
        loop {
            if (xi * big_ni) % b.1 == 1 {
                break;
            }
            xi += 1;
        }
        // println!("bi={}, Ni={}, xi={}", bi, big_ni, xi);
        result += bi * big_ni * xi;
    }
    result %= big_n;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day13_input_demo1.txt")),
            Some(1068781)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day13_input.txt")),
            Some(500033211739354)
        );
    }
}
