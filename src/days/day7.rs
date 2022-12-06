use crate::{solver::Solver, util::*};

pub struct Day7;

impl<'a> Solver<'a> for Day7 {
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
    fn d7p1() {
        assert_eq!(Day7::part1(Day7::parse("")), 0);
    }

    #[test]
    fn d7p2() {
        assert_eq!(Day7::part2(Day7::parse("")), 0);
    }
}
