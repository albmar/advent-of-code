use crate::{solver::Solver, util::*};

pub struct Day12;

impl<'a> Solver<'a> for Day12 {
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
    fn d12p1() {
        assert_eq!(Day12::part1(Day12::parse("")), 0);
    }

    #[test]
    fn d12p2() {
        assert_eq!(Day12::part2(Day12::parse("")), 0);
    }
}
