use crate::{solver::Solver, util::*};

pub struct Day6;

impl<'a> Solver<'a> for Day6 {
    type Parsed = Vec<char>;
    type Output = usize;

    fn parse(input: &'a str) -> Self::Parsed {
        input.trim().chars().collect::<Vec<char>>()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let mut set = HashSet::<char>::with_capacity(4);
        data.windows(4)
            .enumerate()
            .skip_while(|&(_, window)| {
                set.clear();
                set.extend(window);
                set.len() < 4
            })
            .map(|(i, _)| i + 4)
            .nth(0)
            .unwrap()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        let mut set = HashSet::<char>::with_capacity(14);
        data.windows(14)
            .enumerate()
            .skip_while(|&(_, window)| {
                set.clear();
                set.extend(window);
                set.len() < 14
            })
            .map(|(i, _)| i + 14)
            .nth(0)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d6p1() {
        assert_eq!(
            Day6::part1(Day6::parse("mjqjpqmgbljsphdztnvjfqwrcgsmlb")),
            7
        );
        assert_eq!(Day6::part1(Day6::parse("bvwbjplbgvbhsrlpgdmjqwftvncz")), 5);
        assert_eq!(Day6::part1(Day6::parse("nppdvjthqldpwncqszvftbrmjlhg")), 6);
        assert_eq!(
            Day6::part1(Day6::parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
            10
        );
        assert_eq!(
            Day6::part1(Day6::parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")),
            11
        );
    }

    #[test]
    fn d6p2() {
        assert_eq!(
            Day6::part2(Day6::parse("mjqjpqmgbljsphdztnvjfqwrcgsmlb")),
            19
        );
        assert_eq!(Day6::part2(Day6::parse("bvwbjplbgvbhsrlpgdmjqwftvncz")), 23);
        assert_eq!(Day6::part2(Day6::parse("nppdvjthqldpwncqszvftbrmjlhg")), 23);
        assert_eq!(
            Day6::part2(Day6::parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
            29
        );
        assert_eq!(
            Day6::part2(Day6::parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")),
            26
        );
    }
}
