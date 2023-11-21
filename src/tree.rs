use std::{
    collections::HashMap,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::{
    octant::{Octant, OctantType},
    octant_id::Id,
    sign::Sign,
    util::Half,
};

/// # Generic parameters
///
/// - `T`: The numeric type of the coordinates.
/// - `N`: The maximum depth of the octree, 0 indexed and exclusive.
pub struct Octree<T, const N: usize>
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
    tree: HashMap<Id<N>, Octant<T>>,
}

fn position_to_octant<T>(center: &[T; 3], ranges: &[T; 3], point: &[T; 3]) -> [Sign; 3]
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

fn split_octant_to_next_depth<T, const N: usize>(original: Octant<T>, tree: &mut HashMap<Id<N>, Octant<T>>) 
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

}

impl<T, const N: usize> Octree<T, N>
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
    pub fn push(&mut self, point: [T; 3]) {
        let mut center = self.center;
        let mut ranges = self.ranges;
        loop {
            let signs = position_to_octant(&center, &ranges, &point);
            let id = Id::from_depth_and_signs(&[0; N], signs[0], signs[1], signs[2]);
            let octant = self.tree.entry(id).or_insert_with(|| {
                let (center, ranges) = range_and_center_for_octant(center,ranges, signs);
                Octant {
                    center,
                    ranges,
                    octant_type: OctantType::Leaf { points: Vec::new() },
                }
            });

            match octant.points_mut() {
                Some(points) => {
                    // Octant is a leaf
                    if points.len() < self.threshold {
                        points.push(point);
                        break;
                    } else {
                        let node = Octant {
                            center: octant.center,
                            ranges: octant.ranges,
                            octant_type: OctantType::Node,
                        };
                        let original = std::mem::replace(octant, node);
                        split_octant_to_next_depth(original, &mut self.tree);
                        todo!()
                    }
                },
                None => {
                    // Octant is a node
                    center = octant.center;
                    ranges = octant.ranges;
                },
            }
        }
    }
}
