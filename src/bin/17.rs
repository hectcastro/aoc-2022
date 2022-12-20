use aoc::read_file_input;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    character::complete::line_ending,
    multi::{many1, separated_list1},
    *,
};

use std::collections::BTreeMap;

const ROCKS: &str = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##";

#[derive(Debug)]
enum Move {
    Left,
    Right,
}

#[derive(Debug)]
enum Rock {
    Rock,
    Gap,
}

struct Field(BTreeMap<(usize, usize), Rock>);

impl Field {
    fn highest_rock_y(&self) -> usize {
        *self.0.keys().map(|(_, y)| y).max().unwrap_or(&0)
    }

    fn can_place_rock_at(
        &self,
        rock: &RockFormation,
        desired_next_position: (usize, usize),
    ) -> bool {
        rock.offsets.iter().all(|(x, y)| {
            self.0
                .get(&(desired_next_position.0 + x, desired_next_position.1 - y))
                .is_none()
        })
    }
}

#[derive(Debug)]
struct RockFormation {
    rocks: Vec<Vec<Rock>>,
    offsets: Vec<(usize, usize)>,
}

impl RockFormation {
    fn height(&self) -> usize {
        self.rocks.len()
    }

    fn max_width(&self) -> usize {
        self.rocks
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|rock| match rock {
                        Rock::Rock => true,
                        Rock::Gap => false,
                    })
                    .count()
            })
            .max()
            .unwrap()
    }
}

fn main() {
    let input = read_file_input("17.txt".to_string());

    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn moves(input: &str) -> IResult<&str, Vec<Move>> {
    many1(alt((
        complete::char('<').map(|_| Move::Left),
        complete::char('>').map(|_| Move::Right),
    )))(input)
}

fn rocks(input: &str) -> IResult<&str, Vec<RockFormation>> {
    separated_list1(
        tag("\n\n"),
        separated_list1(
            line_ending,
            many1(alt((
                complete::char('#').map(|_| Rock::Rock),
                complete::char('.').map(|_| Rock::Gap),
            ))),
        )
        .map(|rocks| RockFormation {
            offsets: rocks
                .iter()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.iter().enumerate().filter_map(move |(x, r)| match r {
                        Rock::Rock => Some((x, y)),
                        Rock::Gap => None,
                    })
                })
                .collect::<Vec<(usize, usize)>>(),
            rocks,
        }),
    )(input)
}

pub fn process(input: &str, rock_limit: usize) -> usize {
    let (_, rocks) = rocks(ROCKS).unwrap();
    let (_, moves) = moves(input).unwrap();

    let mut rocks = rocks.iter().cycle();
    let mut moves = moves.iter().cycle();
    let mut field: Field = Field(BTreeMap::new());

    for x in 0..7 {
        field.0.insert((x, 0), Rock::Rock);
    }

    let mut rocks_stopped: usize = 0;

    while rocks_stopped != rock_limit {
        let max_rock_height = field.highest_rock_y();
        let current_rock = rocks.next().unwrap();

        let mut current_rock_position: (usize, usize) =
            (2, max_rock_height + 3 + current_rock.height());

        loop {
            let next_move = moves.next().unwrap();

            let current_position = match next_move {
                Move::Left => {
                    if let Some(x_pos) = current_rock_position.0.checked_sub(1) {
                        let desired_next_position = (x_pos, current_rock_position.1);

                        if !field.can_place_rock_at(current_rock, desired_next_position) {
                            current_rock_position
                        } else {
                            desired_next_position
                        }
                    } else {
                        current_rock_position
                    }
                }
                Move::Right => {
                    let desired_next_position =
                        (current_rock_position.0 + 1, current_rock_position.1);

                    if current_rock_position.0 == 7 - current_rock.max_width()
                        || !field.can_place_rock_at(current_rock, desired_next_position)
                    {
                        current_rock_position
                    } else {
                        desired_next_position
                    }
                }
            };

            let desired_next_position = (current_position.0, current_position.1 - 1);

            if field.can_place_rock_at(current_rock, desired_next_position) {
                current_rock_position = desired_next_position;
            } else {
                for position in current_rock.offsets.iter() {
                    field.0.insert(
                        (
                            position.0 + current_position.0,
                            current_position.1 - position.1,
                        ),
                        Rock::Rock,
                    );
                }

                rocks_stopped += 1;
                break;
            }
        }
    }

    field.highest_rock_y()
}

fn part1(input: &str) -> usize {
    process(input, 2022)
}

fn part2(input: &str) -> usize {
    process(input, 1_000_000_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), 3068);
    }

    #[test]
    #[ignore]
    fn part2_works() {
        assert_eq!(part2(INPUT), 1514285714288);
    }
}
