use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

const DEBUG: bool = false;

pub fn main() {
    println!("-- Advent of Code - Day 22 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day22_input_demo1.txt");
    // let input = include_str!("../assets/day22_input_demo2.txt");
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

type Deck = VecDeque<usize>;

fn get_answer(input: &str) -> Option<usize> {
    let (p1, p2) = input.split_once("\n\n").unwrap();
    let mut deck1 = Deck::new();
    p1.lines().skip(1).for_each(|v| {
        let val = v.parse::<usize>().unwrap();
        deck1.push_back(val);
    });
    let mut deck2 = Deck::new();
    p2.lines().skip(1).for_each(|v| {
        let val = v.parse::<usize>().unwrap();
        deck2.push_back(val);
    });

    // let mut result = 0;
    let level = 1;

    let winner = lets_play(level, &mut deck1, &mut deck2);

    if DEBUG {
        println!("== Post-game results ==");
    }
    let v = deck1
        .iter()
        .copied()
        .map(|x| format!("{x}"))
        .collect::<Vec<_>>();
    if DEBUG {
        println!("Player 1's deck: {}", v.join(", "));
    }
    let v = deck2
        .iter()
        .copied()
        .map(|x| format!("{x}"))
        .collect::<Vec<_>>();
    if DEBUG {
        println!("Player 2's deck: {}", v.join(", "));
    }

    let result = if winner == 1 {
        get_deck_score(deck1)
    } else {
        get_deck_score(deck2)
    };
    Some(result)
}

fn lets_play(level: usize, deck1: &mut Deck, deck2: &mut Deck) -> usize {
    let mut round = 1;

    if DEBUG {
        println!();
        println!("=== Game {level} ===");
    }

    let mut prev_games: HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();

    loop {
        if DEBUG {
            println!();
            println!("-- Round {round} (Game {level}) --");
            let v = deck1
                .iter()
                .copied()
                .map(|x| format!("{x}"))
                .collect::<Vec<_>>();
            println!("Player 1's deck: {}", v.join(", "));
            let v = deck2
                .iter()
                .copied()
                .map(|x| format!("{x}"))
                .collect::<Vec<_>>();
            println!("Player 2's deck: {}", v.join(", "));
        }

        // rule 1
        let deck1_p = deck1.clone();
        let deck2_p = deck2.clone();
        let prev1 = &deck1_p.iter().copied().collect::<Vec<_>>();
        let prev2 = &deck2_p.iter().copied().collect::<Vec<_>>();
        if !prev_games.insert((prev1.clone(), prev2.clone())) {
            if DEBUG {
                print!("same deck - player 1 wins");
            }
            return 1;
        }

        if deck1.is_empty() {
            if DEBUG {
                println!("The winner of game {level} is player 1!");
                println!();
                println!("...anyway, back to game {level}.");
            }
            return 2;
        }
        if deck2.is_empty() {
            if DEBUG {
                println!("The winner of game {level} is player 2!");
                println!();
                println!("...anyway, back to game {level}.");
            }
            return 1;
        }

        let c1 = deck1.pop_front().unwrap();
        if DEBUG {
            println!("Player 1 plays: {}", c1);
        }
        let c2 = deck2.pop_front().unwrap();
        if DEBUG {
            println!("Player 2 plays: {}", c2);
        }

        // If both players have at least as many cards remaining in their deck as the value of the card they just drew, the winner of the round is determined by playing a new game of Recursive Combat (see below).
        let winner;
        if deck1.len() >= c1 && deck2.len() >= c2 {
            // start_new_game()
            if DEBUG {
                println!("Playing a sub-game to determine the winner...");
                // in a sub-game, you should only copy the next "n" cards from the original deck !
            }
            let mut subdeck1 = deck1.iter().take(c1).copied().collect::<Deck>();
            let mut subdeck2 = deck2.iter().take(c2).copied().collect::<Deck>();
            winner = lets_play(level + 1, &mut subdeck1, &mut subdeck2);
        } else if c1 > c2 {
            winner = 1;
        } else {
            winner = 2;
        }

        if winner == 1 {
            if DEBUG {
                println!("Player 1 wins round {round} of game {level}!");
            }

            deck1.push_back(c1);
            deck1.push_back(c2);
        } else {
            if DEBUG {
                println!("Player 2 wins round {round} of game {level}!");
            }

            deck2.push_back(c2);
            deck2.push_back(c1);
        }

        round += 1;
    }
}

fn get_deck_score(deck: Deck) -> usize {
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
            Some(291)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day22_input.txt")),
            Some(35495)
        );
    }
}
