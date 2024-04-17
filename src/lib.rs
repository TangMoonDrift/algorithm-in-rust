// 求最大公约数
pub fn gcd(mut m: usize, mut n: usize) -> usize {
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

fn lcm(mut m: usize, mut n: usize) -> usize {
    if m < n {
        m ^= n;
        n ^= m;
        m ^= n;
    }
    m / gcd(m, n) * n
}

#[cfg(test)]
mod test {
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
}
