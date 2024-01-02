// 求最大公约数
pub fn gcd(m: u64, n: u64) -> u64 {
    // assert!(m != 0 && n != 0);
    // if m > n {
    //     return gcd(m - n, n);
    // } else if m < n {
    //     return gcd(n - m, m);
    // }
    // m

    match n {
        0 => m,
        _ => {
            let r = m % n;
            gcd(n, r)
        }
    }
}
