use crate::{solver::Solver, util::*};

pub struct Day2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    Rock,
    Paper,
    Scissor,
}

impl TryFrom<u32> for Choice {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Rock),
            1 => Ok(Self::Paper),
            2 => Ok(Self::Scissor),
            x => Err(format!("`{x}` is not a valid choice.")),
        }
    }
}

impl Choice {
    fn winner(self) -> Self {
        ((self as u32 + 1) % 3).try_into().unwrap()
    }

    fn looser(self) -> Self {
        ((self as u32 + 2) % 3).try_into().unwrap()
    }

    fn looses(self, other: Choice) -> bool {
        other == self.winner()
    }

    fn value(self) -> u32 {
        self as u32 + 1
    }
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

fn score(pair: &[Choice; 2]) -> u32 {
    let [a, b] = pair.to_owned();
    let selected = b.value();
    let outcome = if b.looses(a) {
        0
    } else if b == a {
        3
    } else {
        6
    };
    selected + outcome
}

impl<'a> Solver<'a> for Day2 {
    type Parsed = Vec<[&'a str; 2]>;
    type Output = u32;

    fn parse(input: &'a str) -> Self::Parsed {
        input
            .split("\n")
            .flat_map(|x| x.splitn(2, " "))
            .array_chunks::<2>()
            .collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        data.iter()
            .map(|chunk| chunk.map(|x| Choice::try_from(x).unwrap()))
            .map(|chunk| score(&chunk))
            .sum()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        data.iter()
            .map(|chunk| {
                let a = Choice::try_from(chunk[0]).unwrap();
                match chunk[1] {
                    "X" => [a, a.looser()],
                    "Z" => [a, a.winner()],
                    _ => [a, a],
                }
            })
            .map(|chunk| score(&chunk))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choice() {
        assert_eq!(Choice::Rock.value(), 1);
        assert_eq!(Choice::Paper.value(), 2);
        assert_eq!(Choice::Scissor.value(), 3);
        assert_eq!(Choice::Rock, Choice::Paper.looser());
        assert_eq!(Choice::Rock, Choice::Scissor.winner());
        assert_eq!(Choice::Scissor, Choice::Rock.looser());
        assert_eq!(Choice::Scissor, Choice::Paper.winner());
        assert_eq!(Choice::Paper, Choice::Rock.winner());
        assert_eq!(Choice::Paper, Choice::Scissor.looser());
        assert!(
            !Choice::Rock.looses(Choice::Rock)
                && !Choice::Rock.looses(Choice::Rock.looser())
                && Choice::Rock.looses(Choice::Rock.winner())
        );
        assert!(
            !Choice::Paper.looses(Choice::Paper)
                && !Choice::Paper.looses(Choice::Paper.looser())
                && Choice::Paper.looses(Choice::Paper.winner())
        );
        assert!(
            !Choice::Paper.looses(Choice::Paper)
                && !Choice::Paper.looses(Choice::Paper.looser())
                && Choice::Paper.looses(Choice::Paper.winner())
        );
    }

    #[test]
    fn test_score() {
        [Choice::Rock, Choice::Paper, Choice::Scissor]
            .iter()
            .for_each(|&choice| {
                assert_eq!(
                    score(&[choice, choice.looser()]),
                    choice.looser().value() + 0
                );
                assert_eq!(score(&[choice, choice]), choice.value() + 3);
                assert_eq!(
                    score(&[choice, choice.winner()]),
                    choice.winner().value() + 6
                );
            });
    }

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
        assert_eq!(
            Day2::part2(Day2::parse(
                "A Y
B X
C Z"
            )),
            12
        );
    }
}
