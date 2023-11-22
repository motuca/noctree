use std::{
    collections::HashMap,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::{
    error::{NegativeRangeError, OutOfRangeError},
    octant::{Leaf, Node, Octant},
    octant_id::Id,
    sign::Sign,
    util::{Abs, Half, NonNegative},
};

/// # Generic parameters
///
/// - `T`: The numeric type of the coordinates.
/// - `N`: The maximum depth of the octree, 0 indexed and exclusive.
pub struct Octree<T, U, const N: usize>
where
    T: Mul
        + Div
        + Add
        + Sub
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + PartialEq
        + PartialOrd
        + Half
        + Copy,
{
    center: [T; 3],
    ranges: [T; 3],
    threshold: usize,
    tree: HashMap<Id<N>, Octant<T, U>>,
}

fn position_to_octant<T>(center: &[T; 3], point: &[T; 3]) -> [Sign; 3]
where
    T: Mul
        + Div
        + Add
        + Sub
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + PartialEq
        + PartialOrd
        + Half
        + Copy,
{
    let mut signs = [Sign::Positive; 3];
    for i in 0..3 {
        if point[i] < center[i] {
            signs[i] = Sign::Negative;
        }
    }
    signs
}

fn range_and_center_for_octant<T>(
    parent_center: [T; 3],
    parent_ranges: [T; 3],
    signs: [Sign; 3],
) -> ([T; 3], [T; 3])
where
    T: Mul
        + Div
        + Add
        + Sub
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + PartialEq
        + PartialOrd
        + Half
        + Copy,
{
    let mut center = parent_center;
    let mut ranges = parent_ranges;
    for i in 0..3 {
        if signs[i] == Sign::Positive {
            center[i] += ranges[i].half();
        } else {
            center[i] -= ranges[i].half();
        }
        ranges[i] = ranges[i].half();
    }
    (center, ranges)
}

fn split_octant_to_next_depth<T, U, const N: usize>(
    original_id: Id<N>,
    original: Octant<T, U>,
    tree: &mut HashMap<Id<N>, Octant<T, U>>,
) where
    T: Mul
        + Div
        + Add
        + Sub
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + PartialEq
        + PartialOrd
        + Half
        + Copy,
{
    let (center, ranges, points) = match original {
        Octant::Leaf(leaf) => (leaf.center, leaf.ranges, leaf.points),
        Octant::Node(_) => unreachable!("Octant is a node"),
    };

    for point in points {
        let signs = position_to_octant(&center, &point.0);
        let id = Id::child(original_id, signs[0], signs[1], signs[2]);
        let octant = tree.entry(id).or_insert_with(|| {
            let (center, ranges) = range_and_center_for_octant(center, ranges, signs);
            let leaf = Leaf {
                center,
                ranges,
                points: Vec::new(),
            };
            Octant::Leaf(leaf)
        });
        match octant.points_mut() {
            Some(points) => {
                // Octant is a leaf
                points.push(point);
            }
            None => {
                // Octant is a node
                unreachable!("New octant cannot be a node")
            }
        }
    }
}

impl<T, U, const N: usize> Octree<T, U, N>
where
    T: Mul
        + Div
        + Add
        + Sub<Output = T>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + PartialEq
        + PartialOrd
        + Half
        + Abs
        + NonNegative
        + Copy,
{
    pub fn new(
        center: [T; 3],
        ranges: [T; 3],
        threshold: usize,
    ) -> Result<Self, NegativeRangeError> {
        // Range must be positive
        if ranges.iter().any(|r| !r.is_non_negative()) {
            return Err(NegativeRangeError);
        }

        Ok(Self {
            center,
            ranges,
            threshold,
            tree: HashMap::new(),
        })
    }

    pub fn push(&mut self, point: [T; 3], data: U) -> Result<Id<N>, OutOfRangeError> {
        if self
            .ranges
            .iter()
            .zip(self.center.iter())
            .zip(point.iter())
            .any(|((r, c), p)| Abs::abs(p) - Abs::abs(c) > r.half())
        {
            return Err(OutOfRangeError);
        }

        let mut center = self.center;
        let mut ranges = self.ranges;
        let mut signs = position_to_octant(&center, &point);
        let mut id = Id::root(signs[0], signs[1], signs[2]);
        let mut level = 0;
        loop {
            let octant = self.tree.entry(id).or_insert_with(|| {
                let (center, ranges) = range_and_center_for_octant(center, ranges, signs);
                let leaf = Leaf {
                    center,
                    ranges,
                    points: Vec::new(),
                };
                Octant::Leaf(leaf)
            });

            match octant.points_mut() {
                Some(points) => {
                    // Octant is a leaf
                    if (level == N - 1) || (points.len() < self.threshold) {
                        points.push((point, data));
                        break;
                    } else {
                        let node = Node {
                            center: *octant.center(),
                            ranges: *octant.ranges(),
                        };
                        let node = Octant::Node(node);
                        center = *node.center();
                        ranges = *node.ranges();

                        let original = std::mem::replace(octant, node);
                        split_octant_to_next_depth(id, original, &mut self.tree);
                    }
                }
                None => {
                    // Octant is a node
                    center = *octant.center();
                    ranges = *octant.ranges();
                }
            }
            signs = position_to_octant(&center, &point);
            id = Id::child(id, signs[0], signs[1], signs[2]);
            level += 1;
        }

        Ok(id)
    }

    /// Iterate over all the leaves in the tree
    pub fn leaves(&self) -> impl Iterator<Item = &Leaf<T, U>> {
        self.tree.values().filter_map(|octant| match octant {
            Octant::Leaf(leaf) => Some(leaf),
            Octant::Node(_) => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::Octree;

    const CENTER: [f32; 3] = [0.0, 0.0, 0.0];

    /// Points have to be within +/- 5.0 of the center
    const HALF_RANGE: f32 = 5.0;
    const RANGES: [f32; 3] = [2.0 * HALF_RANGE, 2.0 * HALF_RANGE, 2.0 * HALF_RANGE];

    const NEGATIVE_RANGES: [f32; 3] = [-10.0, -10.0, -10.0];

    const THRESHOLD: usize = 10;

    const MAX_DEPTH: usize = 3;

    #[test]
    fn create_octree_with_positive_range_returns_ok() {
        let result = Octree::<_, (), MAX_DEPTH>::new(CENTER, RANGES, THRESHOLD);
        assert!(result.is_ok());
    }

    #[test]
    fn create_octree_with_negative_range_returns_error() {
        let result = Octree::<_, (), MAX_DEPTH>::new(CENTER, NEGATIVE_RANGES, THRESHOLD);
        assert!(result.is_err());
    }

    #[test]
    fn octree_can_push_points_within_range() {
        let mut tree = Octree::<_, (), MAX_DEPTH>::new(CENTER, RANGES, THRESHOLD).unwrap();
        let result = tree.push([0.5, 0.5, 0.5], ());
        assert!(result.is_ok());

        let result = tree.push([5.0, 5.0, 5.0], ());
        assert!(result.is_ok());

        let result = tree.push([-5.0, -5.0, -5.0], ());
        assert!(result.is_ok());
    }

    #[test]
    fn octree_cannot_push_points_outside_range() {
        let mut tree = Octree::<_, (), MAX_DEPTH>::new(CENTER, RANGES, THRESHOLD).unwrap();
        let result = tree.push([10.5, 0.5, 0.5], ());
        assert!(result.is_err());

        let result = tree.push([-10.5, 0.5, 0.5], ());
        assert!(result.is_err());

        let result = tree.push([0.5, 10.5, 0.5], ());
        assert!(result.is_err());

        let result = tree.push([0.5, -10.5, 0.5], ());
        assert!(result.is_err());

        let result = tree.push([0.5, 0.5, 10.5], ());
        assert!(result.is_err());

        let result = tree.push([0.5, 0.5, -10.5], ());
        assert!(result.is_err());
    }

    #[test]
    fn octree_split_to_finer_levels() {
        let mut tree = Octree::<_, (), MAX_DEPTH>::new(CENTER, RANGES, THRESHOLD).unwrap();

        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let x = rng.gen_range(-HALF_RANGE..HALF_RANGE);
            let y = rng.gen_range(-HALF_RANGE..HALF_RANGE);
            let z = rng.gen_range(-HALF_RANGE..HALF_RANGE);
            let point = [x, y, z];
            tree.push(point, ()).unwrap();
        }

        let leaves = tree.leaves().collect::<Vec<_>>();
        assert!(leaves.len() > 0);
        // println!("# Leaves: {}", leaves.len());
    }
}
