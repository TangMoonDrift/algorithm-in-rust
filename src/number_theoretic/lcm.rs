//! LCM (Least Common Multiple) algorithms.
//!
//! This module provides functions to compute the least common multiple (LCM) of two integers.
//! The LCM of two integers `a` and `b` is the smallest positive integer that is divisible by both `a` and `b`.
//!
//! # Examples
//!
//! ```
//! use algorithm::number_theoretic::lcm;
//!
//! assert_eq!(lcm(4, 6), 12);
//! assert_eq!(lcm(21, 6), 42);
//! assert_eq!(lcm(0, 5), 0);
//! ```
//!
//! # Implementation Details
//!
//! The LCM is calculated using the formula:
//! ```text
//! LCM(a, b) = |a * b| / GCD(a, b)
//! ```
//! where `GCD(a, b)` is the greatest common divisor of `a` and `b`.
//!
//! # Performance
//!
//! The time complexity of the LCM calculation is O(log(min(a, b))).
//!
//! # References
//!
//! - [Least Common Multiple](https://en.wikipedia.org/wiki/Least_common_multiple)
//! - [Euclidean Algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm)
//! - [Greatest Common Divisor](https://en.wikipedia.org/wiki/Greatest_common_divisor)
//! - [Least Common Multiple](https://en.wikipedia.org/wiki/Least_common_multiple)
//! - [Least Common Multiple](https://en.wikipedia.org/wiki/Least_common_multiple)
use super::*;
pub fn lcm(mut m: u64, mut n: u64) -> u64 {
    if m == 0 || n == 0 {
        0
    } else {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m / gcd_stein(m, n) * n
    }
}

// 求第 n 个神奇数字
pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
    let lcm = lcm(a as u64, b as u64) as i64;
    let (mut ans, mut l, mut r): (i64, i64, i64) = (0, 0, n as i64 * lcm);
    while l <= r {
        let m = (r - l) >> 1 + l;
        if m / a as i64 + m / b as i64 - m / lcm as i64 >= n as i64 {
            ans = m;
            r = m - 1;
        } else {
            l = m + 1;
        }
    }
    (ans % 1000_000_007) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_lcm() {
        assert_eq!(lcm(72, 30), 360);
        assert_eq!(lcm(30, 72), 360);
    }

    #[test]
    fn check_nth_magical_number() {
        assert_eq!(nth_magical_number(1, 1, 2), 1);
        assert_eq!(nth_magical_number(2, 1, 2), 2);
    }
}
