use std::ops::{BitAnd, BitOr, Not};

struct BitField{
}

impl BitField {
    fn new() -> Self {
        unimplemented!()
    }

    fn lowest_significant_value() -> Self {
        unimplemented!()
    }

    fn index_into<T>(slice: &[T]) -> &[T] {
        unimplemented!()
    }
}

impl BitOr for BitField{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl BitAnd for BitField{
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Not for BitField{
    type Output = Self;

    fn not(self) -> Self::Output {
        todo!()
    }
}