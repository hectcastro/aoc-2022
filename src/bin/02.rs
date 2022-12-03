use aoc::read_file_input;
use std::{cmp::Ordering, str::FromStr};

#[derive(PartialEq, Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == &Move::Scissors && other == &Move::Rock {
            Some(Ordering::Less)
        } else if self == &Move::Rock && other == &Move::Scissors {
            Some(Ordering::Greater)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Not a known move".to_string()),
        }
    }
}

fn main() {
    let input = read_file_input("02.txt".to_string());

    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let rounds = input.lines();

    rounds
        .map(|line| {
            let moves: Vec<Move> = line
                .split(' ')
                .map(|m| m.parse::<Move>().unwrap())
                .collect();

            let (them, us) = (moves[0], moves[1]);

            match them.partial_cmp(&us) {
                // Draw.
                Some(Ordering::Equal) => 3 + us as u32,
                // We win.
                Some(Ordering::Less) => 6 + us as u32,
                // They win.
                Some(Ordering::Greater) => us as u32,
                None => {
                    panic!("Moves should be comparable")
                }
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let rounds = input.lines();

    let scores = rounds.map(|line| {
        let moves: Vec<&str> = line.split(' ').collect();
        let them = moves[0].parse::<Move>().unwrap();

        match moves[1] {
            // We need to lose.
            "X" => {
                let us = match them {
                    Move::Rock => Move::Scissors,
                    Move::Paper => Move::Rock,
                    Move::Scissors => Move::Paper,
                };
                us as u32
            }
            // We need to draw.
            "Y" => 3 + them as u32,
            // We need to win.
            "Z" => {
                let us = match them {
                    Move::Rock => Move::Paper,
                    Move::Paper => Move::Scissors,
                    Move::Scissors => Move::Rock,
                };
                6 + us as u32
            }
            _ => {
                panic!("Unexpected response");
            }
        }
    });

    scores.sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 15);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 12);
    }
}
