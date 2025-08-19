use std::ops::{BitAnd, BitOr, Not, Shl, Sub};

use crate::helper::misc::One;

pub trait Bitmask
where
    Self: Sized
        + Copy
        + One
        + BitOr<Output = Self>
        + BitAnd<Output = Self>
        + Not<Output = Self>
        + Shl<u32, Output = Self>
        + Sub<Output = Self>,
{
    fn set_bit(&mut self, index: usize, val: bool) {
        let mask = Self::one() << (index.try_into().unwrap());
        if val {
            *self = *self | mask;
        } else {
            *self = *self & !mask;
        }
    }
}

// Implement the trait for all bitmask types using a macro
macro_rules! impl_bitmask_trait_for_unsigned {
    ($($t:ty),*) => {
        $(
            impl Bitmask for $t {
            }
        )*
    };
}
impl_bitmask_trait_for_unsigned!(u8, u16, u32, u64, u128, usize);
