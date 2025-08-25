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
