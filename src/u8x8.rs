use crate::mask8x8;

/// A vector of eight `u8` values, which can have SIMD-like operations applied
/// to them without any explicit SIMD instructions.
///
/// This type is really just a [`u64`], but its methods interpret it as eight
/// [`u8`] values where the same operation is applied to all eight values at
/// once.
#[allow(non_camel_case_types)]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct u8x8 {
    pub(crate) n: u64,
}

impl u8x8 {
    /// A [`u8x8`] value where all eight elements are set to zero.
    pub const ZEROES: Self = Self::new(0);

    /// Converts an array of eight `u8` values into a [`u8x8`] value.
    #[inline(always)]
    pub const fn from_array(a: [u8; 8]) -> Self {
        Self {
            n: u64::from_ne_bytes(a),
        }
    }

    /// Reinterprets the given byte slice as a slice of [`u8x8`], along with
    /// individual leading and trailing bytes that are not aligned for
    /// interpretation as `u64`.
    ///
    /// This is a useful primitive for analyzing all bytes in a slice more
    /// efficiently than visiting each byte separately. The following example
    /// counts the number of non-space characters in a byte string:
    ///
    /// ```rust
    /// # use eight_bytes::{u8x8, mask8x8};
    /// // (in practice this could only be productive with a much larger
    /// // input than this, but this is just for example.)
    /// let input = b"The quick brown fox jumps over the lazy dog.";
    /// let (start, middle, end) = u8x8::from_byte_slice(input);
    /// const SPACE: u8 = b' ' ;
    /// let space = u8x8::splat(SPACE); // vector of eight spaces
    /// let mut non_space_count = 0;
    ///
    /// // Deal with up to seven leading scalar bytes...
    /// for b in start {
    ///     if *b != SPACE {
    ///         non_space_count += 1;
    ///     }
    /// }
    /// // First visit vectors of eight bytes at a time...
    /// for v in middle {
    ///     let space_mask = v.equals(space); // returns vector of boolean values
    ///     non_space_count += space_mask.not().count_true();
    /// }
    /// // ...then deal with up to seven scalar stragglers.
    /// for b in end {
    ///     if *b != SPACE {
    ///         non_space_count += 1;
    ///     }
    /// }
    /// assert_eq!(non_space_count, 36);
    /// ```
    #[inline(always)]
    pub fn from_byte_slice<'a>(s: &'a [u8]) -> (&'a [u8], &'a [Self], &'a [u8]) {
        const ALIGN: usize = core::mem::align_of::<u8x8>();
        let addr = s.as_ptr().cast::<u8>() as usize;
        let prior_count = core::cmp::min(addr.next_multiple_of(ALIGN) - addr, s.len());
        let (prior, remain) = s.split_at(prior_count);
        if remain.is_empty() {
            let empty = unsafe { core::slice::from_raw_parts(core::ptr::dangling::<Self>(), 0) };
            return (prior, empty, remain);
        }
        let s = remain; // now guaranteed to be correctly aligned for u64
        let u8x8_len = s.len() / 8;
        let use_len = u8x8_len * 8;
        let (for_u8u8, remain) = s.split_at(use_len);
        let ptr = for_u8u8.as_ptr().cast::<Self>();
        let u8x8s = unsafe { core::slice::from_raw_parts(ptr, u8x8_len) };
        (prior, u8x8s, remain)
    }

    /// Reinterprets the given byte slice as a slice of [`u8x8`], along with
    /// individual leading and trailing bytes that are not aligned for
    /// interpretation as `u64`.
    ///
    /// This is a mutable version of [`Self::from_byte_slice`].
    #[inline(always)]
    pub fn from_byte_slice_mut<'a>(
        s: &'a mut [u8],
    ) -> (&'a mut [u8], &'a mut [Self], &'a mut [u8]) {
        const ALIGN: usize = core::mem::align_of::<u8x8>();
        let addr = s.as_ptr().cast::<u8>() as usize;
        let prior_count = core::cmp::min(addr.next_multiple_of(ALIGN) - addr, s.len());
        let (prior, remain) = s.split_at_mut(prior_count);
        if remain.is_empty() {
            let empty =
                unsafe { core::slice::from_raw_parts_mut(core::ptr::dangling_mut::<Self>(), 0) };
            return (prior, empty, remain);
        }
        let s = remain; // now guaranteed to be correctly aligned for u64
        let u8x8_len = s.len() / 8;
        let use_len = u8x8_len * 8;
        let (for_u8u8, remain) = s.split_at_mut(use_len);
        let ptr = for_u8u8.as_mut_ptr().cast::<Self>();
        let u8x8s = unsafe { core::slice::from_raw_parts_mut(ptr, u8x8_len) };
        (prior, u8x8s, remain)
    }

    #[inline(always)]
    pub(crate) const fn new(n: u64) -> Self {
        Self { n }
    }

    /// Returns a [`u8x8`] with `v` in all eight of its elements.
    #[inline(always)]
    pub const fn splat(v: u8) -> Self {
        Self::new(v as u64 * ALL_ONES)
    }

    /// Converts the vector into an array of eight `u8` values.
    #[inline(always)]
    pub const fn to_array(self) -> [u8; 8] {
        self.n.to_ne_bytes()
    }

    /// Computes the bitwise complement of each element in the vector.
    #[inline(always)]
    pub const fn complement(self) -> Self {
        Self::new(!self.n)
    }

    /// Computes a bitwise OR result for each element across both vectors.
    #[inline(always)]
    pub const fn bitor(self, other: Self) -> Self {
        Self::new(self.n | other.n)
    }

    /// Computes a bitwise AND result for each element across both vectors.
    #[inline(always)]
    pub const fn bitand(self, other: Self) -> Self {
        Self::new(self.n & other.n)
    }

    /// Computes a bitwise XOR result for each element across both vectors.
    #[inline(always)]
    pub const fn bitxor(self, other: Self) -> Self {
        Self::new(self.n ^ other.n)
    }

    /// Compares each element across both vectors and returns a mask value
    /// where `true` represents equality and `false` represents inequality.
    #[inline(always)]
    pub const fn equals(self, other: Self) -> mask8x8 {
        let xo = self.n ^ other.n;
        let lo = ((xo & WITHOUT_HIGH_BITS) + WITHOUT_HIGH_BITS) | xo;
        let hi = !lo & ONLY_HIGH_BITS;
        // hi now has the msb set in each element that was equal,
        // but our mask representation wants lsb set so we'll shift.
        mask8x8::new(hi >> 7)
    }

    /// Compares each element across both vectors and returns a mask value
    /// with elements set to `true` where the corresponding element in `self`
    /// is less than the corresponding element in `other`.
    #[inline(always)]
    pub const fn less_than(self, other: Self) -> mask8x8 {
        let diff = (self.n | ONLY_HIGH_BITS).wrapping_sub(other.n & !ONLY_HIGH_BITS);
        let select =
            ((self.n & (self.n ^ other.n)) | (diff & !(self.n ^ other.n))) & ONLY_HIGH_BITS;
        let inv = select ^ ONLY_HIGH_BITS;
        // inv now has the msb set in each element that was equal,
        // but our mask representation wants lsb set so we'll shift.
        mask8x8::new(inv >> 7)
    }

    /// Compares each element across both vectors and returns a mask value
    /// with elements set to `true` where the corresponding element in `self`
    /// is less than the corresponding element in `other`.
    #[inline(always)]
    pub const fn greater_than(self, other: Self) -> mask8x8 {
        let diff = (other.n | ONLY_HIGH_BITS).wrapping_sub(self.n & !ONLY_HIGH_BITS);
        let select =
            ((other.n & (other.n ^ self.n)) | (diff & !(other.n ^ self.n))) & ONLY_HIGH_BITS;
        let inv = select ^ ONLY_HIGH_BITS;
        // inv now has the msb set in each element that was equal,
        // but our mask representation wants lsb set so we'll shift.
        mask8x8::new(inv >> 7)
    }

    /// Implements addition across corresponding elements, modulo 256.
    #[inline(always)]
    pub const fn wrapping_add(self, other: Self) -> Self {
        let low = (self.n & WITHOUT_HIGH_BITS).wrapping_add(other.n & WITHOUT_HIGH_BITS);
        Self::new(low ^ ((self.n ^ other.n) & ONLY_HIGH_BITS))
    }

    /// Implements addition across corresponding elements, saturating at the
    /// maximum value 255.
    #[inline(always)]
    pub const fn saturating_add(self, other: Self) -> Self {
        let sum = self.wrapping_add(other).n;
        let carry = ((self.n & other.n) | ((self.n | other.n) & !sum)) & ONLY_HIGH_BITS;
        Self::new(sum | msb_mask(carry))
    }

    /// Implements subtraction across corresponding elements, modulo 256.
    #[inline(always)]
    pub const fn wrapping_sub(self, other: Self) -> Self {
        Self::new(
            (self.n | ONLY_HIGH_BITS).wrapping_sub(other.n & WITHOUT_HIGH_BITS)
                ^ ((self.n ^ !other.n) & ONLY_HIGH_BITS),
        )
    }

    /// Implements subtraction across corresponding elements, saturating at the
    /// minimum value 0.
    #[inline(always)]
    pub const fn saturating_sub(self, other: Self) -> Self {
        let diff = self.wrapping_sub(other).n;
        let borrow = ((!self.n & other.n) | ((!self.n | other.n) & diff)) & ONLY_HIGH_BITS;
        Self::new(diff & !msb_mask(borrow))
    }

    /// Computes the absolute difference between corresponding elements.
    #[inline(always)]
    pub const fn abs_difference(self, other: Self) -> Self {
        let diff = self.n.wrapping_sub(other.n);
        let borrow = ((!self.n & other.n) | ((!self.n | other.n) & diff)) & ONLY_HIGH_BITS;
        let msb_mask = msb_mask(borrow);
        let lo = (self.n & !msb_mask) | (other.n & msb_mask);
        let hi = (self.n & msb_mask) | (other.n & !msb_mask);
        Self::new(
            ((lo | ONLY_HIGH_BITS) - (hi & WITHOUT_HIGH_BITS)) ^ ((lo ^ !hi) & ONLY_HIGH_BITS),
        )
    }

    /// Finds the maximum value for each element across both vectors.
    #[inline(always)]
    pub const fn max(self, other: Self) -> Self {
        let diff = self.n.wrapping_sub(other.n);
        let borrow = ((!self.n & other.n) | ((!self.n | other.n) & diff)) & ONLY_HIGH_BITS;
        let msb_mask = msb_mask(borrow);
        Self::new((self.n & !msb_mask) | (other.n & msb_mask))
    }

    /// Finds the minimum value for each element across both vectors.
    #[inline(always)]
    pub const fn min(self, other: Self) -> Self {
        let diff = self.n.wrapping_sub(other.n);
        let borrow = ((!self.n & other.n) | ((!self.n | other.n) & diff)) & ONLY_HIGH_BITS;
        let msb_mask = msb_mask(borrow);
        Self::new((self.n & msb_mask) | (other.n & !msb_mask))
    }

    /// Finds the integer mean value for each element across both vectors.
    ///
    /// This is conceptually the same as (self + other)/2, computed without overflow.
    #[inline(always)]
    pub const fn mean(self, other: Self) -> Self {
        let shared = self.n & other.n;
        let diff = (self.n ^ other.n) & 0xfefefefefefefefe;
        Self::new(shared + (diff >> 1))
    }

    /// Counts the number of bits set in each element.
    #[inline(always)]
    pub const fn popcount(self) -> Self {
        let a = self.n - ((self.n >> 1) & 0x5555555555555555);
        let b = (a & 0x3333333333333333).wrapping_add((a >> 2) & 0x3333333333333333);
        Self::new((b + (b >> 4)) & 0x0f0f0f0f0f0f0f0f)
    }
}

