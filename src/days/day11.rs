use std::{collections::VecDeque, num::ParseIntError, str::FromStr};

use crate::{solver::Solver, util::*};

pub struct Day11;

#[derive(Debug, Clone)]
pub struct Monkey {
    items: VecDeque<u32>,
    op: Operation,
    test: Test,
    inspected: u32,
}

#[derive(Debug, Clone, Copy)]
struct Operation(Exp, Op, Exp);

impl Operation {
    fn execute(&self, old: u32) -> u32 {
        let first = match self.0 {
            Exp::Old => old,
            Exp::Int(x) => x,
        };
        let second = match self.2 {
            Exp::Old => old,
            Exp::Int(x) => x,
        };
        match self.1 {
            Op::Add => first + second,
            Op::Mul => first * second,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Exp {
    Old,
    Int(u32),
}

impl FromStr for Exp {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Self::Old),
            _ => s.parse::<u32>().map(|x| Self::Int(x)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Mul),
            _ => Err(format!("{s} is not a valid operator")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Test {
    divisible: u32,
    true_case: u32,
    false_case: u32,
}

impl Test {
    fn test(&self, worry: u32) -> u32 {
        if worry % self.divisible == 0 {
            self.true_case
        } else {
            self.false_case
        }
    }
}

fn round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        let monkey = monkeys.get_mut(i).unwrap();
        let throws = monkey
            .items
            .iter()
            .map(|&item| monkey.op.execute(item) / 3)
            .map(|worry| (monkey.test.test(worry) as usize, worry))
            .collect::<Vec<_>>();
        monkey.inspected += throws.len() as u32;
        monkey.items.clear();
        throws
            .iter()
            .for_each(|&(monkey, worry)| monkeys[monkey].items.push_back(worry));
    }
}

impl<'a> Solver<'a> for Day11 {
    type Parsed = Vec<Monkey>;
    type Output = u32;

    fn parse(input: &'a str) -> Self::Parsed {
        input
            .split("\n\n")
            .flat_map(|monkey| {
                monkey
                    .splitn(6, "\n")
                    .map(|line| line.trim())
                    .enumerate()
                    .filter_map(|(i, attr)| match i {
                        0 => None,
                        1 => attr.strip_prefix("Starting items:"),
                        2 => attr.strip_prefix("Operation: new ="),
                        3 => attr.strip_prefix("Test: divisible by"),
                        4 => attr.strip_prefix("If true: throw to monkey"),
                        5 => attr.strip_prefix("If false: throw to monkey"),
                        _ => None,
                    })
                    .map(|line| line.trim())
                    .array_chunks::<5>()
                    .map(|[items, op, test, true_case, false_case]| {
                        let items = items
                            .split(", ")
                            .map(|num| num.parse::<u32>().unwrap())
                            .collect::<VecDeque<_>>();
                        let mut ops = op.splitn(3, " ");
                        let first = ops.next().unwrap().parse::<Exp>().unwrap();
                        let op = ops.next().unwrap().parse::<Op>().unwrap();
                        let second = ops.next().unwrap().parse::<Exp>().unwrap();
                        let op = Operation(first, op, second);
                        let test = Test {
                            divisible: test.parse().unwrap(),
                            true_case: true_case.parse().unwrap(),
                            false_case: false_case.parse().unwrap(),
                        };
                        Monkey {
                            items,
                            op,
                            test,
                            inspected: Default::default(),
                        }
                    })
            })
            .collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let mut monkeys = data.clone();
        (0..20).for_each(|_| round(&mut monkeys));
        let mut total_inspected = monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();
        total_inspected.sort_unstable_by(|a, b| b.cmp(a));
        total_inspected[0..2]
            .iter()
            .cloned()
            .reduce(|prod, ele| prod * ele)
            .unwrap()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d11p1() {
        assert_eq!(
            Day11::part1(Day11::parse(
                "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
            )),
            10605
        );
    }

    #[test]
    fn d11p2() {
        assert_eq!(Day11::part2(Day11::parse("")), 0);
    }
}
