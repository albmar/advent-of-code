use std::{cmp::Reverse, collections::BinaryHeap, fmt::Display, ops::Add, rc::Rc};

use grid::Grid;
use nalgebra::Vector2;
use ordered_float::OrderedFloat;

use crate::{solver::Solver, util::*};

pub struct Day12;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Cost(Reverse<OrderedFloat<f32>>);

impl Default for Cost {
    fn default() -> Self {
        Self(Reverse(f32::INFINITY.into()))
    }
}

impl Add for Cost {
    type Output = Cost;

    fn add(self, rhs: Self) -> Self::Output {
        (self.0 .0 .0 + rhs.0 .0 .0).into()
    }
}

impl From<f32> for Cost {
    fn from(value: f32) -> Self {
        Cost(Reverse(value.into()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
struct State {
    cost: Cost,
    heuristic: Cost,
    pos: Vector2<i16>,
    parent: Option<Rc<State>>,
}

impl State {
    fn new_start(pos: Vector2<i16>) -> Self {
        Self {
            pos,
            cost: 0f32.into(),
            heuristic: f32::INFINITY.into(),
            parent: None,
        }
    }

    fn new(pos: Vector2<i16>, cost: Cost, heuristic: Cost, parent: Rc<State>) -> Self {
        Self {
            pos,
            cost,
            heuristic,
            parent: Some(parent),
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.cost + self.heuristic;
        let b = other.cost + other.heuristic;
        a.cmp(&b)
    }
}

#[derive(Clone)]
pub struct Heightmap {
    start: Vector2<i16>,
    end: Vector2<i16>,
    goal: Option<Rc<dyn Fn(&Heightmap, &State) -> bool>>,
    valid_height: Option<Rc<dyn Fn(&Heightmap, Vector2<i16>, Vector2<i16>) -> bool>>,
    grid: Grid<Height>,
}

impl Heightmap {
    fn shortest_path(&mut self, start: State) -> Option<Vec<State>> {
        let mut visited = HashSet::new();
        let mut frontier = BinaryHeap::new();
        visited.insert(start.pos);
        frontier.push(Rc::new(start));
        loop {
            if frontier.is_empty() {
                return None;
            }
            let current = frontier.pop().unwrap();
            let goal = self.goal.as_ref().unwrap();
            if goal(self, &current) {
                return Some(self.solution(current));
            }
            frontier.extend(
                self.get_neighbours(current.pos)
                    .into_iter()
                    .filter(|&new_pos| visited.insert(new_pos))
                    .map(|new_pos| {
                        Rc::new(State::new(
                            new_pos,
                            self.cost(&current),
                            self.heuristic(new_pos),
                            current.clone(),
                        ))
                    }),
            );
        }
    }

    fn get_neighbours(&self, pos: Vector2<i16>) -> impl Iterator<Item = Vector2<i16>> + '_ {
        let min = Vector2::<i16>::new(0, 0);
        let max = Vector2::<i16>::new(self.grid.cols() as i16, self.grid.rows() as i16);
        let neighbours = [
            Vector2::<i16>::new(-1, 0),
            Vector2::<i16>::new(0, -1),
            Vector2::<i16>::new(0, 1),
            Vector2::<i16>::new(1, 0),
        ];
        let is_valied = self.valid_height.as_ref().unwrap();
        neighbours
            .into_iter()
            .map(move |v| pos + v)
            .filter(move |&new_pos| new_pos >= min && new_pos < max)
            .filter(move |&new_pos| is_valied(self, new_pos, pos))
    }

    fn height(&self, pos: Vector2<i16>) -> i16 {
        self.grid
            .get(pos.y as usize, pos.x as usize)
            .unwrap_or_else(|| panic!("{pos} not in grid"))
            .value()
    }

    fn cost(&self, current: &State) -> Cost {
        current.cost + 1f32.into()
    }

    fn heuristic(&self, current: Vector2<i16>) -> Cost {
        (current - self.end).abs().cast::<f32>().sum().into()
    }

    fn solution(&self, current: Rc<State>) -> Vec<State> {
        let mut path = Vec::new();
        let mut current = current;
        path.push(current.as_ref().clone());
        loop {
            if let Some(parent) = current.parent.clone() {
                current = parent;
                path.push(current.as_ref().clone());
            } else {
                break;
            }
        }
        path.reverse();
        path
    }
}

impl Display for Heightmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in 0..self.grid.rows() {
            writeln!(
                f,
                "{}",
                self.grid.iter_row(row).map(|h| h.0).collect::<String>()
            )?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct Height(char);

impl Height {
    fn value(&self) -> i16 {
        let v = match self.0 {
            'S' => 'a',
            'E' => 'z',
            x => x,
        };
        v as i16 - 'a' as i16
    }

    fn is_start(&self) -> bool {
        self.0 == 'S'
    }
    fn is_end(&self) -> bool {
        self.0 == 'E'
    }
}

impl From<char> for Height {
    fn from(value: char) -> Self {
        Self(value)
    }
}

impl Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> Solver<'a> for Day12 {
    type Parsed = Heightmap;
    type Output = u32;

    fn parse(input: &'a str) -> Self::Parsed {
        let heights = input
            .lines()
            .flat_map(|h| h.chars().map(Height::from))
            .collect::<Vec<_>>();
        let grid = Grid::from_vec(
            heights,
            input.lines().nth(0).map(|line| line.len()).unwrap(),
        );
        let start = grid
            .iter()
            .position(|h| h.is_start())
            .map(|i| Vector2::new(i % grid.cols(), i / grid.cols()).cast())
            .unwrap();
        let end = grid
            .iter()
            .position(|h| h.is_end())
            .map(|i| Vector2::new(i % grid.cols(), i / grid.cols()).cast())
            .unwrap();
        Heightmap {
            start,
            end,
            grid,
            goal: None,
            valid_height: None,
        }
    }

    fn part1(mut data: Self::Parsed) -> Self::Output {
        let start = State::new_start(data.start);
        data.goal = Some(Rc::new(|map, current| current.pos == map.end));
        data.valid_height = Some(Rc::new(|map, new_pos, pos| {
            map.height(new_pos) - map.height(pos) <= 1
        }));
        let path = data.shortest_path(start);
        path.inspect(|path| {
            path.iter().for_each(|state| {
                if let Some(cell) = data
                    .grid
                    .get_mut(state.pos.y as usize, state.pos.x as usize)
                {
                    *cell = cell.0.to_ascii_uppercase().into()
                }
            });
        })
        .map(|path| path.len() as u32 - 1)
        .unwrap_or_else(|| panic!("No solution found"))
    }

    fn part2(mut data: Self::Parsed) -> Self::Output {
        let start = State::new_start(data.end);
        data.goal = Some(Rc::new(|map, current| {
            map.height(current.pos) == Height('a').value()
        }));
        data.valid_height = Some(Rc::new(|map, new_pos, pos| {
            map.height(pos) - map.height(new_pos) <= 1
        }));
        let path = data.shortest_path(start);
        path.inspect(|path| {
            path.iter().for_each(|state| {
                if let Some(cell) = data
                    .grid
                    .get_mut(state.pos.y as usize, state.pos.x as usize)
                {
                    *cell = cell.0.to_ascii_uppercase().into()
                }
            });
        })
        .map(|path| path.len() as u32 - 1)
        .unwrap_or_else(|| panic!("No solution found"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cost_order() {
        let costs = [1.0, 3.0, 5.0, 4.0, 2.0, 0.0].map(Cost::from);
        let mut frontier = BinaryHeap::from_iter(costs);
        assert_eq!(frontier.pop(), Some(Cost::from(0f32)));
        assert_eq!(frontier.pop(), Some(Cost::from(1f32)));
        assert_eq!(frontier.pop(), Some(Cost::from(2f32)));
        assert_eq!(frontier.pop(), Some(Cost::from(3f32)));
        assert_eq!(frontier.pop(), Some(Cost::from(4f32)));
        assert_eq!(frontier.pop(), Some(Cost::from(5f32)));
    }

    #[test]
    fn cost_add() {
        assert_eq!(Cost::from(5.0) + Cost::from(5.0), Cost::from(10.0));
        assert_eq!(Cost::from(-5.0) + Cost::from(5.0), Cost::from(0.0));
        assert_eq!(
            Cost::from(5.0) + Cost::from(f32::INFINITY),
            Cost::from(f32::INFINITY)
        );
    }

    #[test]
    fn d12p1() {
        assert_eq!(
            Day12::part1(Day12::parse(
                "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
            )),
            31
        );
    }

    #[test]
    fn d12p2() {
        assert_eq!(
            Day12::part2(Day12::parse(
                "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
            )),
            29
        );
    }
}
