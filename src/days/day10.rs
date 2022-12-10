use crate::{solver::Solver, util::*};

pub struct Day10;

#[derive(Debug, Clone, Copy)]
pub struct Command {
    cycle: u32,
    val: i32,
}

#[derive(Debug, Clone, Copy)]
struct State {
    cmd: Option<Command>,
    signal: u32,
    pos: u32,
    x: i32,
}

impl<'a> Solver<'a> for Day10 {
    type Parsed = Vec<Command>;
    type Output = i32;

    fn parse(input: &'a str) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                if line == "noop" {
                    Command { cycle: 1, val: 0 }
                } else {
                    Command {
                        cycle: 2,
                        val: line.split_once(" ").unwrap().1.parse().unwrap(),
                    }
                }
            })
            .collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let mut state = State {
            cmd: None,
            signal: 20,
            pos: 0,
            x: 1,
        };
        data.iter()
            .fold(vec![], |mut signal_strengths, cmd| {
                state.cmd = Some(*cmd);
                let new_pos = state.pos + cmd.cycle;
                if new_pos >= state.signal {
                    signal_strengths.push(state);
                    state.signal += 40;
                }
                state.x += cmd.val;
                state.pos = new_pos;
                signal_strengths
            })
            .iter()
            .map(|state| state.signal as i32 * state.x)
            .sum()
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
        assert_eq!(
            Day10::part1(Day10::parse(
                "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
            )),
            13140
        );
    }

    #[test]
    fn d10p2() {
        assert_eq!(Day10::part2(Day10::parse("")), 0);
    }
}
