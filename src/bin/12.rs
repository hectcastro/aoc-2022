use aoc::read_file_input;

use itertools::Itertools;
use nom::{
    character::complete::{alpha1, newline},
    multi::separated_list1,
    *,
};
use petgraph::algo::dijkstra;
use petgraph::prelude::*;

fn main() {
    let input = read_file_input("12.txt".to_string());

    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn grid(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(
        newline,
        alpha1.map(|letters: &str| letters.chars().collect()),
    )(input)
}

pub fn part1(input: &str) -> usize {
    let (_, grid) = grid(input).unwrap();

    let start = grid
        .iter()
        .enumerate()
        .flat_map(|(i, v)| v.iter().enumerate().zip(std::iter::repeat(i)))
        .find_map(|((x, &c), y)| {
            if c == 'S' {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .unwrap();

    let end = grid
        .iter()
        .enumerate()
        .flat_map(|(i, v)| v.iter().enumerate().zip(std::iter::repeat(i)))
        .find_map(|((x, &c), y)| {
            if c == 'E' {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .unwrap();

    let grid: Vec<Vec<char>> = grid
        .iter()
        .map(|vec| {
            vec.iter()
                .map(|c| match c {
                    'S' => 'a',
                    'E' => 'z',
                    v => *v,
                })
                .collect()
        })
        .collect();

    let edges = (0i32..(grid.len() as i32))
        .cartesian_product(0i32..(grid[0].len() as i32))
        .flat_map(|(y, x)| {
            let neighbors = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
            let current_node_id = (x, y);

            neighbors
                .iter()
                .filter_map(|cell| {
                    grid.get(cell.1 as usize)
                        .and_then(|vec| vec.get(cell.0 as usize))
                        .and_then(|existing_cell| {
                            let current_node_height = grid[y as usize][x as usize];

                            if current_node_height as u8 + 1 >= *existing_cell as u8 {
                                Some((
                                    (current_node_id.0, current_node_id.1, current_node_height),
                                    (cell.0, cell.1, *existing_cell),
                                ))
                            } else {
                                None
                            }
                        })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<((i32, i32, char), (i32, i32, char))>>();

    let graph = DiGraphMap::<(i32, i32, char), ()>::from_edges(&edges);

    let path_cost = dijkstra(
        &graph,
        (start.0, start.1, 'a'),
        Some((end.0, end.1, 'z')),
        |_| 1,
    );

    path_cost[&(end.0, end.1, 'z')]
}

pub fn part2(input: &str) -> usize {
    let (_, grid) = grid(input).unwrap();

    let end = grid
        .iter()
        .enumerate()
        .flat_map(|(i, v)| v.iter().enumerate().zip(std::iter::repeat(i)))
        .find_map(|((x, &c), y)| {
            if c == 'E' {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .unwrap();

    let grid: Vec<Vec<char>> = grid
        .iter()
        .map(|vec| {
            vec.iter()
                .map(|c| match c {
                    'S' => 'a',
                    'E' => 'z',
                    v => *v,
                })
                .collect()
        })
        .collect();

    let edges = (0i32..(grid.len() as i32))
        .cartesian_product(0i32..(grid[0].len() as i32))
        .flat_map(|(y, x)| {
            let neighbors = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
            let current_node_id = (x, y);

            neighbors
                .iter()
                .filter_map(|cell| {
                    grid.get(cell.1 as usize)
                        .and_then(|vec| vec.get(cell.0 as usize))
                        .and_then(|existing_cell| {
                            let current_node_height = grid[y as usize][x as usize];

                            if current_node_height as u8 + 1 >= *existing_cell as u8 {
                                Some((
                                    (current_node_id.0, current_node_id.1, current_node_height),
                                    (cell.0, cell.1, *existing_cell),
                                ))
                            } else {
                                None
                            }
                        })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<((i32, i32, char), (i32, i32, char))>>();

    let graph = DiGraphMap::<(i32, i32, char), ()>::from_edges(edges.iter().map(|(a, b)| (*b, *a)));
    let path_cost = dijkstra(&graph, (end.0, end.1, 'z'), None, |_| 1);

    let mut steps: Vec<i32> = path_cost
        .iter()
        .filter_map(
            |(node, cost)| {
                if node.2 == 'a' {
                    Some(*cost)
                } else {
                    None
                }
            },
        )
        .collect();

    steps.sort();

    *steps.first().unwrap() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), 31);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), 29);
    }
}
