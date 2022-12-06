use crate::{solver::Solver, util::*};

pub struct Day6;

impl<'a> Solver<'a> for Day6 {
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
    fn d6p1() {
        assert_eq!(Day6::part1(Day6::parse("")), 0);
    }

    #[test]
    fn d6p2() {
        assert_eq!(Day6::part2(Day6::parse("")), 0);
    }
}
