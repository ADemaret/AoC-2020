use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 07 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day07_input_demo1.txt");
    let input = include_str!("../assets/day07_input.txt");

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
    let mut rules = Vec::new();
    input.lines().for_each(|line| {
        let chunks = line.split_whitespace().collect::<Vec<_>>();
        let mut bag;
        let mut nbr = 4;
        let mut content = Vec::new();
        while nbr < chunks.len() {
            let nbr_bag = chunks[nbr];
            bag = chunks[nbr + 1].to_string();
            if nbr_bag.parse::<usize>().is_ok() {
                bag.push(' ');
                bag.push_str(chunks[nbr + 2]);
                content.push(bag);
            }
            nbr += 4;
        }
        bag = chunks[0].to_string();
        bag.push(' ');
        bag.push_str(chunks[1]);
        rules.push((bag, content));
    });
    // dbg!(&rules);

    let mut list = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back("shiny gold");
    while let Some(q) = queue.pop_front() {
        for r in &rules {
            if r.1.contains(&q.to_string()) {
                list.insert(&r.0);
                queue.push_back(r.0.as_str());
            }
        }
    }
    Some(list.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day07_input_demo1.txt")),
            Some(4)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day07_input.txt")),
            Some(124)
        );
    }
}
