use crate::{ALL_ONES, u8x8};

/// A vector of eight `bool` values, which can have SIMD-like operations applied
/// to them without any explicit SIMD instructions.
///
/// This type is really just a [`u64`], but its methods interpret it as eight
/// [`bool`] values where the same operation is applied to all eight values at
/// once.
#[allow(non_camel_case_types)]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct mask8x8 {
    pub(crate) n: u64,
}

impl mask8x8 {
    /// A [`mask8x8`] value where all eight elements are set to `false`.
    pub const ALL_FALSE: Self = Self::new(0);

    /// A [`mask8x8`] value where all eight elements are set to `true`.
    pub const ALL_TRUE: Self = Self::new(ALL_ONES);

    /// Converts the given array into a [`mask8x8`].
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
    ///
    /// ```rust
    /// # use eight_bytes::{mask8x8};
    /// let mask = mask8x8::from_bitmask_le(0b11001010);
    /// assert_eq!(mask.to_array(), [false, true, false, true, false, false, true, true]);
    /// ```
    #[inline(always)]
    pub const fn from_bitmask_le(mask: u8) -> Self {
        let raw = Self::from_bitmask_raw(mask).to_le();
        Self::new(raw)
    }

    /// Converts the given bitmask into a [`mask8x8`] by treating a set bit
    /// as `true` and an unset bit as `false`. The least significant bit
    /// appears in the last element.
    ///
    /// ```rust
    /// # use eight_bytes::{mask8x8};
    /// let mask = mask8x8::from_bitmask_be(0b11001010);
    /// assert_eq!(mask.to_array(), [true, true, false, false, true, false, true, false]);
    /// ```
    #[inline(always)]
    pub const fn from_bitmask_be(mask: u8) -> Self {
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

    #[inline(always)]
    const fn to_bitmask_raw(raw: u64) -> u8 {
        const MASK: u64 = 0x0102040810204080;
        (raw.wrapping_mul(MASK) >> 56) as u8
    }

    /// Converts the vector into a bitmask where the first element is in
    /// the least significant bit.
    #[inline(always)]
    pub const fn to_bitmask_le(self) -> u8 {
        Self::to_bitmask_raw(self.n.to_le())
    }

    /// Converts the vector into a bitmask where the first element is in
    /// the most significant bit.
    #[inline(always)]
    pub const fn to_bitmask_be(self) -> u8 {
        Self::to_bitmask_raw(self.n.to_be())
    }

    /// Returns a [`u8x8`] representation of the mask where true elements
    /// are represented as `0x01` and false elements are represented as `0x00`.
    #[inline(always)]
    pub const fn to_u8x8(self) -> u8x8 {
        u8x8::new(self.n)
    }

    /// Returns a [`u8x8`] representation of the mask where true elements
    /// are represented as `v` and false elements are represented as `0x00`.
    #[inline(always)]
    pub const fn to_u8x8_with(self, v: u8) -> u8x8 {
        u8x8::new(u8x8::new(self.n).n * u8x8::splat(v).n)
    }

    /// Computes the complement of each element in the vector.
    #[inline(always)]
    pub const fn not(self) -> Self {
        Self::new(self.n ^ 0x0101010101010101)
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

    /// Builds a [`u8x8`] by selecting one of the two given values for each
    /// element corresponding to the elements in the mask.
    ///
    /// For example, this can be useful when expanding a one-bit-per-pixel
    /// bitmap into eight palette indices represented as `u8`, as part of
    /// rendering an indexed-color pixmap:
    ///
    /// ```rust
    /// # use eight_bytes::{u8x8, mask8x8};
    /// let bitmap = 0b10101100;
    /// let mask = mask8x8::from_bitmask_be(bitmap);
    /// let fg_color = 0xff;
    /// let bg_color = 0x01;
    /// let pixels = mask.select(fg_color, bg_color);
    /// assert_eq!(pixels.to_array(), [0xff, 0x01, 0xff, 0x01, 0xff, 0xff, 0x01, 0x01]);
    /// ```
    #[inline(always)]
    pub const fn select(self, true_value: u8, false_value: u8) -> u8x8 {
        let true_value = u8x8::splat(true_value).n;
        let false_value = u8x8::splat(false_value).n;
        let mask = self.n * 0xff;
        u8x8::new((true_value & mask) | (false_value & !mask))
    }

    /// Returns the number of elements in the mask that are set to `true`.
    #[inline(always)]
    pub const fn count_true(self) -> u32 {
        self.to_u8x8().reduce_sum() as u32
    }

    /// Returns the number of elements in the mask that are set to `false`.
    #[inline(always)]
    pub const fn count_false(self) -> u32 {
        8 - self.n.count_ones()
    }
}

impl core::fmt::Debug for mask8x8 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("mask8x8").field(&self.to_array()).finish()
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
