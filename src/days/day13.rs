use crate::{solver::Solver, util::*};

pub struct Day13;

impl<'a> Solver<'a> for Day13 {
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
    fn d13p1() {
        assert_eq!(Day13::part1(Day13::parse("")), 0);
    }

    #[test]
    fn d13p2() {
        assert_eq!(Day13::part2(Day13::parse("")), 0);
    }
}