use std::ops::RangeInclusive;

use crate::{solver::Solver, util::*};

pub struct Day4;

impl<'a> Solver<'a> for Day4 {
    type Parsed = Vec<RangeInclusive<u8>>;
    type Output = u32;

    fn parse(input: &'a str) -> Self::Parsed {
        input
            .lines()
            .flat_map(|pair| {
                pair.splitn(4, &[',', '-'])
                    .map(|id| id.parse::<u8>().unwrap())
            })
            .array_chunks::<2>()
            .map(|range| range[0]..=range[1])
            .collect::<Vec<_>>()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        data.array_chunks::<2>()
            .filter(|[a, b]| {
                a.contains(b.start()) && a.contains(b.end())
                    || b.contains(a.start()) && b.contains(a.end())
            })
            .count() as u32
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        data.array_chunks::<2>()
            .filter(|[a, b]| {
                a.contains(b.start())
                    || a.contains(b.end())
                    || b.contains(a.start())
                    || b.contains(a.end())
            })
            .count() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d4p1() {
        assert_eq!(
            Day4::part1(Day4::parse(
                "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            )),
            2
        );
    }

    #[test]
    fn d4p2() {
        assert_eq!(
            Day4::part2(Day4::parse(
                "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            )),
            4
        );
    }
}
