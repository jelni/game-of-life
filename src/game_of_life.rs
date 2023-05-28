use std::cmp::Ordering;

use crate::quad_tree::QuadTree;

const DIRECTIONS: [(usize, usize); 8] = [
    (0, 0),
    (0, 1),
    (0, 2),
    (1, 0),
    (1, 2),
    (2, 0),
    (2, 1),
    (2, 2),
];

pub struct Board {
    state: QuadTree<bool>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        Self {
            state: QuadTree::new(size, (0, 0), false),
        }
    }

    pub fn set_cell(&mut self, position: (usize, usize), value: bool) {
        self.state.insert(position, value);
    }

    pub fn next_state(&self, new_size: usize) -> Self {
        let mut counts = QuadTree::new(new_size, (0, 0), 0);

        for (x, y) in self.cells() {
            for dir in DIRECTIONS {
                if dir.0 == 0 && x == 0
                    || dir.1 == 0 && y == 0
                    || dir.0 == 2 && x == self.state.size() - 1
                    || dir.1 == 2 && y == self.state.size() - 1
                {
                    continue;
                }

                let position = ((x + dir.0 - 1), (y + dir.1 - 1));

                if position.0 > new_size || position.1 > new_size {
                    continue;
                }

                let count = counts.get(position).unwrap_or(0);
                counts.insert(position, count + 1);
            }
        }

        let mut new_board = Board::new(new_size);

        for ((x, y), count) in counts.all_nodes() {
            if count == 0 {
                continue;
            }

            let value = match count.cmp(&2) {
                Ordering::Less => false,
                Ordering::Equal => self.state.get((x, y)).unwrap_or(false),
                Ordering::Greater => count == 3,
            };

            new_board.set_cell((x, y), value);
        }

        new_board
    }

    pub fn cells(&self) -> impl Iterator<Item = (usize, usize)> {
        self.state
            .all_nodes()
            .into_iter()
            .filter(|(_, value)| *value)
            .map(|(position, _)| position)
    }
}
