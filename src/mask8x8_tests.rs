use super::*;

#[test]
pub fn from_bitmask_bigend() {
    let got = mask8x8::from_bitmask_bigend(0b10101010);
    let want = mask8x8::from_array([true, false, true, false, true, false, true, false]);
    assert_eq!(got, want);
}

#[test]
pub fn from_bitmask_littleend() {
    let got = mask8x8::from_bitmask_littleend(0b10101010);
    let want = mask8x8::from_array([false, true, false, true, false, true, false, true]);
    assert_eq!(got, want);
}

#[test]
pub fn or() {
    let a = mask8x8::from_array([true, false, true, false, true, true, false, false]);
    let b = mask8x8::from_array([true, true, false, false, true, false, true, false]);
    let got = a.or(b);
    let want = mask8x8::from_array([true, true, true, false, true, true, true, false]);
    assert_eq!(got, want);
}

#[test]
pub fn and() {
    let a = mask8x8::from_array([true, false, true, false, true, true, false, false]);
    let b = mask8x8::from_array([true, true, false, false, true, false, true, false]);
    let got = a.and(b);
    let want = mask8x8::from_array([true, false, false, false, true, false, false, false]);
    assert_eq!(got, want);
}
