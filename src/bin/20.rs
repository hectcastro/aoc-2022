use aoc::read_file_input;

use nom::{
    branch::alt,
    character::complete,
    character::complete::line_ending,
    combinator::{eof, iterator},
    sequence::terminated,
    *,
};

fn main() {
    let input = read_file_input("20.txt".to_string());

    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn numbers(input: &str) -> IResult<&str, Vec<(usize, i64)>> {
    let mut it = iterator(input, terminated(complete::i64, alt((line_ending, eof))));
    let numbers = it.enumerate().collect::<Vec<_>>();
    let (input, _) = it.finish()?;

    Ok((input, numbers))
}

pub fn part1(input: &str) -> i64 {
    let (_, numbers) = numbers(input).unwrap();
    let mut state = numbers.clone();

    for (id, _) in numbers.iter() {
        let index = state
            .iter()
            .position(|state_value| state_value.0 == *id)
            .unwrap();

        let current = state.remove(index);
        let added = index as i64 + current.1;
        let new_index = added.rem_euclid(state.len() as i64);

        state.insert(new_index as usize, current);
    }

    let zero_pos = state.iter().position(|v| v.1 == 0).unwrap();

    let a = state[(1000 + zero_pos) % state.len()].1;
    let b = state[(2000 + zero_pos) % state.len()].1;
    let c = state[(3000 + zero_pos) % state.len()].1;

    a + b + c
}

pub fn part2(input: &str) -> i64 {
    let (_, mut numbers) = numbers(input).unwrap();
    numbers.iter_mut().for_each(|tuple| tuple.1 *= 811589153);

    let mut state = numbers.clone();

    for _ in 0..10 {
        for (id, _) in numbers.iter() {
            let index = state
                .iter()
                .position(|state_value| state_value.0 == *id)
                .unwrap();

            let current = state.remove(index);
            let added = index as i64 + current.1;
            let new_index = added.rem_euclid(state.len() as i64);

            state.insert(new_index as usize, current);
        }
    }

    let zero_pos = state.iter().position(|v| v.1 == 0).unwrap();

    let a = state[(1000 + zero_pos) % state.len()].1;
    let b = state[(2000 + zero_pos) % state.len()].1;
    let c = state[(3000 + zero_pos) % state.len()].1;

    a + b + c
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), 3);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), 1_623_178_306);
    }
}
