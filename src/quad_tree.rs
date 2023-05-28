use std::iter;

#[derive(Clone)]
pub struct QuadTree<T> {
    size: (usize, usize),
    anchor: (usize, usize),
    position: (usize, usize),
    value: T,
    children: Box<[Option<QuadTree<T>>; 4]>,
}

impl<T: Copy> QuadTree<T> {
    pub fn new(size: (usize, usize), position: (usize, usize), value: T) -> Self {
        assert_eq!(size.0.count_ones(), 1, "size.0 must be a power of 2");
        assert_eq!(size.1.count_ones(), 1, "size.1 must be a power of 2");

        Self::new_node(size, position, (0, 0), value)
    }

    fn new_node(
        size: (usize, usize),
        position: (usize, usize),
        anchor: (usize, usize),
        value: T,
    ) -> Self {
        Self {
            size,
            anchor,
            position,
            value,
            children: Box::new([None, None, None, None]),
        }
    }

    fn child_index(&self, position: (usize, usize)) -> usize {
        match (
            position.0 < self.anchor.0 + self.size.0 / 2,
            position.1 < self.anchor.1 + self.size.1 / 2,
        ) {
            (true, true) => 0,
            (true, false) => 1,
            (false, true) => 2,
            (false, false) => 3,
        }
    }

    pub fn get(&self, position: (usize, usize)) -> Option<T> {
        if position == self.position {
            return Some(self.value);
        }

        let index = self.child_index(position);
        self.children[index].as_ref()?.get(position)
    }

    pub fn get_mut(&mut self, position: (usize, usize)) -> Option<&mut T> {
        if position == self.position {
            return Some(&mut self.value);
        }

        let index = self.child_index(position);
        self.children[index].as_mut()?.get_mut(position)
    }

    pub fn insert(&mut self, position: (usize, usize), value: T) {
        if position == self.position {
            self.value = value;
            return;
        }

        let index = self.child_index(position);

        match self.children.get_mut(index).unwrap() {
            Some(child) => {
                child.insert(position, value);
            }
            None => {
                let anchor = (
                    if position.0 < self.anchor.0 + self.size.0 / 2 {
                        self.anchor.0
                    } else {
                        self.anchor.0 + self.size.0 / 2
                    },
                    if position.1 < self.anchor.1 + self.size.1 / 2 {
                        self.anchor.1
                    } else {
                        self.anchor.1 + self.size.1 / 2
                    },
                );

                self.children[index] = Some(Self::new_node(
                    (self.size.0 / 2, self.size.1 / 2),
                    position,
                    anchor,
                    value,
                ));
            }
        }
    }

    pub fn all_nodes(&self) -> Vec<((usize, usize), T)> {
        iter::once((self.position, self.value))
            .chain(self.children.iter().flatten().flat_map(QuadTree::all_nodes))
            .collect()
    }

    pub fn size(&self) -> (usize, usize) {
        self.size
    }
}
