use std::fmt::Display;

use regex::Regex;

use crate::solver::Solver;

pub struct Day5;

#[derive(Debug, Clone, Copy)]
struct Container {
    mark: char,
}

type Stack = Vec<Container>;

#[derive(Debug, Clone)]
pub struct Port {
    stacks: Vec<Stack>,
}

impl Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.stacks
            .iter()
            .map(|stack| stack.iter().map(|c| c.mark))
            .enumerate()
            .map(|(i, stack)| write!(f, "{}: {}\n", i, stack.collect::<String>()))
            .collect()
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Move {
    count: u8,
    from: u8,
    to: u8,
}

impl Move {
    fn execute(&self, port: &mut Port) {
        let from = &mut port.stacks[self.from as usize];
        assert!(from.len() >= self.count as usize);
        let i = from.len() - self.count as usize;
        let drain = from.drain(i..).rev().collect::<Vec<_>>();
        port.stacks[self.to as usize].extend(drain);
    }

    fn execute_9001(&self, port: &mut Port) {
        let from = &mut port.stacks[self.from as usize];
        assert!(from.len() >= self.count as usize);
        let i = from.len() - self.count as usize;
        let drain = from.drain(i..).collect::<Vec<_>>();
        port.stacks[self.to as usize].extend(drain);
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} × {} → {}", self.count, self.from, self.to)
    }
}

impl<'a> Solver<'a> for Day5 {
    type Parsed = (Port, Vec<Move>);
    type Output = String;

    fn parse(input: &'a str) -> Self::Parsed {
        let move_pattern = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let (stacks, moves) = input.split_once("\n\n").unwrap();
        let stacks = stacks
            .lines()
            .flat_map(|containers| {
                containers
                    .chars()
                    .enumerate()
                    .filter(|&(i, _)| (i + 1) % 4 != 0)
                    .map(|(_, c)| c)
                    .array_chunks::<3>()
                    .map(|[_, mark, _]| {
                        if mark != ' ' {
                            Some(Container { mark })
                        } else {
                            None
                        }
                    })
            })
            .collect::<Vec<_>>();
        let stack_count = stacks.last().unwrap().unwrap().mark.to_digit(10).unwrap() as usize;
        let port = stacks.iter().enumerate().rev().skip(stack_count).fold(
            Port {
                stacks: vec![Stack::new(); stack_count],
            },
            |mut acc, (i, con)| {
                if let Some(container) = con {
                    acc.stacks[i % stack_count].push(*container);
                }
                acc
            },
        );

        let moves = moves
            .lines()
            .map(|mov| {
                let captures = move_pattern.captures(mov).unwrap();
                Move {
                    count: captures[1].parse::<u8>().unwrap(),
                    from: captures[2].parse::<u8>().unwrap() - 1,
                    to: captures[3].parse::<u8>().unwrap() - 1,
                }
            })
            .collect::<Vec<_>>();
        (port, moves)
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let (mut port, moves) = data;
        moves.iter().enumerate().map(|(_, x)| x).for_each(|mov| {
            mov.execute(&mut port);
        });
        let res = port
            .stacks
            .iter()
            .map(|stack| stack.last().unwrap().mark)
            .collect();
        res
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        let (mut port, moves) = data;
        moves.iter().enumerate().map(|(_, x)| x).for_each(|mov| {
            mov.execute_9001(&mut port);
        });
        let res = port
            .stacks
            .iter()
            .map(|stack| stack.last().unwrap().mark)
            .collect();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d5p1() {
        assert_eq!(
            Day5::part1(Day5::parse(
                "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            )),
            "CMZ".to_string()
        );
    }

    #[test]
    fn d5p2() {
        assert_eq!(
            Day5::part2(Day5::parse(
                "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            )),
            "MCD".to_string()
        );
    }
}
