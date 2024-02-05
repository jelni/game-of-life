use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i16,
    pub y: i16,
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

    const fn child_index(&self, position: Point) -> usize {
        match (position.x < self.position.x, position.y < self.position.y) {
            (true, true) => 0,
            (true, false) => 1,
            (false, true) => 2,
            (false, false) => 3,
        }
    }

    pub fn get(&self, position: Point) -> Option<T> {
        let mut point = self;

        while point.position != position {
            let index = point.child_index(position);
            point = point.children[index].as_ref()?;
        }

        Some(point.value)
    }

    pub fn get_mut(&mut self, position: Point) -> Option<&mut T> {
        let mut point = self;

        while point.position != position {
            let index = point.child_index(position);
            point = point.children[index].as_mut()?;
        }

        Some(&mut point.value)
    }

    pub fn insert(&mut self, position: Point, value: T) {
        let mut point = self;

        loop {
            if point.position == position {
                point.value = value;
                break;
            }

            let index = point.child_index(position);

            if point.children[index].is_some() {
                point = point.children[index].as_mut().unwrap();
            } else {
                point.children[index] = Some(Self::new(position, value));
            }
        }
    }

    pub fn all_points(&self) -> Vec<(Point, T)> {
        let mut points = Vec::new();
        let mut nodes = self.children.iter().flatten().collect::<VecDeque<_>>();

        while let Some(node) = nodes.pop_front() {
            points.push((node.position, node.value));
            nodes.extend(node.children.iter().flatten());
        }

        points
    }
}

impl From<&[Point]> for PointQuadtree<bool> {
    fn from(points: &[Point]) -> Self {
        let mut tree = Self::new(Point { x: 0, y: 0 }, false);

        for &point in points {
            tree.insert(point, true);
        }

        tree
    }
}
