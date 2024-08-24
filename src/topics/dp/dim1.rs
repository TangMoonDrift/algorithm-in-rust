//! 一维动态规划专题

/**
 * 91. 解码方法
 * https://leetcode.cn/problems/decode-ways/description/
 */
pub fn num_decodings(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();

    // let mut dp = vec![0; n + 1];
    // dp[n] = 1;
    // for i in (0..n).rev() {
    //     if s[i] == b'0' {
    //         dp[i] = 0;
    //     } else {
    //         dp[i] = dp[i + 1];
    //         if i + 1 < n && (s[i] == b'1' || s[i] == b'2' && s[i + 1] <= b'6') {
    //             dp[i] += dp[i + 2];
    //         }
    //     }
    // }

    // dp[0]

    let mut next_next = 0;
    let mut next = 1;
    for i in (0..n).rev() {
        let mut cur;
        if s[i] == b'0' {
            cur = 0;
        } else {
            cur = next;
            if i + 1 < n && (s[i] == b'1' || s[i] == b'2' && s[i + 1] <= b'6') {
                cur += next_next;
            }
        }
        next_next = next;
        next = cur;
    }

    next
}

/**
 * 32. 最长有效括号
 * https://leetcode.cn/problems/longest-valid-parentheses/description/
 */
pub fn longest_valid_parentheses(s: String) -> i32 {
    let n = s.len();
    let chars: Vec<char> = s.chars().collect();

    let mut dp = vec![0; n];
    let mut ans = 0;

    for i in 1..n {
        if chars[i] == ')' {
            let index = (i - dp[i - 1] - 1) as i32;
            if index >= 0 && chars[index as usize] == '(' {
                dp[i] = dp[i - 1] + 2;
                if index >= 1 {
                    dp[i] += dp[index as usize - 1]
                }
            }
        }
        ans = ans.max(dp[i] as i32);
    }

    ans
}
