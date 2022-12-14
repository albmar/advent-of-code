use crate::{solver::Solver, util::*};

pub struct Day14;

impl<'a> Solver<'a> for Day14 {
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
    fn d14p1() {
        assert_eq!(Day14::part1(Day14::parse("")), 0);
    }

    #[test]
    fn d14p2() {
        assert_eq!(Day14::part2(Day14::parse("")), 0);
    }
}
