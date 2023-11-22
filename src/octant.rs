pub enum Octant<T, U> {
    Node(Node<T>),
    Leaf(Leaf<T, U>),
}

impl<T, U> Octant<T, U> {
    pub fn center(&self) -> &[T; 3] {
        match self {
            Octant::Node(node) => &node.center,
            Octant::Leaf(leaf) => &leaf.center,
        }
    }

    pub fn ranges(&self) -> &[T; 3] {
        match self {
            Octant::Node(node) => &node.ranges,
            Octant::Leaf(leaf) => &leaf.ranges,
        }
    }

    // pub fn is_leaf(&self) -> bool {
    //     match self {
    //         Octant::Node(_) => false,
    //         Octant::Leaf(_) => true,
    //     }
    // }

    // pub fn is_node(&self) -> bool {
    //     match self {
    //         Octant::Node(_) => true,
    //         Octant::Leaf(_) => false,
    //     }
    // }

    // pub fn points(&self) -> Option<&Vec<[T; 3]>> {
    //     match self {
    //         Octant::Node(_) => None,
    //         Octant::Leaf(leaf) => Some(&leaf.points),
    //     }
    // }

    pub fn points_mut(&mut self) -> Option<&mut Vec<([T; 3], U)>> {
        match self {
            Octant::Node(_) => None,
            Octant::Leaf(leaf) => Some(&mut leaf.points),
        }
    }
}

#[derive(Debug)]
pub struct Node<T> {
    pub center: [T; 3],
    pub ranges: [T; 3],
}

#[derive(Debug)]
pub struct Leaf<T, U> {
    pub center: [T; 3],
    pub ranges: [T; 3],
    pub points: Vec<([T; 3], U)>,
}
