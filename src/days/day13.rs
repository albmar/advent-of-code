use std::{cmp::Ordering, collections::BTreeSet, fmt::Display};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u8},
    multi::{separated_list0, separated_list1},
    sequence::separated_pair,
    IResult,
};

use crate::{solver::Solver, util::*};

pub struct Day13;

#[derive(Debug, Clone, Eq)]
pub enum Packet {
    Int(u8),
    List(Vec<Packet>),
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Int(i) => write!(f, "{i}"),
            Packet::List(list) => {
                write!(f, "[")?;
                let mut iter = list.iter();
                if let Some(p) = iter.next() {
                    p.fmt(f)?;
                }
                for p in iter {
                    write!(f, ",")?;
                    p.fmt(f)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl From<u8> for Packet {
    fn from(value: u8) -> Self {
        Self::Int(value)
    }
}

impl From<Vec<Packet>> for Packet {
    fn from(value: Vec<Packet>) -> Self {
        Self::List(value)
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a == b,
            (Self::List(a), Self::List(b)) => a == b,
            (a, Packet::List(b)) => b.len() == 1 && a == &b[0],
            (Packet::List(a), b) => a.len() == 1 && b == &a[0],
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a.cmp(b),
            (Self::List(a), Self::List(b)) => {
                for ord in a.iter().zip(b.iter()).map(|(a, b)| a.cmp(b)) {
                    match ord {
                        Ordering::Equal => (),
                        _ => return ord,
                    }
                }
                return a.len().cmp(&b.len());
            }
            (a, Packet::List(b)) => {
                if let Some(b) = b.first() {
                    match a.cmp(b) {
                        Ordering::Equal => (),
                        ord => return ord,
                    }
                }
                return 1.cmp(&b.len());
            }
            (Packet::List(a), b) => {
                if let Some(a) = a.first() {
                    match a.cmp(b) {
                        Ordering::Equal => (),
                        ord => return ord,
                    }
                }
                return a.len().cmp(&1);
            }
        }
    }
}

fn parse_list<'a>(i: &'a str) -> IResult<&'a str, Packet> {
    let (i, _) = tag("[")(i)?;
    let (i, list) = separated_list0(tag(","), parse_packet)(i)?;
    let (i, _) = tag("]")(i)?;
    Ok((i, list.into()))
}

fn parse_int<'a>(i: &'a str) -> IResult<&'a str, Packet> {
    let (i, res) = u8(i)?;
    Ok((i, res.into()))
}

fn parse_packet<'a>(i: &'a str) -> IResult<&'a str, Packet> {
    alt((parse_int, parse_list))(i)
}

fn parse_pair<'a>(i: &'a str) -> IResult<&'a str, (Packet, Packet)> {
    separated_pair(parse_packet, newline, parse_packet)(i)
}

fn parse_input<'a>(i: &'a str) -> IResult<&'a str, Vec<(Packet, Packet)>> {
    separated_list1(tag("\n\n"), parse_pair)(i)
}

impl<'a> Solver<'a> for Day13 {
    type Parsed = Vec<(Packet, Packet)>;
    type Output = usize;

    fn parse(input: &'a str) -> Self::Parsed {
        parse_input(input).unwrap().1
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let right_ordered = data
            .iter()
            .enumerate()
            .map(|(i, x)| (i + 1, x))
            .filter(|(_, (left, right))| left < right)
            .collect::<Vec<_>>();
        right_ordered.iter().map(|(i, _)| i).sum()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        let mut packets = data
            .into_iter()
            .flat_map(|pair| [pair.0, pair.1])
            .collect::<BTreeSet<_>>();
        let first = parse_packet("[[2]]").unwrap().1;
        let second = parse_packet("[[6]]").unwrap().1;
        packets.insert(first.clone());
        packets.insert(second.clone());
        let first = dbg!(packets.iter().position(|x| x == &first)).unwrap() + 1;
        let second = dbg!(packets.iter().position(|x| x == &second)).unwrap() + 1;
        first * second
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    #[test]
    fn ord_int() {
        let left = parse_packet("0").unwrap().1;
        let right = parse_packet("1").unwrap().1;
        assert_eq!(left.cmp(&right), Ordering::Less);
        let left = parse_packet("1").unwrap().1;
        assert_eq!(left.cmp(&right), Ordering::Equal);
        let left = parse_packet("2").unwrap().1;
        assert_eq!(left.cmp(&right), Ordering::Greater);
    }
    #[test]
    fn ord_int_and_list() {
        let left = parse_packet("0").unwrap().1;
        let right = parse_packet("[1]").unwrap().1;
        assert_eq!(left.cmp(&right), Ordering::Less);
        let left = parse_packet("1").unwrap().1;
        assert_eq!(left.cmp(&right), Ordering::Equal);
        let left = parse_packet("2").unwrap().1;
        assert_eq!(left.cmp(&right), Ordering::Greater);
    }

    #[test]
    fn ord_int_and_empty_list() {
        let left = parse_packet("0").unwrap().1;
        let right = parse_packet("[]").unwrap().1;
        assert_eq!(left.cmp(&right), Ordering::Greater);
    }

    #[test]
    fn ord_int_and_larger_list() {
        let left = parse_packet("0").unwrap().1;
        let right = parse_packet("[1,0]").unwrap().1;
        assert_eq!(left.cmp(&right), Ordering::Less);
        let left = parse_packet("1").unwrap().1;
        assert_eq!(left.cmp(&right), Ordering::Less);
        let left = parse_packet("[2]").unwrap().1;
        assert_eq!(left.cmp(&right), Ordering::Greater);
    }

    #[test]
    fn ord_equal_sized_lists() {
        let left = parse_packet("[0]").unwrap().1;
        let right = parse_packet("[1]").unwrap().1;
        assert_eq!(left.cmp(&right), Ordering::Less);
        let left = parse_packet("[1]").unwrap().1;
        assert_eq!(left.cmp(&right), Ordering::Equal);
        let left = parse_packet("[2]").unwrap().1;
        assert_eq!(left.cmp(&right), Ordering::Greater);
    }

    #[test]
    fn ord_larger_right_list() {
        let left = parse_packet("[0]").unwrap().1;
        let right = parse_packet("[1,0]").unwrap().1;
        assert_eq!(left.cmp(&right), Ordering::Less);
        let left = parse_packet("[1]").unwrap().1;
        assert_eq!(left.cmp(&right), Ordering::Less);
        let left = parse_packet("[2]").unwrap().1;
        assert_eq!(left.cmp(&right), Ordering::Greater);
    }
    #[test]
    fn ord_larger_left_list() {
        let left = parse_packet("[0,0]").unwrap().1;
        let right = parse_packet("[1]").unwrap().1;
        assert_eq!(left.cmp(&right), Ordering::Less);
        let left = parse_packet("[1,0]").unwrap().1;
        assert_eq!(left.cmp(&right), Ordering::Greater);
        let left = parse_packet("[2,0]").unwrap().1;
        assert_eq!(left.cmp(&right), Ordering::Greater);
    }

    #[test]
    fn ord_list_of_lists() {
        let left = parse_packet("[[1],[2,3,4]]").unwrap().1;
        let right = parse_packet("[[1],4]").unwrap().1;
        assert_eq!(left.cmp(&right), Ordering::Less);
    }

    #[test]
    fn d13p1() {
        assert_eq!(
            Day13::part1(Day13::parse(
                "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
            )),
            13
        );
    }

    #[test]
    fn d13p2() {
        assert_eq!(
            Day13::part2(Day13::parse(
                "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
            )),
            140
        );
    }
}
