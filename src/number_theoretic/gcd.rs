//! GCD(Greatest Common Divisor) algorithms.
//!
//! This module provides several algorithms for computing the greatest common divisor (GCD) of two numbers.

pub fn gcd_sub(m: u64, n: u64) -> u64 {
    assert!(m > 0 && n > 0);
    if m > n {
        gcd_sub(m - n, n);
    } else {
        gcd_sub(m, n - m);
    }
    m
}

pub fn gcd_euclid(m: u64, n: u64) -> u64 {
    assert!(m > 0 && n > 0);
    match n {
        0 => m,
        _ => gcd_euclid(n, m % n),
    }
}

pub fn gcd_stein(mut m: u64, mut n: u64) -> u64 {
    if m < n {
        m ^= n;
        n ^= m;
        m ^= n;
    }
    if n == 0 {
        return m;
    }
    if m % 2 == 0 && n % 2 == 0 {
        gcd_stein(m >> 1, n >> 1) << 1
    } else if m % 2 == 0 && n % 2 != 0 {
        gcd_stein(m >> 1, n)
    } else if m % 2 != 0 && n % 2 == 0 {
        gcd_stein(m, n >> 1)
    } else {
        gcd_stein(m - n, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_sub() {
        assert_eq!(gcd_sub(12, 8), 4);
        assert_eq!(gcd_sub(10, 15), 5);
        assert_eq!(gcd_sub(21, 14), 7);
    }

    #[test]
    fn test_gcd_euclid() {
        assert_eq!(gcd_euclid(12, 8), 4);
        assert_eq!(gcd_euclid(10, 15), 5);
        assert_eq!(gcd_euclid(21, 14), 7);
    }

    #[test]
    fn test_gcd_stein() {
        assert_eq!(gcd_stein(12, 8), 4);
        assert_eq!(gcd_stein(10, 15), 5);
        assert_eq!(gcd_stein(21, 14), 7);
    }
}
