use std::iter;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub struct PointQuadtree<T> {
    position: Point,
    value: T,
    children: Box<[Option<PointQuadtree<T>>; 4]>,
}

impl<T: Copy> PointQuadtree<T> {
    pub fn new(position: Point, value: T) -> Self {
        Self {
            position,
            value,
            children: Box::new([None, None, None, None]),
        }
    }

    fn child_index(&self, position: Point) -> usize {
        match (position.x < self.position.x, position.y < self.position.y) {
            (true, true) => 0,
            (true, false) => 1,
            (false, true) => 2,
            (false, false) => 3,
        }
    }

    pub fn get(&self, position: Point) -> Option<T> {
        if position == self.position {
            return Some(self.value);
        }

        let index = self.child_index(position);
        self.children[index].as_ref()?.get(position)
    }

    pub fn get_mut(&mut self, position: Point) -> Option<&mut T> {
        if position == self.position {
            return Some(&mut self.value);
        }

        let index = self.child_index(position);
        self.children[index].as_mut()?.get_mut(position)
    }

    pub fn insert(&mut self, position: Point, value: T) {
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
                self.children[index] = Some(Self::new(position, value));
            }
        }
    }

    pub fn all_nodes(&self) -> Vec<(Point, T)> {
        iter::once((self.position, self.value))
            .chain(
                self.children
                    .iter()
                    .flatten()
                    .flat_map(PointQuadtree::all_nodes),
            )
            .collect()
    }
}
