use super::*;

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
