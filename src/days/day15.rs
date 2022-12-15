use crate::{solver::Solver, util::*};

pub struct Day15;

impl<'a> Solver<'a> for Day15 {
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
    fn d15p1() {
        assert_eq!(Day15::part1(Day15::parse("")), 0);
    }

    #[test]
    fn d15p2() {
        assert_eq!(Day15::part2(Day15::parse("")), 0);
    }
}
