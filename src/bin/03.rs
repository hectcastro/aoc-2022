#![feature(iter_array_chunks)]

use std::collections::HashMap;

use aoc::read_file_input;

fn main() {
    let input = read_file_input("03.txt".to_string());

    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    let common_item_scores = input.lines().map(|line| {
        let half_sack_item_count = line.len() / 2;
        let compartment_a_items = &line[0..half_sack_item_count];
        let compartment_b_items = &line[half_sack_item_count..(half_sack_item_count * 2)];

        let common_items = compartment_a_items
            .chars()
            .find(|c| compartment_b_items.contains(*c))
            .unwrap();

        letter_scores.get(&common_items).unwrap()
    });

    common_item_scores.sum::<usize>()
}

fn part2(input: &str) -> usize {
    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    let sack_groups = input.lines().array_chunks::<3>();

    let priorities = sack_groups.map(|[sack_a, sack_b, sack_c]| {
        let common_items = sack_a
            .chars()
            .find(|item| sack_b.contains(*item) && sack_c.contains(*item))
            .unwrap();
        letter_scores.get(&common_items).unwrap()
    });

    priorities.sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 157);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 70);
    }
}
