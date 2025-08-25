use crate::{ALL_ONES, u8x8};

/// A vector of eight `bool` values, which can have SIMD-like operations applied
/// to them without any explicit SIMD instructions.
///
/// This type is really just a u64, but its methods interpret it as eight `bool`
/// values where the same operation is applied to all eight values at once.
#[allow(non_camel_case_types)]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct mask8x8 {
    n: u64,
}

impl mask8x8 {
    #[inline(always)]
    pub const fn from_array(a: [bool; 8]) -> Self {
        // Safety: The Rust spec guarantees that `bool` is has size 1 and
        // alignment 1, and that true is 0x01 while false is 0x00. The
        // layout matches u8 and both bit patterns are valid for u8.
        let a: [u8; 8] = unsafe { core::mem::transmute(a) };
        Self {
            n: u64::from_ne_bytes(a),
        }
    }

    #[inline(always)]
    const fn from_bitmask_raw(mask: u8) -> u64 {
        let raw = mask as u64;
        (((raw & 0x55) * 0x02040810204081) | ((raw & 0xaa) * 0x02040810204081)) & ALL_ONES
    }

    /// Converts the given bitmask into a [`mask8x8`] by treating a set bit
    /// as `true` and an unset bit as `false`. The least significant bit
    /// appears in the first element.
    #[inline(always)]
    pub const fn from_bitmask_littleend(mask: u8) -> Self {
        let raw = Self::from_bitmask_raw(mask).to_le();
        Self::new(raw)
    }

    /// Converts the given bitmask into a [`mask8x8`] by treating a set bit
    /// as `true` and an unset bit as `false`. The least significant bit
    /// appears in the last element.
    #[inline(always)]
    pub const fn from_bitmask_bigend(mask: u8) -> Self {
        let raw = Self::from_bitmask_raw(mask).to_be();
        Self::new(raw)
    }

    #[inline(always)]
    pub(crate) const fn new(n: u64) -> Self {
        Self { n }
    }

    /// Converts the vector into an array of eight `bool` values.
    #[inline(always)]
    pub const fn to_array(self) -> [bool; 8] {
        // Safety: The Rust spec guarantees that `bool` is has size 1 and
        // alignment 1, and that true is 0x01 while false is 0x00. The
        // layout matches u8 and both bit patterns are valid for u8.
        let u8s = self.n.to_ne_bytes();
        unsafe { core::mem::transmute(u8s) }
    }

    /// Returns a [`u8x8`] representation of the mask where true elements
    /// are represented as `0x01` and false elements are represented as `0x00`.
    #[inline(always)]
    pub fn to_u8x8(self) -> u8x8 {
        u8x8::new(self.n)
    }

    /// Computes the complement of each element in the vector.
    #[inline(always)]
    pub const fn not(self) -> Self {
        Self::new(!self.n)
    }

    /// Computes a logical OR result for each element across both vectors.
    #[inline(always)]
    pub const fn or(self, other: Self) -> Self {
        Self::new(self.n | other.n)
    }

    /// Computes a logical AND result for each element across both vectors.
    #[inline(always)]
    pub const fn and(self, other: Self) -> Self {
        Self::new(self.n & other.n)
    }
}

impl core::fmt::Debug for mask8x8 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "mask8x8({:?})", self.to_array())
    }
}

impl core::ops::Not for mask8x8 {
    type Output = Self;

    /// Implements the unary `!` operator using [`Self::not`].
    #[inline(always)]
    fn not(self) -> Self {
        self.not()
    }
}

impl core::ops::BitOr for mask8x8 {
    type Output = Self;

    /// Implements the `|` operator using [`Self::or`].
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self {
        self.or(rhs)
    }
}

impl core::ops::BitOrAssign for mask8x8 {
    /// Implements the `|=` operator using [`Self::or`].
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.or(rhs);
    }
}

impl core::ops::BitAnd for mask8x8 {
    type Output = Self;

    /// Implements the `&` operator using [`Self::and`].
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self {
        self.and(rhs)
    }
}

impl core::ops::BitAndAssign for mask8x8 {
    /// Implements the `&=` operator using [`Self::and`].
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.and(rhs);
    }
}
