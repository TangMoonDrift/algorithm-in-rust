//! 一维动态规划专题

/**
 * 983. 最低票价
 * https://leetcode.cn/problems/minimum-cost-for-tickets/description/
 */
pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
    const DURATION: [i32; 3] = [1, 7, 30];
    const ONE_YEAR: usize = 366;
    let mut dp = [i32::MAX; ONE_YEAR];
    let n = days.len();
    dp[n] = 0;

    for i in (0..n).rev() {
        for k in 0..3 {
            let mut j = i;
            while j < n && days[i] + DURATION[k] > days[j] {
                j += 1;
            }
            dp[i] = dp[i].min(costs[k] + dp[j]);
        }
    }

    dp[0]
}
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
 * 639. 解码方法 II
 * https://leetcode.cn/problems/decode-ways-ii/description/
 */
pub fn num_decodings_ii(s: String) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let bytes = s.as_bytes();
    let n = s.len();

    let mut next = 1;
    let mut next_next = 0;

    for i in (0..n).rev() {
        let mut curr = 0;
        if bytes[i] != b'0' {
            curr = if bytes[i] == b'*' { 9 } else { 1 } * next;
            if i + 1 < n {
                if bytes[i] != b'*' {
                    if bytes[i + 1] != b'*' {
                        if bytes[i] == b'1' || bytes[i] == b'2' && bytes[i + 1] <= b'6' {
                            curr += next_next;
                        }
                    } else {
                        if bytes[i] == b'1' {
                            curr += 9 * next_next;
                        } else if bytes[i] == b'2' {
                            curr += 6 * next_next;
                        }
                    }
                } else {
                    if bytes[i + 1] != b'*' {
                        if bytes[i + 1] <= b'6' {
                            curr += 2 * next_next;
                        } else {
                            curr += next_next;
                        }
                    } else {
                        curr += 15 * next_next;
                    }
                }
            }
            curr %= MOD;
        }
        next_next = next;
        next = curr;
    }

    next as i32
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