impl core::ops::Not for u8x8 {
    type Output = Self;

    /// Implements the unary `!` operator using [`Self::complement`].
    fn not(self) -> Self {
        self.complement()
    }
}

impl core::ops::BitOr for u8x8 {
    type Output = Self;

    /// Implements the `|` operator using [`Self::bitor`].
    fn bitor(self, rhs: Self) -> Self {
        self.bitor(rhs)
    }
}

impl core::ops::BitOrAssign for u8x8 {
    /// Implements the `|=` operator using [`Self::bitor`].
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

impl core::ops::BitAnd for u8x8 {
    type Output = Self;

    /// Implements the `&` operator using [`Self::bitand`].
    fn bitand(self, rhs: Self) -> Self {
        self.bitand(rhs)
    }
}

impl core::ops::BitAndAssign for u8x8 {
    /// Implements the `&=` operator using [`Self::bitand`].
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

impl core::ops::BitXor for u8x8 {
    type Output = Self;

    /// Implements the `^` operator using [`Self::bitxor`].
    fn bitxor(self, rhs: Self) -> Self {
        self.bitxor(rhs)
    }
}

impl core::ops::BitXorAssign for u8x8 {
    /// Implements the `^=` operator using [`Self::bitxor`].
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
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
        *self = self.wrapping_add(rhs);
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
        *self = self.wrapping_sub(rhs);
    }
}

impl IntoIterator for u8x8 {
    type Item = u8;
    type IntoIter = core::array::IntoIter<u8, 8>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.to_array().into_iter()
    }
}

impl core::fmt::Debug for u8x8 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "u8x8({:?})", self.to_array())
    }
}

/// Raw representation of a vector where all bytes are 1.
pub(crate) const ALL_ONES: u64 = 0x0101010101010101;

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

#[inline(always)]
const fn msb_mask(n: u64) -> u64 {
    (n >> 7) * 255
}
