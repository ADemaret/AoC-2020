use std::{collections::VecDeque, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

const DEBUG: bool = false;

pub fn main() {
    println!("-- Advent of Code - Day 22 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day22_input_demo1.txt");
    let input = include_str!("../assets/day22_input.txt");

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
    let (p1, p2) = input.split_once("\n\n").unwrap();
    let mut deck1 = VecDeque::new();
    p1.lines().skip(1).for_each(|v| {
        let val = v.parse::<usize>().unwrap();
        deck1.push_back(val);
    });
    let mut deck2 = VecDeque::new();
    p2.lines().skip(1).for_each(|v| {
        let val = v.parse::<usize>().unwrap();
        deck2.push_back(val);
    });

    let mut result = 0;
    let mut round = 1;
    loop {
        if DEBUG {
            println!("-- Round {round} --");
            println!("Player 1's deck : {:?}", deck1);
            println!("Player 2's deck : {:?}", deck2);
        }
        let c1 = deck1.pop_front().unwrap();
        if DEBUG {
            println!("Player 1 plays: {}", c1);
        }
        let c2 = deck2.pop_front().unwrap();
        if DEBUG {
            println!("Player 2 plays: {}", c2);
        }
        if c1 > c2 {
            if DEBUG {
                println!("Player 1 wins the round!");
            }
            deck1.push_back(c1);
            deck1.push_back(c2);
        } else {
            if DEBUG {
                println!("Player 2 wins the round!");
            }
            deck2.push_back(c2);
            deck2.push_back(c1);
        }
        if deck1.is_empty() || deck2.is_empty() {
            if c1 > c2 {
                return Some(get_deck_score(deck1));
            } else {
                return Some(get_deck_score(deck1));
            }
        }
        round += 1;
    }
}

fn get_deck_score(deck: VecDeque<usize>) -> usize {
    let mut result = 0;
    let mut max = deck.len() + 1;
    for v in deck {
        if DEBUG {
            println!("{} x {}", max - 1, v);
        }
        result += (max - 1) * v;
        max -= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day22_input_demo1.txt")),
            Some(306)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day22_input.txt")),
            Some(32083)
        );
    }
}
