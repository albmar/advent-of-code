use crate::{solver::Solver, util::*};

pub struct Day4;

impl<'a> Solver<'a> for Day4 {
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
    fn d4p1() {
        assert_eq!(Day4::part1(Day4::parse("")), 0);
    }

    #[test]
    fn d4p2() {
        assert_eq!(Day4::part2(Day4::parse("")), 0);
    }
}
