//! Root module for 256-bit unsigned integer type.

//mod api;
mod cmp;
mod convert;
mod ops;

// todo!()
mod temp;

/// A 256-bit unsigned integer type.
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct I256(pub [i128; 2]);

/// A 256-bit unsigned integer type.
#[allow(non_camel_case_types)]
pub type i256 = I256;

impl I256 {
    /// The additive identity for this integer type, i.e. `0`.
    pub const ZERO: Self = I256([0; 2]);

    /// The multiplicative identity for this integer type, i.e. `1`.
    pub const ONE: Self = I256::new(1);

    /// Creates a new 256-bit integer value from a primitive `i128` integer.
    #[inline]
    pub const fn new(value: i128) -> Self {
        I256::from_words(value >> 127, value)
    }

    /// Creates a new 256-bit integer value from high and low words.
    #[inline]
    pub const fn from_words(hi: i128, lo: i128) -> Self {
        #[cfg(target_endian = "little")]
        {
            I256([lo, hi])
        }
        #[cfg(target_endian = "big")]
        {
            I256([hi, lo])
        }
    }

    /// Splits a 256-bit integer into high and low words.
    #[inline]
    pub const fn into_words(self) -> (i128, i128) {
        #[cfg(target_endian = "little")]
        {
            let I256([lo, hi]) = self;
            (hi, lo)
        }
        #[cfg(target_endian = "big")]
        {
            let I256([hi, lo]) = self;
            (hi, lo)
        }
    }

    /// Get the low 128-bit word for this unsigned integer.
    #[inline]
    pub fn low(&self) -> &i128 {
        #[cfg(target_endian = "little")]
        {
            &self.0[0]
        }
        #[cfg(target_endian = "big")]
        {
            &self.0[1]
        }
    }

    /// Get the low 128-bit word for this unsigned integer as a mutable
    /// reference.
    #[inline]
    pub fn low_mut(&mut self) -> &mut i128 {
        #[cfg(target_endian = "little")]
        {
            &mut self.0[0]
        }
        #[cfg(target_endian = "big")]
        {
            &mut self.0[1]
        }
    }

    /// Get the high 128-bit word for this unsigned integer.
    #[inline]
    pub fn high(&self) -> &i128 {
        #[cfg(target_endian = "little")]
        {
            &self.0[1]
        }
        #[cfg(target_endian = "big")]
        {
            &self.0[0]
        }
    }

    /// Get the high 128-bit word for this unsigned integer as a mutable
    /// reference.
    #[inline]
    pub fn high_mut(&mut self) -> &mut i128 {
        #[cfg(target_endian = "little")]
        {
            &mut self.0[1]
        }
        #[cfg(target_endian = "big")]
        {
            &mut self.0[0]
        }
    }

    /// Cast to a primitive `i8`.
    pub const fn as_i8(self) -> i8 {
        let (_, lo) = self.into_words();
        lo as _
    }

    /// Cast to a primitive `i16`.
    pub const fn as_i16(self) -> i16 {
        let (_, lo) = self.into_words();
        lo as _
    }

    /// Cast to a primitive `i32`.
    pub const fn as_i32(self) -> i32 {
        let (_, lo) = self.into_words();
        lo as _
    }

    /// Cast to a primitive `i64`.
    pub const fn as_i64(self) -> i64 {
        let (_, lo) = self.into_words();
        lo as _
    }

    /// Cast to a primitive `i128`.
    pub const fn as_i128(self) -> i128 {
        let (_, lo) = self.into_words();
        lo as _
    }

    /// Cast to a primitive `u8`.
    pub const fn as_u8(self) -> u8 {
        let (_, lo) = self.into_words();
        lo as _
    }

    /// Cast to a primitive `u16`.
    pub const fn as_u16(self) -> u16 {
        let (_, lo) = self.into_words();
        lo as _
    }

    /// Cast to a primitive `u32`.
    pub const fn as_u32(self) -> u32 {
        let (_, lo) = self.into_words();
        lo as _
    }

    /// Cast to a primitive `u64`.
    pub const fn as_u64(self) -> u64 {
        let (_, lo) = self.into_words();
        lo as _
    }

    /// Cast to a primitive `u128`.
    pub const fn as_u128(self) -> u128 {
        let (_, lo) = self.into_words();
        lo as _
    }

    /* todo!()
    /// Cast to a `U256`.
    pub const fn as_u256(self) -> U256 {
        AsU256::as_u256(self)
    }
    */

    /// Cast to a primitive `isize`.
    pub const fn as_isize(self) -> isize {
        let (_, lo) = self.into_words();
        lo as _
    }

    /// Cast to a primitive `usize`.
    pub const fn as_usize(self) -> usize {
        let (_, lo) = self.into_words();
        lo as _
    }

    /// Cast to a primitive `f32`.
    pub fn as_f32(self) -> f32 {
        self.as_f64() as _
    }

    /// Cast to a primitive `f64`.
    pub fn as_f64(self) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::I256;

    #[test]
    #[ignore]
    #[allow(clippy::float_cmp)]
    fn converts_to_f64() {
        assert_eq!(I256::from_words(1, 0).as_f64(), 2.0f64.powi(128))
    }
}