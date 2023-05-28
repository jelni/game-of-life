use std::iter;

pub struct QuadTree<T> {
    size: usize,
    anchor: (usize, usize),
    position: (usize, usize),
    value: T,
    children: Box<[Option<QuadTree<T>>; 4]>,
}

impl<T: Copy> QuadTree<T> {
    pub fn new(size: usize, position: (usize, usize), value: T) -> Self {
        assert_eq!(size.count_ones(), 1, "size must be a power of 2");

        Self::new_node(size, position, (0, 0), value)
    }

    fn new_node(size: usize, position: (usize, usize), anchor: (usize, usize), value: T) -> Self {
        Self {
            size,
            anchor,
            position,
            value,
            children: Box::new([None, None, None, None]),
        }
    }

    pub fn get(&self, position: (usize, usize)) -> Option<T> {
        if position == self.position {
            return Some(self.value);
        }

        let index = match (
            position.0 < self.anchor.0 + self.size / 2,
            position.1 < self.anchor.1 + self.size / 2,
        ) {
            (true, true) => 0,
            (true, false) => 1,
            (false, true) => 2,
            (false, false) => 3,
        };

        self.children[index].as_ref()?.get(position)
    }

    pub fn insert(&mut self, position: (usize, usize), value: T) {
        if position == self.position {
            self.value = value;
            return;
        }

        let (index, anchor) = match (
            position.0 < self.anchor.0 + self.size / 2,
            position.1 < self.anchor.1 + self.size / 2,
        ) {
            (true, true) => (0, self.anchor),
            (true, false) => (1, (self.anchor.0, self.anchor.1 + self.size / 2)),
            (false, true) => (2, (self.anchor.0 + self.size / 2, self.anchor.1)),
            (false, false) => (
                3,
                (self.anchor.0 + self.size / 2, self.anchor.1 + self.size / 2),
            ),
        };

        match self.children.get_mut(index).unwrap() {
            Some(child) => {
                child.insert(position, value);
            }
            None => {
                self.children[index] = Some(Self::new_node(self.size / 2, position, anchor, value));
            }
        }
    }

    pub fn all_nodes(&self) -> Vec<((usize, usize), T)> {
        iter::once((self.position, self.value))
            .chain(self.children.iter().flatten().flat_map(QuadTree::all_nodes))
            .collect()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
