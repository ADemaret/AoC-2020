use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 07 - Part 2 --");
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
    let mut rules = HashMap::new();
    input.lines().for_each(|line| {
        let chunks = line.split_whitespace().collect::<Vec<_>>();
        let mut bag;
        let mut nbr = 4;
        let mut content = Vec::new();
        while nbr < chunks.len() {
            let nbr_bag = chunks[nbr];
            bag = chunks[nbr + 1].to_string();
            if let Ok(nbr_bag) = nbr_bag.parse::<usize>() {
                bag.push(' ');
                bag.push_str(chunks[nbr + 2]);
                content.push((nbr_bag, bag));
            }
            nbr += 4;
        }
        bag = chunks[0].to_string();
        bag.push(' ');
        bag.push_str(chunks[1]);
        rules.insert(bag, content);
    });
    // dbg!(&rules);

    let mut total = 0;
    let mut queue = VecDeque::new();
    queue.push_back((1, "shiny gold"));
    while let Some(q) = queue.pop_front() {
        total += q.0;
        if let Some(content) = rules.get(q.1) {
            for c in content {
                queue.push_back((q.0 * c.0, c.1.as_str()));
            }
        }
    }
    total -= 1;

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day07_input_demo1.txt")),
            Some(32)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day07_input_demo2.txt")),
            Some(126)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day07_input.txt")),
            Some(34862)
        );
    }
}
