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
    let mut next_next = 0;
    let mut next = 1;
    for i in (0..n).rev() {
        let mut cur = 0;
        if s[i] != b'0' {
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
            curr %= MOD;
            if i + 1 < n {
                match (bytes[i] == b'*', bytes[i + 1] == b'*') {
                    (true, true) => {
                        curr += next_next * 15;
                    }
                    (true, false) => {
                        if bytes[i + 1] > b'6' {
                            curr += next_next * 1;
                        } else {
                            curr += next_next * 2;
                        }
                    }
                    (false, false) => {
                        if bytes[i] == b'1' || bytes[i] == b'2' && bytes[i + 1] <= b'6' {
                            curr += next_next;
                        }
                    }
                    (false, true) => {
                        if bytes[i] == b'1' {
                            curr += next_next * 9;
                        } else if bytes[i] == b'2' {
                            curr += next_next * 6;
                        }
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
 * 264. 丑数 II
 * https://leetcode.cn/problems/ugly-number-ii/description/
 */
pub fn nth_ugly_number(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![0; n + 1];
    dp[1] = 1;

    let mut two = 1;
    let mut three = 1;
    let mut five = 1;

    for i in 2..=n {
        let a = dp[two] * 2;
        let b = dp[three] * 3;
        let c = dp[five] * 5;

        let curr = a.min(b).min(c);
        // 注意 这里不能用else if
        if curr == a {
            two += 1;
        }
        if curr == b {
            three += 1;
        }
        if curr == c {
            five += 1;
        }
        dp[i] = curr;
    }

    dp[n]
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

/**
 * 940. 不同的子序列 II
 * https://leetcode.cn/problems/distinct-subsequences-ii/description/
 */
pub fn distinct_subseq_ii(s: String) -> i32 {
    const MOD: i64 = 1_000_000_007;

    let chars = s.chars().collect::<Vec<char>>();
    let mut dp = vec![0; 26];
    let start = 'a' as usize;
    let mut all = 1;

    for &char in &chars {
        let index = char as usize - start;
        let increment = (all - dp[index] + MOD) % MOD;
        dp[index] = (dp[index] + increment) % MOD;
        all = (all + increment) % MOD;
    }

    ((all - 1 + MOD) % MOD) as i32
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_mincost_tickets() {
        assert_eq!(mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]), 11);
        assert_eq!(
            mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15]),
            17
        );
    }

    #[test]
    fn test_num_decodings() {
        assert_eq!(num_decodings("12".to_string()), 2);
        assert_eq!(num_decodings("226".to_string()), 3);
        assert_eq!(num_decodings("06".to_string()), 0);
    }

    #[test]
    fn test_num_decodings_ii() {
        assert_eq!(num_decodings_ii("*".to_string()), 9);
        assert_eq!(num_decodings_ii("1*".to_string()), 18);
        assert_eq!(num_decodings_ii("2*".to_string()), 15);
        assert_eq!(num_decodings_ii("**".to_string()), 96);
    }

    #[test]
    fn test_nth_ugly_number() {
        assert_eq!(nth_ugly_number(1), 1);
    }
}
