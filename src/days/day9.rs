use crate::{solver::Solver, util::*};

pub struct Day9;

impl<'a> Solver<'a> for Day9 {
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
    fn d9p1() {
        assert_eq!(Day9::part1(Day9::parse("")), 0);
    }

    #[test]
    fn d9p2() {
        assert_eq!(Day9::part2(Day9::parse("")), 0);
    }
}
