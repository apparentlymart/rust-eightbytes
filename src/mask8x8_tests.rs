use super::*;

#[test]
pub fn from_bitmask_be() {
    let got = mask8x8::from_bitmask_be(0b10101010);
    let want = mask8x8::from_array([true, false, true, false, true, false, true, false]);
    assert_eq!(got, want);
}

#[test]
pub fn to_bitmask_be() {
    let mask = mask8x8::from_array([true, false, true, false, true, false, true, false]);
    let got = mask.to_bitmask_be();
    assert_eq!(got, 0b10101010);
}

#[test]
pub fn from_bitmask_le() {
    let got = mask8x8::from_bitmask_le(0b10101010);
    let want = mask8x8::from_array([false, true, false, true, false, true, false, true]);
    assert_eq!(got, want);
}

#[test]
pub fn to_bitmask_le() {
    let mask = mask8x8::from_array([true, false, true, false, true, false, true, false]);
    let got = mask.to_bitmask_le();
    assert_eq!(got, 0b01010101);
}

#[test]
pub fn not() {
    let a = mask8x8::from_array([true, false, true, false, true, true, false, false]);
    let got = a.not();
    let want = mask8x8::from_array([false, true, false, true, false, false, true, true]);
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

#[test]
pub fn select() {
    let choices = mask8x8::from_array([true, false, true, false, true, true, false, false]);
    let got = choices.select(0xff, 0xbb);
    let want = u8x8::from_array([0xff, 0xbb, 0xff, 0xbb, 0xff, 0xff, 0xbb, 0xbb]);
    assert_eq!(got, want);
}

#[test]
pub fn count_true() {
    let choices = mask8x8::from_array([true, false, true, false, true, true, true, false]);
    assert_eq!(choices.count_true(), 5);
}

#[test]
pub fn count_false() {
    let choices = mask8x8::from_array([true, false, true, false, true, true, true, false]);
    assert_eq!(choices.count_false(), 3);
}
