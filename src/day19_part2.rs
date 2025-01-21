use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
// use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 19 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day19_input_demo1.txt");
    let input = include_str!("../assets/day19_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

#[derive(Debug, Default, Clone)]
struct Rule {
    vals: Vec<String>,
    or_vals: Vec<String>,
    vec: Vec<String>,
}

fn get_answer(input: &str) -> Option<usize> {
    let (r, msgs) = input.split_once("\n\n").unwrap();

    // parse rules
    let mut rules = r
        .lines()
        .map(|line| {
            let chunks = line
                .split([':', ' ', '\"'])
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>();
            // println!("{line} => {:?}", chunks);
            if line.contains("|") {
                let pos = chunks.iter().position(|&x| x == "|").unwrap();
                let mut vals = Vec::new();
                let mut or_vals = Vec::new();
                for ch in chunks.iter().skip(1).take(pos - 1) {
                    vals.push(ch.to_string());
                }
                for ch in chunks.iter().skip(pos + 1) {
                    or_vals.push(ch.to_string());
                }
                (
                    chunks[0].to_string(),
                    Rule {
                        vals,
                        or_vals,
                        vec: Vec::new(),
                    },
                )
            } else if ["a", "b"].contains(&chunks[1]) {
                (
                    chunks[0].to_string(),
                    Rule {
                        vals: vec![],
                        or_vals: vec![],
                        vec: vec![chunks[1].to_string()],
                    },
                )
            } else {
                (
                    chunks[0].to_string(),
                    Rule {
                        vals: chunks
                            .iter()
                            .skip(1)
                            .map(|ch| ch.to_string())
                            .collect::<Vec<_>>(),
                        or_vals: vec![],
                        vec: vec![],
                    },
                )
            }
        })
        .collect::<HashMap<String, Rule>>();
    // dbg!(&rules);

    let mut from_42 = HashMap::new();
    dive_in(&mut rules, "42".to_string());
    let rules2 = rules.clone();
    for (i, v) in (rules2.get("42").unwrap().vec).iter().enumerate() {
        // println!("{i} {v}");
        from_42.insert(v, i);
    }
    let mut from_31 = HashMap::new();
    dive_in(&mut rules, "31".to_string());
    for (i, v) in (rules.get("31").unwrap().vec).iter().enumerate() {
        // println!("{i} {v}");
        from_31.insert(v, i);
    }

    let mut all_vec_from = Vec::new();
    'out: for msg in msgs.lines().collect::<Vec<&str>>() {
        let mut vec_from = Vec::new();
        // println!("{msg}");
        let mut parts = Vec::new();
        for i in 0..msg.len() / 8 {
            let a_part = msg[i * 8..8 + i * 8].to_string();
            parts.push(a_part.clone());
            let id42 = from_42.get(&a_part);
            let id31 = from_31.get(&a_part);
            if id42.is_some() && id31.is_some() {
                panic!("both 42 and 31");
            }
            if id42.is_some() {
                // print!("42 ");
                vec_from.push(42);
            }
            if id31.is_some() {
                // print!("31 ");
                vec_from.push(31);
            }
            // println!("{a_part}");
            if id42.is_none() && id31.is_none() {
                // print!("XX ");
                continue 'out;
            }
        }
        all_vec_from.push(vec_from);
    }
    // for debug purpose only
    // all_vec_from.sort();

    let mut result = 0;
    for vf in all_vec_from {
        // for item in &vf {
        //     print!("{item} ");
        // }
        // msgs must start with 42
        // msgs must end with 31
        let start_same = vf.iter().take_while(|&x| *x == 42).count();
        let end_same = vf.iter().rev().take_while(|&x| *x == 31).count();
        if start_same > 0
            && end_same > 0
            && start_same + end_same == vf.len()
            && start_same > end_same
        {
            // println!(" => ok");
            result += 1;
            continue;
        } else {
            // println!(" => not ok");
        }
    }

    Some(result)
}

fn dive_in(rules: &mut HashMap<String, Rule>, start_rule: String) -> Vec<String> {
    let mut big_vec = Vec::new();

    // rule must be valid
    if let Some(rule) = rules.clone().get(&start_rule) {
        // already known
        if !rule.vec.is_empty() {
            return rule.vec.clone();
        }

        // concat vals
        let mut vec = Vec::new();
        let mut vec_copy = Vec::new();
        for i in 0..rule.vals.len() {
            let bfs = dive_in(rules, rule.vals[i].clone());
            if vec_copy.is_empty() {
                for b in &bfs {
                    vec.push(b.to_string());
                }
                vec_copy = vec.clone();
            } else {
                vec = Vec::new();
                for v in &vec_copy {
                    for b in &bfs {
                        vec.push(format!("{v}{b}").to_string());
                    }
                }
                vec_copy = vec.clone();
            }
        }
        let str_val = vec.clone();
        // dbg!(&str_val);

        // concat or_vals
        let mut vec = Vec::new();
        let mut vec_copy = Vec::new();
        for i in 0..rule.or_vals.len() {
            let bfs = dive_in(rules, rule.or_vals[i].clone());
            if vec_copy.is_empty() {
                for b in &bfs {
                    vec.push(b.to_string());
                }
                vec_copy = vec.clone();
            } else {
                vec = Vec::new();
                for v in &vec_copy {
                    for b in &bfs {
                        vec.push(format!("{v}{b}").to_string());
                    }
                }
                vec_copy = vec.clone();
            }
        }
        let mut str_val2 = vec.clone();
        // dbg!(&str_val2);

        big_vec = str_val;
        big_vec.append(&mut str_val2);

        rules.insert(
            start_rule.clone(),
            Rule {
                vals: rule.vals.clone(),
                or_vals: rule.or_vals.clone(),
                vec: big_vec.clone(),
            },
        );
    } else {
        // panic!("invalid rule");
    }
    big_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day19_input.txt")),
            Some(407)
        );
    }
}
