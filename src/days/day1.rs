use crate::{solver::Solver, util::*};

pub struct Day1;

impl<'a> Solver<'a> for Day1 {
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
    fn d1p1() {
        assert_eq!(Day1::part1(Day1::parse("")), 0);
    }

    #[test]
    fn d1p2() {
        assert_eq!(Day1::part2(Day1::parse("")), 0);
    }
}
