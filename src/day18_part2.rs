use std::{fmt, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 18 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day18_input_demo1.txt");
    // let input = "1 + (2 * 3) + (4 * (5 + 6))";
    // let input = "4 * (3 + 9) + 9 + 4 + 8 + 4";
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
    let mut calc = Vec::new();
    calc.push(Elem {
        op: Op::Open,
        val: 0,
    });
    line.chars().filter(|&c| c != ' ').for_each(|c| {
        if let Some(val) = c.to_digit(10) {
            calc.push(Elem {
                op: Op::Val,
                val: val as isize,
            })
        } else {
            calc.push(match c {
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
            });
        }
    });
    calc.push(Elem {
        op: Op::Close,
        val: 0,
    });

    loop {
        loop {
            let mut done_subs = false;
            loop {
                if !find_and_process_add(&mut calc) {
                    break;
                } else {
                    // print(&calc);
                }
            }
            loop {
                if !find_and_process_subs(&mut calc) {
                    break;
                } else {
                    // print(&calc);
                    done_subs = true;
                }
            }
            if !done_subs {
                break;
            }
        }
        if !find_and_process_isolated_mul(&mut calc) {
            break;
        } else {
            // print(&calc);
        }

        if calc.len() == 1 {
            break;
        }
    }

    calc[0].val
}

fn find_and_process_add(calc: &mut Vec<Elem>) -> bool {
    let mut something_changed = false;
    let mut to_remove = Vec::new();
    for idx in 1..calc.len() - 1 {
        if calc[idx - 1].op == Op::Val && calc[idx].op == Op::Add && calc[idx + 1].op == Op::Val {
            calc[idx - 1].val += calc[idx + 1].val;
            to_remove.push(idx);
            to_remove.push(idx + 1);
            something_changed = true;
            break;
        }
    }
    if !to_remove.is_empty() {
        clean_calc(calc, &mut to_remove);
    } else {
        return something_changed;
    }
    something_changed
}

fn find_and_process_subs(calc: &mut Vec<Elem>) -> bool {
    let mut something_changed = false;
    let mut to_remove = Vec::new();
    for idx in 1..calc.len() - 1 {
        if calc[idx - 1].op == Op::Open && calc[idx].op == Op::Val && calc[idx + 1].op == Op::Close
        {
            to_remove.push(idx - 1);
            to_remove.push(idx + 1);
            something_changed = true;
            break;
        }
    }
    if !to_remove.is_empty() {
        clean_calc(calc, &mut to_remove);
    } else {
        return something_changed;
    }
    something_changed
}

fn find_and_process_isolated_mul(calc: &mut Vec<Elem>) -> bool {
    let mut something_changed = false;
    if calc.len() == 1 {
        return false;
    }
    let mut idx = 0;
    'main: loop {
        // print(calc);
        // println!("idx {idx}");
        if calc[idx].op == Op::Open {
            for idx2 in idx + 1..calc.len() {
                // println!("idx2 {idx2}");
                if calc[idx2].op == Op::Open {
                    idx = idx2;
                    continue 'main;
                }
                if calc[idx2].op == Op::Close {
                    // println!("processing");
                    let mut to_remove = Vec::new();
                    for idx3 in idx + 1..idx2 - 1 {
                        if calc[idx3 - 1].op == Op::Val
                            && calc[idx3].op == Op::Mul
                            && calc[idx3 + 1].op == Op::Val
                        {
                            calc[idx3 - 1].val *= calc[idx3 + 1].val;
                            to_remove.push(idx3);
                            to_remove.push(idx3 + 1);
                            something_changed = true;
                            break;
                        }
                    }
                    if !to_remove.is_empty() {
                        clean_calc(calc, &mut to_remove);
                        find_and_process_subs(calc);
                    }
                    return something_changed;
                }
            }
        }
        idx += 1;
        if idx >= calc.len() - 1 {
            return something_changed;
        }
    }
}

fn clean_calc(calc: &mut Vec<Elem>, to_remove: &mut Vec<usize>) {
    to_remove.sort();
    to_remove.reverse();
    // dbg!(&to_remove);
    for idx in to_remove {
        calc.remove(*idx);
    }
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
            Some(231)
        );
        assert_eq!(get_answer("1 + (2 * 3) + (4 * (5 + 6))"), Some(51));
        assert_eq!(get_answer("2 * 3 + (4 * 5)"), Some(46));
        assert_eq!(get_answer("5 + (8 * 3 + 9 + 3 * 4 * 3)"), Some(1445));
        assert_eq!(
            get_answer("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            Some(669060)
        );
        assert_eq!(
            get_answer("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            Some(23340)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day18_input.txt")),
            Some(545115449981968)
        );
    }
}
