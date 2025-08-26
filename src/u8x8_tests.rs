use super::*;

#[test]
pub fn equals() {
    let a = u8x8::from_array([1, 2, 5, 6, 9, 10, 255, 255]);
    let b = u8x8::from_array([1, 3, 5, 7, 9, 10, 255, 127]);
    let got = a.equals(b);
    let want = mask8x8::from_array([true, false, true, false, true, true, true, false]);
    assert_eq!(got, want);
}

#[test]
pub fn less_than() {
    let a = u8x8::from_array([1, 2, 5, 7, 9, 9, 255, 255]);
    let b = u8x8::from_array([1, 3, 5, 6, 9, 10, 255, 127]);
    let got = a.less_than(b);
    let want = mask8x8::from_array([false, true, false, false, false, true, false, false]);
    assert_eq!(got, want);
}

#[test]
pub fn greater_than() {
    let a = u8x8::from_array([1, 2, 5, 7, 9, 9, 255, 255]);
    let b = u8x8::from_array([1, 3, 5, 6, 9, 10, 255, 127]);
    let got = a.greater_than(b);
    let want = mask8x8::from_array([false, false, false, true, false, false, false, true]);
    assert_eq!(got, want);
}

#[test]
pub fn wrapping_add() {
    let a = u8x8::from_array([1, 2, 3, 4, 255, 254, 0, 0]);
    let b = u8x8::from_array([5, 6, 7, 8, 2, 2, 5, 2]);
    let got = a.wrapping_add(b);
    let want = u8x8::from_array([6, 8, 10, 12, 1, 0, 5, 2]);
    assert_eq!(got, want);
}

#[test]
pub fn saturating_add() {
    let a = u8x8::from_array([1, 2, 3, 4, 255, 254, 0, 0]);
    let b = u8x8::from_array([5, 6, 7, 8, 2, 2, 5, 2]);
    let got = a.saturating_add(b);
    let want = u8x8::from_array([6, 8, 10, 12, 255, 255, 5, 2]);
    assert_eq!(got, want);
}

#[test]
pub fn collect_sum() {
    let values = u8x8::from_array([1, 2, 3, 4, 255, 128, 0, 9]);
    assert_eq!(values.collect_sum(), 402);
}

#[test]
pub fn wrapping_sub() {
    let a = u8x8::from_array([6, 8, 10, 12, 1, 0, 5, 2]);
    let b = u8x8::from_array([1, 2, 3, 4, 255, 254, 0, 0]);
    let got = a.wrapping_sub(b);
    let want = u8x8::from_array([5, 6, 7, 8, 2, 2, 5, 2]);
    assert_eq!(got, want);
}

#[test]
pub fn saturating_sub() {
    let a = u8x8::from_array([6, 8, 10, 12, 1, 0, 5, 2]);
    let b = u8x8::from_array([1, 2, 3, 4, 255, 254, 0, 0]);
    let got = a.saturating_sub(b);
    let want = u8x8::from_array([5, 6, 7, 8, 0, 0, 5, 2]);
    assert_eq!(got, want);
}

#[test]
pub fn abs_difference() {
    let a = u8x8::from_array([6, 8, 10, 12, 1, 0, 5, 2]);
    let b = u8x8::from_array([1, 2, 3, 4, 255, 254, 0, 0]);
    let got = a.abs_difference(b);
    let want = u8x8::from_array([5, 6, 7, 8, 254, 254, 5, 2]);
    assert_eq!(got, want);
}

#[test]
pub fn max() {
    let a = u8x8::from_array([6, 8, 10, 12, 1, 0, 0, 2]);
    let b = u8x8::from_array([1, 2, 3, 4, 255, 254, 5, 2]);
    let got = a.max(b);
    let want = u8x8::from_array([6, 8, 10, 12, 255, 254, 5, 2]);
    assert_eq!(got, want);
}

#[test]
pub fn min() {
    let a = u8x8::from_array([6, 8, 10, 12, 1, 0, 0, 2]);
    let b = u8x8::from_array([1, 2, 3, 4, 255, 254, 5, 2]);
    let got = a.min(b);
    let want = u8x8::from_array([1, 2, 3, 4, 1, 0, 0, 2]);
    assert_eq!(got, want);
}

#[test]
pub fn mean() {
    let a = u8x8::from_array([0, 1, 2, 3, 127, 128, 254, 255]);
    let b = u8x8::from_array([255, 255, 254, 3, 255, 0, 64, 0]);
    let got = a.mean(b);
    let want = u8x8::from_array([127, 128, 128, 3, 191, 64, 159, 127]);
    assert_eq!(got, want);
}

#[test]
pub fn popcount() {
    let a = u8x8::from_array([0x00, 0x01, 0x10, 0x03, 0x0f, 0xf0, 0xff, 0xfe]);
    let got = a.popcount();
    let want = u8x8::from_array([0, 1, 1, 2, 4, 4, 8, 7]);
    assert_eq!(got, want);
}
