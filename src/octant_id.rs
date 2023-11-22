use crate::sign::Sign;

const LEVEL_ACTIVE_MASK: u8 = 0b0000_1000;
const X_MASK: u8 = 0b0000_0100;
const Y_MASK: u8 = 0b0000_0010;
const Z_MASK: u8 = 0b0000_0001;

/// Octant ID
///
/// # Generic parameters
///
/// - `N`: The maximum depth of the octree, 0 indexed and exclusive.
///
/// # Octant numbering
///
/// Positive is numbered 0b0 while negative is numbered 0b1.
/// A `u8` is used to represent the octant number within the depth.
/// This is a bit waste of space, but it makes the code simpler.
///
/// | Binary | Level active (1 if the level is active) | Bit x | Bit y | Bit z | Decimal |
/// |--------|----------------------------|-------|-------|-------| ------- |
/// | 0b0000  | 0                         | 0,+   | 0,+   | 0,+   | 0      |
/// | 0b0001  | 0                         | 0,+   | 0,+   | 1,-   | 1      |
/// | 0b0010  | 0                         | 0,+   | 1,-   | 0,+   | 2      |
/// | 0b0011  | 0                         | 0,+   | 1,-   | 1,-   | 3      |
/// | 0b0100  | 0                         | 1,-   | 0,+   | 0,+   | 4      |
/// | 0b0101  | 0                         | 1,-   | 0,+   | 1,-   | 5      |
/// | 0b0110  | 0                         | 1,-   | 1,-   | 0,+   | 6      |
/// | 0b0111  | 0                         | 1,-   | 1,-   | 1,-   | 7      |
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id<const N: usize> {
    bits: [u8; N],
}

fn signs_to_bits(x: Sign, y: Sign, z: Sign) -> u8 {
    let mut bits = 0;
    if x == Sign::Negative {
        bits |= X_MASK;
    }
    if y == Sign::Negative {
        bits |= Y_MASK;
    }
    if z == Sign::Negative {
        bits |= Z_MASK;
    }
    bits
}

impl<const N: usize> Id<N> {
    /// Create a new uninitialized ID
    pub fn uninitialized() -> Self {
        Self { bits: [0; N] }
    }

    pub fn is_uninitialized(&self) -> bool {
        self.bits.iter().all(|&b| b == 0)
    }

    /// Create a new ID at the root level
    pub fn root(x: Sign, y: Sign, z: Sign) -> Self {
        let mut bits = [0; N];
        bits[0] = signs_to_bits(x, y, z);
        bits[0] |= LEVEL_ACTIVE_MASK;

        Self { bits }
    }

    pub fn child(parent_id: Self, x: Sign, y: Sign, z: Sign) -> Self {
        let mut bits = parent_id.bits;
        let current_depth = find_first_zero(&bits).unwrap_or(N - 1);
        bits[current_depth] = signs_to_bits(x, y, z);
        bits[current_depth] |= LEVEL_ACTIVE_MASK;

        Self { bits }
    }

    // pub fn depth(&self) -> usize {
    //     find_first_zero(&self.bits)
    //         .map(|i| i - 1)
    //         .unwrap_or(N-1)
    // }

    // /// Get the bits of the ID
    // pub fn bits(&self) -> &[u8; N] {
    //     &self.bits
    // }
}

impl<const N: usize> From<[u8; N]> for Id<N> {
    fn from(bits: [u8; N]) -> Self {
        Self { bits }
    }
}

fn find_first_zero<const N: usize>(bits: &[u8; N]) -> Option<usize> {
    bits.iter().position(|&b| b == 0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_hash_octant_id() {
        use crate::octant_id::Id;
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert(Id::from([0b0000_0001]), 1);
    }
}
