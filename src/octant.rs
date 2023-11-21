pub enum OctantType<T> {
    /// A node is an octant that has children
    Node,

    /// A leaf is an octant that has no children
    Leaf {
        points: Vec<[T; 3]>,
    },
}

pub struct Octant<T> {
    pub center: [T; 3],
    pub ranges: [T; 3],
    pub octant_type: OctantType<T>,
}

impl<T> Octant<T> {
    pub(crate) fn is_leaf(&self) -> bool {
        match self.octant_type {
            OctantType::Node => false,
            OctantType::Leaf { .. } => true,
        }
    }

    pub(crate) fn is_node(&self) -> bool {
        match self.octant_type {
            OctantType::Node => true,
            OctantType::Leaf { .. } => false,
        }
    }

    pub(crate) fn points(&self) -> Option<&Vec<[T; 3]>> {
        match &self.octant_type {
            OctantType::Node => None,
            OctantType::Leaf { points } => Some(points),
        }
    }

    pub(crate) fn points_mut(&mut self) -> Option<&mut Vec<[T; 3]>> {
        match &mut self.octant_type {
            OctantType::Node => None,
            OctantType::Leaf { points } => Some(points),
        }
    }

}