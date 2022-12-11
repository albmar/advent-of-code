use crate::{solver::Solver, util::*};

pub struct Day11;

impl<'a> Solver<'a> for Day11 {
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
    fn d11p1() {
        assert_eq!(Day11::part1(Day11::parse("")), 0);
    }

    #[test]
    fn d11p2() {
        assert_eq!(Day11::part2(Day11::parse("")), 0);
    }
}
