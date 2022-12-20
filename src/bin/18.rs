use aoc::read_file_input;

use glam::IVec3;
use nom::{
    bytes::complete::tag, character::complete, character::complete::line_ending,
    multi::separated_list1, *,
};

use std::collections::HashSet;

fn main() {
    let input = read_file_input("18.txt".to_string());

    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

pub fn points(input: &str) -> IResult<&str, Vec<IVec3>> {
    separated_list1(
        line_ending,
        separated_list1(tag(","), complete::i32).map(|vec| IVec3::new(vec[0], vec[1], vec[2])),
    )(input)
}

fn process_block(&IVec3 { x, y, z }: &IVec3, points: &HashSet<IVec3>) -> usize {
    let x_low = IVec3::new(x - 1, y, z);
    let x_high = IVec3::new(x + 1, y, z);
    let y_low = IVec3::new(x, y - 1, z);
    let y_high = IVec3::new(x, y + 1, z);
    let z_low = IVec3::new(x, y, z - 1);
    let z_high = IVec3::new(x, y, z + 1);

    [x_low, x_high, y_low, y_high, z_low, z_high]
        .iter()
        .filter(|ivec| points.get(ivec).is_none())
        .map(|ivec| {
            if is_interior_block(ivec, points) {
                (1, 0)
            } else {
                (0, 1)
            }
        })
        .map(|(_interior, exterior)| exterior)
        .sum::<usize>()
}

fn is_interior_block(&IVec3 { x, y, z }: &IVec3, points: &HashSet<IVec3>) -> bool {
    let bounded_x_pos = points
        .iter()
        .any(|point| point.x > x && point.y == y && point.z == z);
    let bounded_x_neg = points
        .iter()
        .any(|point| point.x < x && point.y == y && point.z == z);
    let bounded_y_pos = points
        .iter()
        .any(|point| point.x == x && point.y > y && point.z == z);
    let bounded_y_neg = points
        .iter()
        .any(|point| point.x == x && point.y < y && point.z == z);
    let bounded_z_pos = points
        .iter()
        .any(|point| point.x == x && point.y == y && point.z > z);
    let bounded_z_neg = points
        .iter()
        .any(|point| point.x == x && point.y == y && point.z < z);

    [
        bounded_x_pos,
        bounded_x_neg,
        bounded_y_pos,
        bounded_y_neg,
        bounded_z_pos,
        bounded_z_neg,
    ]
    .iter()
    .all(|v| *v)
}

fn part1(input: &str) -> usize {
    let (_, points) = points(input).unwrap();
    let points: HashSet<IVec3> = HashSet::from_iter(points.into_iter());

    let surface_area = points
        .iter()
        .map(|&IVec3 { x, y, z }| {
            let x_low = IVec3::new(x - 1, y, z);
            let x_high = IVec3::new(x + 1, y, z);
            let y_low = IVec3::new(x, y - 1, z);
            let y_high = IVec3::new(x, y + 1, z);
            let z_low = IVec3::new(x, y, z - 1);
            let z_high = IVec3::new(x, y, z + 1);

            [x_low, x_high, y_low, y_high, z_low, z_high]
                .iter()
                .filter(|ivec| points.get(ivec).is_none())
                .count()
        })
        .sum::<usize>();

    surface_area
}

fn part2(input: &str) -> usize {
    let (_, points) = points(input).unwrap();
    let points: HashSet<IVec3> = HashSet::from_iter(points.into_iter());

    let surface_area = points
        .iter()
        .map(|&IVec3 { x, y, z }| {
            let x_low = IVec3::new(x - 1, y, z);
            let x_high = IVec3::new(x + 1, y, z);
            let y_low = IVec3::new(x, y - 1, z);
            let y_high = IVec3::new(x, y + 1, z);
            let z_low = IVec3::new(x, y, z - 1);
            let z_high = IVec3::new(x, y, z + 1);

            [x_low, x_high, y_low, y_high, z_low, z_high]
                .iter()
                .filter(|ivec| points.get(ivec).is_none())
                .map(|ivec| {
                    if is_interior_block(ivec, &points) {
                        let IVec3 { x, y, z } = *ivec;
                        let x_low = IVec3::new(x - 1, y, z);
                        let x_high = IVec3::new(x + 1, y, z);
                        let y_low = IVec3::new(x, y - 1, z);
                        let y_high = IVec3::new(x, y + 1, z);
                        let z_low = IVec3::new(x, y, z - 1);
                        let z_high = IVec3::new(x, y, z + 1);

                        let is_really_exterior_block =
                            [x_low, x_high, y_low, y_high, z_low, z_high]
                                .iter()
                                .filter(|ivec| points.get(ivec).is_none())
                                .any(|block| process_block(block, &points) >= 1);

                        if is_really_exterior_block {
                            (0, 1)
                        } else {
                            (1, 0)
                        }
                    } else {
                        (0, 1)
                    }
                })
                .map(|(_interior, exterior)| exterior)
                .sum::<usize>()
        })
        .sum::<usize>();

    surface_area
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), 64);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), 58);
    }
}
