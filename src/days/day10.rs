use crate::{solver::Solver, util::*};

pub struct Day10;

impl<'a> Solver<'a> for Day10 {
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
    fn d10p1() {
        assert_eq!(Day10::part1(Day10::parse("")), 0);
    }

    #[test]
    fn d10p2() {
        assert_eq!(Day10::part2(Day10::parse("")), 0);
    }
}
