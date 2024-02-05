use std::cmp::Ordering;

use crate::quad_tree::{Point, PointQuadtree};

const DIRECTIONS: [(i16, i16); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Clone)]
pub struct Board {
    time: u32,
    state: PointQuadtree<bool>,
}

impl Board {
    pub fn new() -> Self {
        Self::new_with_time(0)
    }

    fn new_with_time(time: u32) -> Self {
        Self {
            time,
            state: PointQuadtree::new(Point { x: 0, y: 0 }, false),
        }
    }

    pub fn set_cell(&mut self, position: Point, value: bool) {
        self.state.insert(position, value);
    }

    pub fn next_state(&self) -> Self {
        let mut counts = PointQuadtree::<u8>::new(Point { x: 0, y: 0 }, 0);

        for cell in self.cells() {
            for dir in DIRECTIONS {
                let position = Point {
                    x: (cell.x.wrapping_add(dir.0)),
                    y: (cell.y.wrapping_add(dir.1)),
                };

                match counts.get_mut(position) {
                    Some(count) => *count = count.wrapping_add(1),
                    None => counts.insert(position, 1),
                }
            }
        }

        let mut new_board = Self::new_with_time(self.time + 1);

        for (position, count) in counts.all_points() {
            if count == 0 {
                continue;
            }

            let value = match count.cmp(&2) {
                Ordering::Less => false,
                Ordering::Equal => self.state.get(position).unwrap_or(false),
                Ordering::Greater => count == 3,
            };

            if value {
                new_board.set_cell(position, true);
            }
        }

        new_board
    }

    pub fn cells(&self) -> impl Iterator<Item = Point> {
        self.state
            .all_points()
            .into_iter()
            .filter(|(_, value)| *value)
            .map(|(position, _)| position)
    }

    pub const fn time(&self) -> u32 {
        self.time
    }
}

impl From<&State> for Board {
    fn from(state: &State) -> Self {
        Self {
            time: state.time,
            state: state.points.as_slice().into(),
        }
    }
}

pub struct State {
    time: u32,
    points: Vec<Point>,
}

impl From<&Board> for State {
    fn from(board: &Board) -> Self {
        Self {
            time: board.time,
            points: board.cells().collect(),
        }
    }
}
