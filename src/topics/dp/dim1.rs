//! 一维动态规划专题

/**
 * 91. 解码方法
 * https://leetcode.cn/problems/decode-ways/description/
 */
pub fn num_decodings(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut dp = vec![0; n + 1];

    dp[n] = 1;

    for i in (0..n).rev() {
        if s[i] == b'0' {
            dp[i] = 0;
        } else {
            dp[i] = dp[i + 1];
            if i + 1 < n && (s[i] == b'1' || s[i] == b'2' && s[i + 1] <= b'6') {
                dp[i] += dp[i + 2];
            }
        }
    }

    dp[0]
}
