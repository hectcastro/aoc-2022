use aoc::read_file_input;

use std::collections::BTreeSet;

use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete, character::complete::line_ending,
    multi::separated_list1, sequence::separated_pair, *,
};

fn main() {
    let input = read_file_input("14.txt".to_string());

    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn line(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (input, pairs) = separated_list1(
        tag(" -> "),
        separated_pair(complete::u32, complete::char(','), complete::u32),
    )(input)?;

    let product = pairs
        .into_iter()
        .tuple_windows()
        .flat_map(|((ax, ay), (bx, by))| {
            let x_range = ax.min(bx)..=ax.max(bx);
            let y_range = ay.min(by)..=ay.max(by);

            x_range.cartesian_product(y_range)
        })
        .collect();

    Ok((input, product))
}

fn rocks(input: &str) -> IResult<&str, BTreeSet<(u32, u32)>> {
    let (input, pairs) = separated_list1(line_ending, line)(input)?;
    let map = pairs.into_iter().flatten().collect();

    Ok((input, map))
}

fn part1(input: &str) -> usize {
    let (_, mut board) = rocks(input).unwrap();
    let rock_count = board.len();
    let mut rocks = board.iter().collect::<Vec<&(u32, u32)>>();

    rocks.sort_by(|a, b| a.1.cmp(&b.1));

    let lowest_rock = **rocks.last().unwrap();
    let mut current_sand = (500, 0);

    loop {
        if current_sand.1 > lowest_rock.1 {
            println!("break");
            break;
        }

        let down = (current_sand.0, current_sand.1 + 1);
        let left = (current_sand.0 - 1, current_sand.1 + 1);
        let right = (current_sand.0 + 1, current_sand.1 + 1);

        match (board.get(&down), board.get(&left), board.get(&right)) {
            (Some(_), Some(_), Some(_)) => {
                board.insert(current_sand);
                current_sand = (500, 0);
            }
            (None, _, _) => {
                current_sand = down;
            }
            (_, None, _) => {
                current_sand = left;
            }
            (_, _, None) => {
                current_sand = right;
            }
        };
    }

    board.len() - rock_count
}

fn part2(input: &str) -> usize {
    let (_, mut board) = rocks(input).unwrap();
    let rock_count = board.len();
    let mut rocks = board.iter().collect::<Vec<&(u32, u32)>>();

    rocks.sort_by(|a, b| a.1.cmp(&b.1));

    let lowest_rock = **rocks.last().unwrap();
    let mut current_sand = (500, 0);

    while let None = board.get(&(500, 0)) {
        let down = (current_sand.0, current_sand.1 + 1);
        let left = (current_sand.0 - 1, current_sand.1 + 1);
        let right = (current_sand.0 + 1, current_sand.1 + 1);

        match (
            board.get(&down).or_else(|| {
                if down.1 == lowest_rock.1 + 2 {
                    Some(&lowest_rock)
                } else {
                    None
                }
            }),
            board.get(&left).or_else(|| {
                if left.1 == lowest_rock.1 + 2 {
                    Some(&lowest_rock)
                } else {
                    None
                }
            }),
            board.get(&right).or_else(|| {
                if right.1 == lowest_rock.1 + 2 {
                    Some(&lowest_rock)
                } else {
                    None
                }
            }),
        ) {
            (Some(_), Some(_), Some(_)) => {
                board.insert(current_sand);
                current_sand = (500, 0);
            }
            (None, _, _) => {
                current_sand = down;
            }
            (_, None, _) => {
                current_sand = left;
            }
            (_, _, None) => {
                current_sand = right;
            }
        };
    }

    board.len() - rock_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), 24);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), 93);
    }
}
