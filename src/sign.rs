#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sign {
    Positive,
    Negative,
}

impl From<Sign> for u8 {
    fn from(sign: Sign) -> Self {
        match sign {
            Sign::Positive => 0,
            Sign::Negative => 1,
        }
    }
}
