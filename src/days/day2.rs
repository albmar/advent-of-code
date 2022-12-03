use crate::{solver::Solver, util::*};

pub struct Day2;

#[derive(Debug, Clone, Copy)]
pub enum Choice {
    Rock,
    Paper,
    Scissor,
}

impl TryFrom<&str> for Choice {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissor),
            x => Err(format!("`{x}` is not a valid choice")),
        }
    }
}

fn combine(a: Choice, b: Choice) -> u32 {
    a as u32 + 3 * b as u32
}

fn score(pair: &[Choice; 2]) -> u32 {
    let selected = pair[1] as u32 + 1;
    let c = combine(pair[0], pair[1]);

    let outcome = match c {
        x if x == combine(Choice::Rock, Choice::Paper)
            || x == combine(Choice::Paper, Choice::Scissor)
            || x == combine(Choice::Scissor, Choice::Rock) =>
        {
            6
        }
        x if x == combine(Choice::Rock, Choice::Rock)
            || x == combine(Choice::Paper, Choice::Paper)
            || x == combine(Choice::Scissor, Choice::Scissor) =>
        {
            3
        }
        _ => 0,
    };
    selected + outcome
}

impl<'a> Solver<'a> for Day2 {
    type Parsed = Vec<[Choice; 2]>;
    type Output = u32;

    fn parse(input: &'a str) -> Self::Parsed {
        input
            .split("\n")
            .flat_map(|x| x.splitn(2, " ").filter_map(|x| Choice::try_from(x).ok()))
            .array_chunks::<2>()
            .collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        data.iter().map(score).sum()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d2p1() {
        assert_eq!(
            Day2::part1(Day2::parse(
                "A Y
B X
C Z"
            )),
            15
        );
    }

    #[test]
    fn d2p2() {
        assert_eq!(Day2::part2(Day2::parse("")), 0);
    }
}
