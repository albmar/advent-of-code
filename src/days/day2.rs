use crate::{solver::Solver, util::*};

pub struct Day2;

impl<'a> Solver<'a> for Day2 {
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
    fn d2p1() {
        assert_eq!(Day2::part1(Day2::parse("")), 0);
    }

    #[test]
    fn d2p2() {
        assert_eq!(Day2::part2(Day2::parse("")), 0);
    }
}
