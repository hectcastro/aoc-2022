use aoc::read_file_input;

use nom::{
    character::complete::{anychar, newline},
    combinator::verify,
    multi::{many1, separated_list1},
    *,
};

fn main() {
    let input = read_file_input("08.txt".to_string());

    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn tree(input: &str) -> IResult<&str, u32> {
    let (input, tree) = verify(anychar, |tree| tree.is_ascii_digit())(input)?;

    Ok((input, tree.to_digit(10).unwrap()))
}

fn parse_trees(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (input, vecs) = separated_list1(newline, many1(tree))(input)?;

    Ok((input, vecs))
}

pub fn part1(input: &str) -> usize {
    let (_, trees) = parse_trees(input).unwrap();

    let max_length = trees.len() - 1;
    let mut visible_trees: Vec<Vec<bool>> = trees
        .iter()
        .enumerate()
        .map(|(i, tree_line)| {
            let tree_line_max_length = tree_line.len() - 1;

            tree_line
                .iter()
                .enumerate()
                .map(|(j, _)| (i == 0 || i == max_length || j == 0 || j == tree_line_max_length))
                .collect()
        })
        .collect();

    for i in 0..trees.len() {
        let mut current_tree_size = 0;

        for j in 0..trees[0].len() {
            if j == 0 {
                current_tree_size = trees[i][j] as usize;
            } else if trees[i][j] > current_tree_size as u32 {
                current_tree_size = trees[i][j] as usize;
                visible_trees[i][j] = true;
            }
        }
    }

    for i in (0..trees.len()).rev() {
        let mut current_tree_size = 0;

        for j in (0..trees[0].len()).rev() {
            if j == trees.len() - 1 {
                current_tree_size = trees[i][j] as usize;
            } else if trees[i][j] > current_tree_size as u32 {
                current_tree_size = trees[i][j] as usize;
                visible_trees[i][j] = true;
            }
        }
    }

    for i in 0..trees.len() {
        let mut current_tree_size = 0;

        for j in 0..trees[0].len() {
            if j == 0 {
                current_tree_size = trees[j][i] as usize;
            } else if trees[j][i] > current_tree_size as u32 {
                current_tree_size = trees[j][i] as usize;
                visible_trees[j][i] = true;
            }
        }
    }

    for i in (0..trees.len()).rev() {
        let mut current_tree_size = 0;

        for j in (0..trees[0].len()).rev() {
            if j == trees.len() - 1 {
                current_tree_size = trees[j][i] as usize;
            } else if trees[j][i] > current_tree_size as u32 {
                current_tree_size = trees[j][i] as usize;
                visible_trees[j][i] = true;
            }
        }
    }

    visible_trees.iter().flatten().filter(|&&v| v).count()
}

pub fn part2(input: &str) -> usize {
    let (_, trees) = parse_trees(input).unwrap();
    let mut high_scenic_score = 0;

    let i_max = trees.len();
    let j_max = trees[0].len();

    for (i, tree_line) in trees.iter().enumerate() {
        for (j, treehouse_height) in tree_line.iter().enumerate() {
            let mut scores = [0, 0, 0, 0];

            // to left
            for j_position in (0..j).rev() {
                if trees[i][j_position] < *treehouse_height {
                    scores[0] += 1;
                } else {
                    scores[0] += 1;
                    break;
                }
            }
            // to right
            for j_position in (j + 1)..j_max {
                if trees[i][j_position] < *treehouse_height {
                    scores[1] += 1;
                } else {
                    scores[1] += 1;
                    break;
                }
            }

            // to up
            for i_position in (0..i).rev() {
                if trees[i_position][j] < *treehouse_height {
                    scores[2] += 1;
                } else {
                    scores[2] += 1;
                    break;
                }
            }
            // to down
            #[allow(clippy::needless_range_loop)]
            for i_position in (i + 1)..i_max {
                if trees[i_position][j] < *treehouse_height {
                    scores[3] += 1;
                } else {
                    scores[3] += 1;
                    break;
                }
            }
            let scenic_score: u32 = scores.iter().product();

            if scenic_score > high_scenic_score {
                high_scenic_score = scenic_score
            }
        }
    }

    high_scenic_score as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), 21);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), 8);
    }
}
