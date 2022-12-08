use grid::Grid;

use crate::{solver::Solver, util::*};

pub struct Day8;

#[derive(Debug, Default, Clone, Copy)]
pub struct Tree {
    x: u8,
    y: u8,
    height: u8,
}

impl Tree {
    fn is_visible(&self, grid: &Grid<Self>) -> bool {
        let x = self.x as usize;
        let y = self.y as usize;
        x == 0
            || x == grid.cols() - 1
            || y == 0
            || y == grid.rows() - 1
            || grid
                .iter_row(y)
                .take(x)
                .all(|tree| tree.height < self.height)
            || grid
                .iter_row(y)
                .skip(x + 1)
                .all(|tree| tree.height < self.height)
            || grid
                .iter_col(x)
                .take(y)
                .all(|tree| tree.height < self.height)
            || grid
                .iter_col(x)
                .skip(y + 1)
                .all(|tree| tree.height < self.height)
    }

    fn scenic_score(&self, grid: &Grid<Self>) -> usize {
        let x = self.x as usize;
        let y = self.y as usize;
        let cols = grid.cols();
        let rows = grid.rows();
        let left = grid
            .iter_row(y)
            .take(x)
            .rev()
            .take_while(|tree| tree.height < self.height)
            .last()
            .map_or(0, |tree| {
                let i = tree.x as usize;
                x.abs_diff(i) + (i > 0) as usize
            });
        let right = grid
            .iter_row(y)
            .skip(x + 1)
            .take_while(|tree| tree.height < self.height)
            .last()
            .map_or(0, |tree| {
                let i = tree.x as usize;
                x.abs_diff(i) + (i < cols - 1) as usize
            });
        let up = grid
            .iter_col(x)
            .take(y)
            .rev()
            .take_while(|tree| tree.height < self.height)
            .last()
            .map_or(0, |tree| {
                let i = tree.y as usize;
                y.abs_diff(i) + (i > 0) as usize
            });
        let down = grid
            .iter_col(x)
            .skip(y + 1)
            .take_while(|tree| tree.height < self.height)
            .last()
            .map_or(0, |tree| {
                let i = tree.y as usize;
                y.abs_diff(i) + (i < rows - 1) as usize
            });
        left * right * up * down
    }
}

impl From<(usize, usize, char)> for Tree {
    fn from(value: (usize, usize, char)) -> Self {
        Tree {
            x: value.0 as u8,
            y: value.1 as u8,
            height: value.2.to_digit(10).unwrap() as u8,
        }
    }
}

impl<'a> Solver<'a> for Day8 {
    type Parsed = Grid<Tree>;
    type Output = usize;

    fn parse(input: &'a str) -> Self::Parsed {
        let lines = input.lines();
        let columns = lines.clone().nth(0).unwrap().len();
        Grid::<Tree>::from_vec(
            lines
                .enumerate()
                .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (x, y, c).into()))
                .collect::<Vec<_>>(),
            columns,
        )
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        data.iter().filter(|tree| tree.is_visible(&data)).count()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        data.iter()
            .map(|tree| tree.scenic_score(&data))
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d8p1() {
        assert_eq!(
            Day8::part1(Day8::parse(
                "30373
25512
65332
33549
35390"
            )),
            21
        );
    }

    #[test]
    fn d8p2() {
        assert_eq!(
            Day8::part2(Day8::parse(
                "30373
25512
65332
33549
35390"
            )),
            8
        );
    }
}
