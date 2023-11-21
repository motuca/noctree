pub trait Half {
    fn half(&self) -> Self;
}

impl Half for f32 {
    fn half(&self) -> Self {
        // This is probably faster than division?
        self * 0.5
    }
}

impl Half for f64 {
    fn half(&self) -> Self {
        // This is probably faster than division?
        self * 0.5
    }
}

impl Half for i8 {
    fn half(&self) -> Self {
        self / 2
    }
}

impl Half for i16 {
    fn half(&self) -> Self {
        self / 2
    }
}

impl Half for i32 {
    fn half(&self) -> Self {
        self / 2
    }
}

impl Half for i64 {
    fn half(&self) -> Self {
        self / 2
    }
}

impl Half for i128 {
    fn half(&self) -> Self {
        self / 2
    }
}

impl Half for isize {
    fn half(&self) -> Self {
        self / 2
    }
}

impl Half for u8 {
    fn half(&self) -> Self {
        self / 2
    }
}

impl Half for u16 {
    fn half(&self) -> Self {
        self / 2
    }
}

impl Half for u32 {
    fn half(&self) -> Self {
        self / 2
    }
}

impl Half for u64 {
    fn half(&self) -> Self {
        self / 2
    }
}

impl Half for u128 {
    fn half(&self) -> Self {
        self / 2
    }
}

impl Half for usize {
    fn half(&self) -> Self {
        self / 2
    }
}

pub trait Abs {
    fn abs(&self) -> Self;
}

impl Abs for f32 {
    fn abs(&self) -> Self {
        f32::abs(*self)
    }
}

impl Abs for f64 {
    fn abs(&self) -> Self {
        f64::abs(*self)
    }
}

impl Abs for i8 {
    fn abs(&self) -> Self {
        i8::abs(*self)
    }
}

impl Abs for i16 {
    fn abs(&self) -> Self {
        i16::abs(*self)
    }
}

impl Abs for i32 {
    fn abs(&self) -> Self {
        i32::abs(*self)
    }
}

impl Abs for i64 {
    fn abs(&self) -> Self {
        i64::abs(*self)
    }
}

impl Abs for i128 {
    fn abs(&self) -> Self {
        i128::abs(*self)
    }
}

impl Abs for isize {
    fn abs(&self) -> Self {
        isize::abs(*self)
    }
}

impl Abs for u8 {
    fn abs(&self) -> Self {
        *self
    }
}

impl Abs for u16 {
    fn abs(&self) -> Self {
        *self
    }
}

impl Abs for u32 {
    fn abs(&self) -> Self {
        *self
    }
}

impl Abs for u64 {
    fn abs(&self) -> Self {
        *self
    }
}

impl Abs for u128 {
    fn abs(&self) -> Self {
        *self
    }
}

pub trait NonNegative {
    fn is_non_negative(&self) -> bool;
}

impl NonNegative for f32 {
    fn is_non_negative(&self) -> bool {
        *self >= 0.0
    }
}

impl NonNegative for f64 {
    fn is_non_negative(&self) -> bool {
        *self >= 0.0
    }
}

impl NonNegative for i8 {
    fn is_non_negative(&self) -> bool {
        *self >= 0
    }
}

impl NonNegative for i16 {
    fn is_non_negative(&self) -> bool {
        *self >= 0
    }
}

impl NonNegative for i32 {
    fn is_non_negative(&self) -> bool {
        *self >= 0
    }
}

impl NonNegative for i64 {
    fn is_non_negative(&self) -> bool {
        *self >= 0
    }
}

impl NonNegative for i128 {
    fn is_non_negative(&self) -> bool {
        *self >= 0
    }
}

impl NonNegative for isize {
    fn is_non_negative(&self) -> bool {
        *self >= 0
    }
}

impl NonNegative for u8 {
    fn is_non_negative(&self) -> bool {
        true
    }
}

impl NonNegative for u16 {
    fn is_non_negative(&self) -> bool {
        true
    }
}

impl NonNegative for u32 {
    fn is_non_negative(&self) -> bool {
        true
    }
}

impl NonNegative for u64 {
    fn is_non_negative(&self) -> bool {
        true
    }
}

impl NonNegative for u128 {
    fn is_non_negative(&self) -> bool {
        true
    }
}