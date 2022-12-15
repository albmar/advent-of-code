use nalgebra::Vector2;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

use crate::{solver::Solver, util::*};

pub struct Day15;

fn parse_input<'a>(i: &'a str) -> IResult<&'a str, Vec<(Vector2<i32>, Vector2<i32>)>> {
    separated_list1(
        newline,
        separated_pair(
            map(
                separated_pair(
                    preceded(tag("Sensor at x="), complete::i32),
                    tag(", y="),
                    complete::i32,
                ),
                |pair| Vector2::new(pair.0, pair.1),
            ),
            tag(": closest beacon is at x="),
            map(
                separated_pair(complete::i32, tag(", y="), complete::i32),
                |pair| Vector2::new(pair.0, pair.1),
            ),
        ),
    )(i)
}

impl<'a> Solver<'a> for Day15 {
    type Parsed = Vec<(Vector2<i32>, Vector2<i32>)>;
    type Output = usize;

    fn parse(input: &'a str) -> Self::Parsed {
        parse_input(input).unwrap().1
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let sensor_distances = data
            .iter()
            .map(|(sensor, beacon)| (sensor, (sensor - beacon).abs().sum()))
            .collect::<Vec<_>>();
        let min_x = sensor_distances.iter().map(|(s, d)| s.x - d).min().unwrap();
        let max_x = sensor_distances.iter().map(|(s, d)| s.x + d).max().unwrap();
        let y = if data.first().unwrap().0.x == 2 {
            10
        } else {
            2_000_000
        };
        (min_x..=max_x)
            .map(move |x| Vector2::new(x, y))
            .filter(|v| data.iter().all(|(s, b)| v != s && v != b))
            .filter(|v| {
                sensor_distances
                    .iter()
                    .any(|&(s, d)| (v - s).abs().sum() <= d)
            })
            .count()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        let sensor_distances = data
            .iter()
            .map(|(sensor, beacon)| (sensor, (sensor - beacon).abs().sum()))
            .collect::<Vec<_>>();
        let max = if data.first().unwrap().0.x == 2 {
            20
        } else {
            4_000_000
        };
        let min = Vector2::repeat(0);
        let max = Vector2::repeat(max);
        sensor_distances
            .iter()
            .map(|(s, d)| (s, d + 1))
            .flat_map(|(&&s, d)| {
                (0..d).flat_map(move |x| {
                    [
                        Vector2::new(x, -(d - x)) + s,
                        Vector2::new(d - x, x) + s,
                        Vector2::new(-x, d - x) + s,
                        Vector2::new(-(d - x), x) + s,
                    ]
                })
            })
            .filter(|&v| v >= min && v <= max)
            .filter(|v| {
                sensor_distances
                    .iter()
                    .all(|&(s, d)| (v - s).abs().sum() > d)
            })
            .last()
            .map(|v| v.x as usize * 4_000_000 + v.y as usize)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d15p1() {
        assert_eq!(
            Day15::part1(Day15::parse(
                "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"
            )),
            26
        );
    }

    #[test]
    fn d15p2() {
        assert_eq!(
            Day15::part2(Day15::parse(
                "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"
            )),
            56000011
        );
    }
}
