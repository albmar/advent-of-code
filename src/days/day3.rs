use crate::solver::Solver;

pub struct Day3;

fn get_priority(c: char) -> u32 {
    c as u32
        - if c.is_lowercase() {
            'a' as u32 - 1
        } else {
            'A' as u32 - 27
        }
}

impl<'a> Solver<'a> for Day3 {
    type Parsed = Vec<&'a str>;
    type Output = u32;

    fn parse(input: &'a str) -> Self::Parsed {
        input.split("\n").collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        data.into_iter()
            .map(|rucksack| {
                let (first, second) = rucksack.split_at(rucksack.len() / 2);
                first
                    .chars()
                    .filter(|&c| second.contains(c))
                    .map(get_priority)
                    .nth(0)
                    .unwrap()
            })
            .sum()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        data.into_iter()
            .array_chunks::<3>()
            .map(|group| {
                group[0]
                    .chars()
                    .filter(|&c| group[1].contains(c))
                    .filter(|&c| group[2].contains(c))
                    .map(get_priority)
                    .nth(0)
                    .unwrap()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('z'), 26);
        assert_eq!(get_priority('A'), 27);
        assert_eq!(get_priority('Z'), 52);
    }

    #[test]
    fn d3p1() {
        assert_eq!(
            Day3::part1(Day3::parse(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            )),
            157
        );
    }

    #[test]
    fn d3p2() {
        assert_eq!(
            Day3::part2(Day3::parse(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            )),
            70
        );
    }
}
