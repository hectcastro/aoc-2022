use std::collections::BTreeSet;

use aoc::read_file_input;

fn main() {
    let input = read_file_input("06.txt".to_string());

    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

pub fn part1(input: &str) -> usize {
    let window_size = 4;
    let chars = input.chars().collect::<Vec<char>>();

    let datastream_buffer = chars
        .windows(window_size)
        .enumerate()
        .find(|(_i, window)| {
            let unique_chars = window.iter().collect::<BTreeSet<&char>>();
            window.len() == unique_chars.len()
        })
        .unwrap();

    datastream_buffer.0 + window_size
}

pub fn part2(input: &str) -> usize {
    let window_size = 14;
    let chars = input.chars().collect::<Vec<char>>();

    let datastream_buffer = chars
        .windows(window_size)
        .enumerate()
        .find(|(_i, window)| {
            let unique_chars = window.iter().collect::<BTreeSet<&char>>();
            window.len() == unique_chars.len()
        })
        .unwrap();

    datastream_buffer.0 + window_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
