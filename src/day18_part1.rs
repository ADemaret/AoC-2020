use std::{fmt, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 18 - Part 1 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day18_input_demo1.txt");
    // let input = "1 + (2 * 3) + (4 * (5 + 6))";
    let input = include_str!("../assets/day18_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
    Open,
    Close,
    Val,
}

#[derive(Debug)]
struct Elem {
    op: Op,
    val: isize,
}

impl fmt::Display for Elem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self.op {
            Op::Add => "+".to_string(),
            Op::Mul => "*".to_string(),
            Op::Open => "(".to_string(),
            Op::Close => ")".to_string(),
            Op::Val => self.val.to_string(),
        };
        write!(f, "{} ", str)
    }
}

fn get_answer(input: &str) -> Option<isize> {
    Some(input.lines().map(do_it).sum())
}

fn do_it(line: &str) -> isize {
    let mut calc = line
        .chars()
        .filter(|&c| c != ' ')
        .map(|c| {
            if let Some(val) = c.to_digit(10) {
                Elem {
                    op: Op::Val,
                    val: val as isize,
                }
            } else {
                match c {
                    '+' => Elem {
                        op: Op::Add,
                        val: 0,
                    },
                    '*' => Elem {
                        op: Op::Mul,
                        val: 0,
                    },
                    '(' => Elem {
                        op: Op::Open,
                        val: 0,
                    },
                    ')' => Elem {
                        op: Op::Close,
                        val: 0,
                    },
                    _ => panic!(),
                }
            }
        })
        .collect::<Vec<_>>();
    loop {
        // print(&calc);
        let mut to_remove = Vec::new();
        for idx in 1..calc.len() - 1 {
            if calc[idx - 1].op == Op::Val && calc[idx].op == Op::Add && calc[idx + 1].op == Op::Val
            {
                calc[idx - 1].val += calc[idx + 1].val;
                to_remove.push(idx);
                to_remove.push(idx + 1);
                break;
            } else if calc[idx - 1].op == Op::Val
                && calc[idx].op == Op::Mul
                && calc[idx + 1].op == Op::Val
            {
                calc[idx - 1].val *= calc[idx + 1].val;
                to_remove.push(idx);
                to_remove.push(idx + 1);
                break;
            } else if calc[idx - 1].op == Op::Open
                && calc[idx].op == Op::Val
                && calc[idx + 1].op == Op::Close
            {
                to_remove.push(idx - 1);
                to_remove.push(idx + 1);
                break;
            }
        }
        if !to_remove.is_empty() {
            to_remove.sort();
            to_remove.reverse();
            // dbg!(&to_remove);
            for idx in &to_remove {
                calc.remove(*idx);
            }
        } else {
            break;
        }
    }
    // print(&calc);

    calc[0].val
}

fn _print(calc: &Vec<Elem>) {
    for el in calc {
        print!("{}", el);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day18_input_demo1.txt")),
            Some(71)
        );
        assert_eq!(get_answer("1 + (2 * 3) + (4 * (5 + 6))"),Some(51));
        assert_eq!(get_answer("2 * 3 + (4 * 5)"),Some(26));
        assert_eq!(get_answer("5 + (8 * 3 + 9 + 3 * 4 * 3)"),Some(437));
        assert_eq!(get_answer("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),Some(12240));
        assert_eq!(get_answer("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),Some(13632));
        assert_eq!(get_answer(include_str!("../assets/day18_input.txt")), Some(14006719520523));
    }
}
