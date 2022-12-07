use aoc::read_file_input;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, multispace1, newline, space1},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded},
    *,
};

#[derive(Debug)]
struct Move {
    number: u32,
    from: u32,
    to: u32,
}

fn main() {
    let input = read_file_input("05.txt".to_string());

    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))(input)?;

    let result = match c {
        "   " => None,
        value => Some(value),
    };

    Ok((input, result))
}

fn parse_crates(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, result) = separated_list1(tag(" "), parse_crate)(input)?;

    Ok((input, result))
}

fn parse_move_instruction(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, number) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;

    Ok((
        input,
        Move {
            number,
            from: from - 1,
            to: to - 1,
        },
    ))
}

fn parse_crate_stacks(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<Move>)> {
    let (input, horizontal_crates) = separated_list1(newline, parse_crates)(input)?;
    // Newline after horizontal crates.
    let (input, _) = newline(input)?;
    // Crate stack indexes.
    let (input, _) = many1(preceded(space1, digit1))(input)?;
    // Newlines after crate stack indexes.
    let (input, _) = multispace1(input)?;
    let (input, moves) = separated_list1(newline, parse_move_instruction)(input)?;

    let mut vertical_crates: Vec<Vec<Option<&str>>> = vec![];

    // Initialize vertical crates vector.
    for _ in 0..=horizontal_crates.len() {
        vertical_crates.push(vec![]);
    }

    // Fill vertical crates vector.
    for vec in horizontal_crates.iter().rev() {
        for (i, c) in vec.iter().enumerate() {
            vertical_crates[i].push(*c)
        }
    }

    // Finalize crate stacks by removing empty crate locations.
    let final_crates: Vec<Vec<&str>> = vertical_crates
        .iter()
        .map(|vec| vec.iter().filter_map(|v| *v).collect())
        .collect();

    Ok((input, (final_crates, moves)))
}

pub fn part1(input: &str) -> String {
    let (_, (mut crate_stacks, moves)) = parse_crate_stacks(input).unwrap();

    for Move { number, from, to } in moves.iter() {
        let crate_stack_len = crate_stacks[*from as usize].len();

        let crate_stack = crate_stacks[*from as usize]
            .drain((crate_stack_len - *number as usize)..)
            .rev()
            .collect::<Vec<&str>>();

        for c in crate_stack.iter() {
            crate_stacks[*to as usize].push(c);
        }
    }

    // Build string based on top-most crate in each stack.
    crate_stacks
        .iter()
        .map(|v| match v.iter().last() {
            Some(c) => c,
            None => "",
        })
        .collect()
}

pub fn part2(input: &str) -> String {
    let (_, (mut crate_stacks, moves)) = parse_crate_stacks(input).unwrap();

    for Move { number, from, to } in moves.iter() {
        let crate_stack_len = crate_stacks[*from as usize].len();

        let crate_stack = crate_stacks[*from as usize]
            .drain((crate_stack_len - *number as usize)..)
            .collect::<Vec<&str>>();

        for c in crate_stack.iter() {
            crate_stacks[*to as usize].push(c);
        }
    }

    let result: String = crate_stacks
        .iter()
        .map(|v| match v.iter().last() {
            Some(c) => c,
            None => "",
        })
        .collect();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "MCD");
    }
}
