use lexical_core::parse;

use crate::{solver::Solver, util::*};

pub struct Day1;

#[derive(Debug, Clone)]
pub struct Parsed {
    elves: Vec<usize>,
    calories: Vec<u32>,
}

impl<'a> Solver<'a> for Day1 {
    type Parsed = Parsed;
    type Output = u32;

    fn parse(input: &'a str) -> Self::Parsed {
        let mut elves = Vec::new();
        let calories = input
            .split("\n")
            .enumerate()
            .filter_map(|(i, cal)| {
                let val = cal.parse::<u32>().ok();
                if cal.is_empty() {
                    elves.push(i - elves.len());
                }
                val
            })
            .collect::<Vec<_>>();
        elves.push(calories.len());
        Parsed {
            elves,
            calories: calories,
        }
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let mut i = 0;
        data.elves
            .iter()
            .map(|&end| {
                let sum = data.calories[i..end].iter().sum::<u32>();
                i = end;
                sum
            })
            .max()
            .unwrap()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        let mut i = 0;
        let mut calories = data
            .elves
            .iter()
            .map(|&end| {
                let sum = data.calories[i..end].iter().sum::<u32>();
                i = end;
                sum
            })
            .collect::<Vec<_>>();
        calories.sort_unstable();
        calories.reverse();
        calories[..3].iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d1p1() {
        assert_eq!(
            Day1::part1(Day1::parse(
                "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
            )),
            24000
        );
    }

    #[test]
    fn d1p2() {
        assert_eq!(
            Day1::part2(Day1::parse(
                "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
            )),
            45000
        );
    }
}
