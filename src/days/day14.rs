use grid::Grid;
use nalgebra::Vector2;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::{solver::Solver, util::*};

pub struct Day14;

fn parse_input<'a>(input: &'a str) -> IResult<&'a str, Vec<Vec<Vector2<u16>>>> {
    separated_list1(
        newline,
        separated_list1(
            tag(" -> "),
            map(
                separated_pair(complete::u16, tag(","), complete::u16),
                |(x, y)| Vector2::new(x, y),
            ),
        ),
    )(input)
}

impl<'a> Solver<'a> for Day14 {
    type Parsed = Vec<Vec<Vector2<u16>>>;
    type Output = usize;

    fn parse(input: &'a str) -> Self::Parsed {
        parse_input(input).unwrap().1
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let (min, max) = data.iter().flat_map(|lines| lines.iter()).fold(
            (Vector2::repeat(u16::MAX), Vector2::repeat(u16::MIN)),
            |(min, max), cur| (min.inf(cur), max.sup(cur)),
        );
        let sand_source = Vector2::new(500, 0);
        let (min, max) = (
            min.cast::<usize>().inf(&sand_source),
            max.cast::<usize>().sup(&sand_source),
        );
        let size = max - min + Vector2::repeat(1);
        let mut grid = Grid::<char>::init(size.y as usize, size.x as usize, '.');
        data.iter()
            .flat_map(|lines| {
                lines
                    .array_windows::<2>()
                    .map(|[start, end]| {
                        let min_x = start.x.min(end.x);
                        let max_x = start.x.max(end.x);
                        let min_y = start.y.min(end.y);
                        let max_y = start.y.max(end.y);
                        [min_x..=max_x, min_y..=max_y]
                    })
                    .flat_map(|[x_range, y_range]| {
                        x_range.flat_map(move |x| y_range.clone().map(move |y| Vector2::new(x, y)))
                    })
            })
            .map(|rock| (rock.cast::<usize>() - min))
            .for_each(|rock| {
                if let Some(c) = grid.get_mut(rock.y, rock.x) {
                    *c = '#'
                }
            });
        let sand_source = sand_source - min;
        if let Some(c) = grid.get_mut(sand_source.y, sand_source.x) {
            *c = '+'
        }
        (0..)
            .map(|_| sand_source.cast::<isize>())
            .map(|mut coord| {
                loop {
                    if coord.x < 0
                        || coord.y < 0
                        || coord.x >= size.x as isize
                        || coord.y >= size.y as isize
                    {
                        break;
                    }
                    let down = grid
                        .iter_col(coord.x as usize)
                        .skip(coord.y as usize)
                        .take_while(|&&c| c == '.' || c == '+')
                        .count()
                        - 1;
                    coord.y += down as isize;
                    if grid
                        .get((coord.y + 1) as usize, (coord.x - 1) as usize)
                        .map_or(true, |&c| c == '.' || c == '+')
                    {
                        coord.x -= 1;
                        coord.y += 1;
                        continue;
                    }
                    if grid
                        .get((coord.y + 1) as usize, (coord.x + 1) as usize)
                        .map_or(true, |&c| c == '.' || c == '+')
                    {
                        coord.x += 1;
                        coord.y += 1;
                        continue;
                    }
                    if down == 0 {
                        if let Some(c) = grid.get_mut(coord.y as usize, coord.x as usize) {
                            *c = 'o'
                        }
                        break;
                    }
                }
                coord
            })
            .take_while(|coord| {
                coord.x >= 0
                    && coord.y >= 0
                    && coord.x < size.x as isize
                    && coord.y < size.y as isize
            })
            .count()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        todo!()
    }
}

fn print_grid(grid: &Grid<char>) {
    print!(
        "{}",
        (0..grid.rows())
            .flat_map(|row| grid.iter_row(row).chain(['\n'].iter()))
            .collect::<String>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d14p1() {
        assert_eq!(
            Day14::part1(Day14::parse(
                "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
            )),
            24
        );
    }

    #[test]
    fn d14p2() {
        assert_eq!(
            Day14::part2(Day14::parse(
                "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
            )),
            0
        );
    }
}
