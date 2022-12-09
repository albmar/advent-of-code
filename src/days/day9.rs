use std::str::FromStr;

use nalgebra::Vector2;

use crate::{solver::Solver, util::*};

pub struct Day9;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::Right),
            "U" => Ok(Self::Up),
            "L" => Ok(Self::Left),
            "D" => Ok(Self::Down),
            _ => Err(format!("`{s}` is not a direction")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Motion {
    dir: Direction,
    amount: i32,
}

impl From<Motion> for Vector2<i32> {
    fn from(value: Motion) -> Self {
        match value.dir {
            Direction::Right => Vector2::new(value.amount, 0),
            Direction::Up => Vector2::new(0, value.amount),
            Direction::Left => Vector2::new(-value.amount, 0),
            Direction::Down => Vector2::new(0, -value.amount),
        }
    }
}

impl<'a> Solver<'a> for Day9 {
    type Parsed = Vec<Motion>;
    type Output = u32;

    fn parse(input: &'a str) -> Self::Parsed {
        input
            .lines()
            .filter_map(|line| line.split_once(" "))
            .map(|(dir, amount)| Motion {
                dir: dir.parse().unwrap(),
                amount: amount.parse().unwrap(),
            })
            .collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let mut head = Vector2::new(0, 0);
        let mut tail = Vector2::new(0, 0);
        let mut visited = HashSet::new();
        visited.insert(tail);
        data.iter().for_each(|motion| {
            (0..motion.amount)
                .map(|_| Motion {
                    dir: motion.dir,
                    amount: 1,
                })
                .for_each(|motion| {
                    let head_before = head;
                    head += Vector2::from(motion);
                    let diff = head - tail;
                    if diff.dot(&diff) > 2 {
                        tail = head_before;
                        visited.insert(tail);
                    }
                });
        });
        visited.len() as u32
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        let mut rope = vec![Vector2::new(0, 0); 10];
        let mut visited = HashSet::new();
        visited.insert(rope.last().unwrap().clone());
        data.iter().for_each(|motion| {
            (0..motion.amount)
                .map(|_| Motion {
                    dir: motion.dir,
                    amount: 1,
                })
                .for_each(|motion| {
                    rope[0] += Vector2::from(motion);
                    (0..9).for_each(|i| {
                        let diff = rope[i] - rope[i + 1];
                        if diff.dot(&diff) > 2 {
                            rope[i + 1] += Vector2::new(diff[0].clamp(-1, 1), diff[1].clamp(-1, 1));
                            if i == 8 {
                                visited.insert(rope[i + 1].clone());
                            }
                        }
                    });
                });
        });
        visited.len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d9p1() {
        assert_eq!(
            Day9::part1(Day9::parse(
                "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
            )),
            13
        );
    }

    #[test]
    fn d9p2() {
        assert_eq!(
            Day9::part2(Day9::parse(
                "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
            )),
            1
        );
    }

    #[test]
    fn larger() {
        assert_eq!(
            Day9::part2(Day9::parse(
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            )),
            36
        );
    }
}
