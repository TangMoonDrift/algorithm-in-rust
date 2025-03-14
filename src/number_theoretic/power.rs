/**
 * 乘法快速幂
 */
pub fn power(mut base: i32, mut exponent: i32, model: i32) -> i32 {
    let mut ans = 1;
    while exponent > 0 {
        if exponent & 1 == 1 {
            ans = (ans * base) % model;
        }
        base = (base * base) % model;
        exponent >>= 1;
    }

    ans
}
