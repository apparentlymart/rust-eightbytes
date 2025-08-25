#![cfg_attr(not(test), no_std)]

/// A vector of eight `u8` values, which can have SIMD-like operations applied
/// to them without any explicit SIMD instructions.
///
/// This type is really just a u64, but its methods interpret it as eight `u8`
/// values where the same operation is applied to all eight values at once.
#[allow(non_camel_case_types)]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct u8x8 {
    n: u64,
}

impl u8x8 {
    /// Converts an array of eight `u8` values into a [`u8x8`] value.
    #[inline(always)]
    pub const fn from_array(a: [u8; 8]) -> Self {
        Self {
            n: u64::from_ne_bytes(a),
        }
    }

    #[inline(always)]
    const fn new(n: u64) -> Self {
        Self { n }
    }

    /// Returns a [`u8x8`] with `v` in all eight of its elements.
    #[inline(always)]
    pub const fn splat(v: u8) -> Self {
        Self::new(v as u64 * ALL_ONES)
    }

    /// Converts the vector into an array of eight `u8` values.
    pub const fn to_array(self) -> [u8; 8] {
        self.n.to_ne_bytes()
    }

    /// Implements addition across corresponding elements, modulo 256.
    #[inline(always)]
    pub const fn wrapping_add(self, other: Self) -> Self {
        let low = (self.n & WITHOUT_HIGH_BITS) + (other.n & WITHOUT_HIGH_BITS);
        Self::new(low ^ ((self.n ^ other.n) & ONLY_HIGH_BITS))
    }

    /// Implements addition across corresponding elements, saturating at the
    /// maximum value 255.
    #[inline(always)]
    pub const fn saturating_add(self, other: Self) -> Self {
        let sum = self.wrapping_add(other).n;
        let carry = ((self.n & other.n) | ((self.n | other.n) & !sum)) & ONLY_HIGH_BITS;
        Self::new(sum | (carry >> 7) * 0xff)
    }

    /// Implements subtraction across corresponding elements, modulo 256.
    #[inline(always)]
    pub const fn wrapping_sub(self, other: Self) -> Self {
        Self::new(
            (self.n | ONLY_HIGH_BITS) - (other.n & WITHOUT_HIGH_BITS)
                ^ ((self.n ^ !other.n) & ONLY_HIGH_BITS),
        )
    }

    /// Implements subtraction across corresponding elements, saturating at the
    /// minimum value 0.
    #[inline(always)]
    pub const fn saturating_sub(self, other: Self) -> Self {
        let diff = self.wrapping_sub(other).n;
        let carry = ((!self.n & other.n) | ((!self.n | other.n) & diff)) & ONLY_HIGH_BITS;
        Self::new(diff & !((carry >> 7) * 0xff))
    }
}

impl core::ops::Add for u8x8 {
    type Output = Self;

    /// Implements the `+` operator using [`Self::wrapping_add`].
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        self.wrapping_add(rhs)
    }
}

impl core::ops::AddAssign for u8x8 {
    /// Implements the `+=` operator using [`Self::wrapping_add`].
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.wrapping_add(rhs)
    }
}

impl core::ops::Sub for u8x8 {
    type Output = Self;

    /// Implements the `+` operator using [`Self::wrapping_sub`].
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        self.wrapping_sub(rhs)
    }
}

impl core::ops::SubAssign for u8x8 {
    /// Implements the `-=` operator using [`Self::wrapping_sub`].
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.wrapping_sub(rhs)
    }
}

impl core::fmt::Debug for u8x8 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "u8x8({:?})", self.to_array())
    }
}

/// Raw representation of a vector where all bytes are 1.
const ALL_ONES: u64 = 0x0101010101010101;

/// Raw representation of a vector where all bytes are 0x7f, and therefore
/// all but the most significant bit is set in any element.
///
/// We use this to implement wrapping operations by masking out the high bit
/// so that the operation cannot carry-out into the neighboring element.
const WITHOUT_HIGH_BITS: u64 = 0x7f7f7f7f7f7f7f7f;

/// Raw representation of a vector where all bytes are 0x80, and therefore
/// only the most significant bit is set across all elements.
///
/// This is the complement of [`WITHOUT_HIGH_BITS`], used to deal with the
/// masked-out remnant of a wrapping operation.
const ONLY_HIGH_BITS: u64 = 0x8080808080808080;

#[cfg(test)]
mod tests;
