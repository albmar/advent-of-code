use crate::{solver::Solver, util::*};

pub struct Day3;

impl<'a> Solver<'a> for Day3 {
    type Parsed = u32;
    type Output = u32;

    fn parse(input: &'a str) -> Self::Parsed {
        todo!()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        todo!()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d3p1() {
        assert_eq!(Day3::part1(Day3::parse("")), 0);
    }

    #[test]
    fn d3p2() {
        assert_eq!(Day3::part2(Day3::parse("")), 0);
    }
}
