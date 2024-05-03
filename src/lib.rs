// 求最大公约数
pub fn gcd(mut m: i32, mut n: i32) -> i32 {
    // assert!(m != 0 && n != 0);
    // if m > n {
    //     return gcd(m - n, n);
    // } else if m < n {
    //     return gcd(n - m, m);
    // }
    // m

    // 欧几里得算法
    // match n {
    //     0 => m,
    //     _ => {
    //         let r = m % n;
    //         gcd(n, r)
    //     }
    // }

    // stein 算法
    if m < n {
        m ^= n;
        n ^= m;
        m ^= n;
    }
    if n == 0 {
        return m;
    }
    if m % 2 == 0 && n % 2 == 0 {
        gcd(m >> 1, n >> 1) << 1
    } else if m % 2 == 0 && n % 2 != 0 {
        gcd(m >> 1, n)
    } else if m % 2 != 0 && n % 2 == 0 {
        gcd(m, n >> 1)
    } else {
        gcd(m - n, n)
    }
}

pub fn lcm(mut m: i32, mut n: i32) -> i32 {
    if m < n {
        m ^= n;
        n ^= m;
        m ^= n;
    }
    m / gcd(m, n) * n
}

pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
    let lcm = lcm(a, b) as i64;
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
    use crate::*;

    #[test]
    fn check_gcd() {
        assert_eq!(gcd(72, 30), 6);
        assert_eq!(gcd(30, 72), 6);
    }

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
