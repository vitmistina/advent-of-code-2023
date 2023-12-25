use super::*;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lowest_common_multiple(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(54, 24), 6);
    assert_eq!(gcd(48, 180), 12);
    assert_eq!(gcd(7, 13), 1);
}

#[test]
fn test_lcm() {
    assert_eq!(lowest_common_multiple(4, 6), 12);
    assert_eq!(lowest_common_multiple(21, 6), 42);
    assert_eq!(lowest_common_multiple(8, 9), 72);
    assert_eq!(lowest_common_multiple(17, 19), 323); // Prime numbers
}
