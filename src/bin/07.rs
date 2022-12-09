#![feature(iter_intersperse)]

use aoc::read_file_input;
use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    *,
};

#[derive(Debug)]
enum Operation<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Files<'a>>),
}

#[derive(Debug)]
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug)]
enum Files<'a> {
    File { size: u32 },
    Dir(&'a str),
}

fn main() {
    let input = read_file_input("07.txt".to_string());

    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn file(input: &str) -> IResult<&str, Files> {
    let (input, (size, _)) = separated_pair(
        nom::character::complete::u32,
        tag(" "),
        is_a("qwertyuiopasdfghjklzxcvbnm."),
    )(input)?;

    Ok((input, Files::File { size }))
}

fn directory(input: &str) -> IResult<&str, Files> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;

    Ok((input, Files::Dir(name)))
}

fn ls(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((file, directory)))(input)?;

    Ok((input, Operation::Ls(files)))
}

fn cd(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag("/"), tag(".."), alpha1))(input)?;
    let op = match dir {
        "/" => Operation::Cd(Cd::Root),
        ".." => Operation::Cd(Cd::Up),
        name => Operation::Cd(Cd::Down(name)),
    };

    Ok((input, op))
}

fn commands(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, cmds) = separated_list1(newline, alt((ls, cd)))(input)?;

    Ok((input, cmds))
}

fn calculate_sizes<'a>(
    (mut context, mut sizes): (Vec<&'a str>, BTreeMap<Vec<&'a str>, u32>),
    command: &'a Operation,
) -> (Vec<&'a str>, BTreeMap<Vec<&'a str>, u32>) {
    match command {
        Operation::Cd(Cd::Root) => {
            context.push("/");
        }
        Operation::Cd(Cd::Up) => {
            context.pop();
        }
        Operation::Cd(Cd::Down(name)) => {
            context.push(name);
        }
        Operation::Ls(files) => {
            let sum = files
                .iter()
                .filter_map(|file| {
                    if let Files::File { size, .. } = file {
                        Some(size)
                    } else {
                        None
                    }
                })
                .sum::<u32>();

            for i in 0..context.len() {
                sizes
                    .entry(context[0..=i].to_vec())
                    .and_modify(|v| *v += sum)
                    .or_insert(sum);
            }
        }
    };
    (context, sizes)
}

pub fn part1(input: &str) -> usize {
    let cmds = commands(input).unwrap().1;
    let (_, sizes) = cmds.iter().fold((vec![], BTreeMap::new()), calculate_sizes);

    sizes
        .iter()
        .filter(|(_, &size)| size < 100000)
        .map(|(_, size)| size)
        .sum::<u32>() as usize
}

pub fn part2(input: &str) -> usize {
    let cmds = commands(input).unwrap().1;
    let (_, sizes) = cmds.iter().fold((vec![], BTreeMap::new()), calculate_sizes);

    let used_space = sizes.get(&vec!["/"]).unwrap();

    let current_free_space = 70_000_000 - used_space;
    let need_to_free_at_least = 30_000_000 - current_free_space;

    let valid_dirs = sizes
        .iter()
        .filter(|(_, &size)| size > need_to_free_at_least)
        .map(|(_, size)| size)
        .collect::<Vec<&u32>>();

    **valid_dirs.iter().min().unwrap() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), 95437);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), 24933642);
    }
}
