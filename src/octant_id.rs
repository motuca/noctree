use crate::sign::Sign;

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
/// | Binary | Bit 0 (reserved, always 0) | Bit x | Bit y | Bit z | Decimal |
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
    bits: [u8; N]
}

impl<const N: usize> Id<N> {
    pub fn from_depth_and_signs(prev_depth: &[u8; N], x: Sign, y: Sign, z: Sign) -> Self {
        let mut bits = *prev_depth;
        let current_depth = find_first_zero(&bits).unwrap_or(N-1);
        bits[current_depth] = match (x, y, z) {
            (Sign::Positive, Sign::Positive, Sign::Positive) => 0b0000_0000,
            (Sign::Positive, Sign::Positive, Sign::Negative) => 0b0000_0001,
            (Sign::Positive, Sign::Negative, Sign::Positive) => 0b0000_0010,
            (Sign::Positive, Sign::Negative, Sign::Negative) => 0b0000_0011,
            (Sign::Negative, Sign::Positive, Sign::Positive) => 0b0000_0100,
            (Sign::Negative, Sign::Positive, Sign::Negative) => 0b0000_0101,
            (Sign::Negative, Sign::Negative, Sign::Positive) => 0b0000_0110,
            (Sign::Negative, Sign::Negative, Sign::Negative) => 0b0000_0111,
        };

        Self { bits }
    }

    pub fn depth(&self) -> usize {
        find_first_zero(&self.bits).unwrap_or(N-1)
    }

    /// Get the bits of the ID
    pub fn bits(&self) -> &[u8; N] {
        &self.bits
    }
}

impl<const N: usize> From<[u8; N]> for Id<N> {
    fn from(bits: [u8; N]) -> Self {
        Self { bits }
    }
}

fn find_first_zero<const N: usize>(bits: &[u8; N]) -> Option<usize> {
    bits.iter().position(|&b| b == 0)
}

mod tests {
    use std::collections::HashMap;

    #[test]
    fn can_hash_octant_id() {
        use crate::octant_id::Id;
        
        let mut map = HashMap::new();
        map.insert(Id::from([0b0000_0001]), 1);
    }
}