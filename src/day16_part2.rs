use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 16 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day16_input_demo2.txt");
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
    let my_ticket = parts[1]
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<usize>>>();
    // dbg!(&my_ticket);

    // other tickets
    let mut tickets = parts[2]
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<usize>>>();
    // dbg!(&tickets);

    // remove invalid tickets
    let mut tickets_to_remove = Vec::new();
    let invalids = tickets
        .iter()
        .map(|fields| {
            fields
                .iter()
                .map(|f| not_valid_all(&rules, *f))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for (i, inv) in invalids.iter().enumerate() {
        if inv.contains(&(true)) {
            tickets_to_remove.push(i);
        }
    }
    tickets_to_remove.reverse();
    for t in tickets_to_remove {
        tickets.remove(t);
    }
    // dbg!(&tickets);

    // fields order
    let mut order = Vec::new();
    let poss = (0..rules.len()).collect::<Vec<usize>>();
    for _ in 0..rules.len() {
        order.push(poss.clone());
    }
    for fields in tickets {
        for (i, f) in fields.iter().enumerate() {
            for (r, rule) in rules.iter().enumerate() {
                if not_valid_one(rule, *f) {
                    // println!("field {i} can't be {r}");
                    order[i].retain(|&x| x != r);
                }
            }
        }
    }
    // dbg!(&order);

    // remove multiple possibilities
    let mut not_set = poss;
    while !not_set.is_empty() {
        let mut idx = 0;
        for (i, zz) in order.iter().enumerate() {
            if zz.len() == 1 && not_set.contains(&zz[0]) {
                idx = i;
            }
        }
        let set = order[idx][0];
        for o in &mut order {
            if o.len() > 1 {
                o.retain(|&x| x != set);
            }
        }
        not_set.retain(|&x| x != set);
        // dbg!(&not_set);
        // dbg!(&order);
    }
    // dbg!(&order);

    let mut departure = Vec::new();
    for idx in 0..6 {
        for (o, oo) in order.iter().enumerate() {
            if oo[0] == idx {
                departure.push(my_ticket[0][o]);
                break;
            }
        }
    }

    Some(departure.iter().product())
}

fn not_valid_all(rules: &Vec<Vec<(usize, usize)>>, f: usize) -> bool {
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

fn not_valid_one(rule: &Vec<(usize, usize)>, f: usize) -> bool {
    let mut valid = false;
    for (min, max) in rule {
        if f >= *min && f <= *max {
            valid = true;
            break;
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
            get_answer(include_str!("../assets/day16_input.txt")),
            Some(426362917709)
        );
    }
}
