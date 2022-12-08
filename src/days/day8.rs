use crate::{solver::Solver, util::*};

pub struct Day8;

impl<'a> Solver<'a> for Day8 {
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
    fn d8p1() {
        assert_eq!(Day8::part1(Day8::parse("")), 0);
    }

    #[test]
    fn d8p2() {
        assert_eq!(Day8::part2(Day8::parse("")), 0);
    }
}
