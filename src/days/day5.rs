use crate::{solver::Solver, util::*};

pub struct Day5;

impl<'a> Solver<'a> for Day5 {
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
    fn d5p1() {
        assert_eq!(Day5::part1(Day5::parse("")), 0);
    }

    #[test]
    fn d5p2() {
        assert_eq!(Day5::part2(Day5::parse("")), 0);
    }
}
