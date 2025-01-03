//! 三维动态规划专题

/**
 * 474. 一和零
 * https://leetcode.cn/problems/ones-and-zeroes/description/
 */
pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for s in strs {
        let zeros = s.chars().filter(|&c| c == '0').count();
        let ones = s.chars().filter(|&c| c == '1').count();
        for i in (zeros..=m).rev() {
            for j in (ones..=n).rev() {
                dp[i][j] = dp[i][j].max(1 + dp[i - zeros][j - ones]);
            }
        }
    }

    dp[m][n]
}

/**
 * 87. 扰乱字符串
 * https://leetcode.cn/problems/scramble-string/description/
 */
pub fn is_scramble(s1: String, s2: String) -> bool {
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    let n = chars1.len();
    let mut dp = vec![vec![vec![false; n + 1]; n]; n];

    for i in 0..n {
        for j in 0..n {
            if chars1[i] == chars2[j] {
                dp[i][j][1] = true;
            };
        }
    }

    for len in 2..=n {
        for l1 in 0..=(n - len) {
            for l2 in 0..=(n - len) {
                for k in 1..len {
                    if dp[l1][l2][k] && dp[l1 + k][l2 + k][len - k] {
                        dp[l1][l2][len] = true;
                        break;
                    }
                }
                if !dp[l1][l2][len] {
                    let mut i = l1 + 1;
                    let mut j = l2 + len - 1;
                    for k in 1..len {
                        if dp[l1][j][k] && dp[i][l2][len - k] {
                            dp[l1][l2][len] = true;
                            break;
                        }
                        i += 1;
                        j -= 1;
                    }
                }
            }
        }
    }

    dp[0][0][n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_form() {
        assert_eq!(
            find_max_form(
                vec![
                    "10".to_string(),
                    "0001".to_string(),
                    "111001".to_string(),
                    "1".to_string(),
                    "0".to_string()
                ],
                5,
                3
            ),
            4
        );
        assert_eq!(
            find_max_form(
                vec!["10".to_string(), "0".to_string(), "1".to_string()],
                1,
                1
            ),
            2
        );
    }
}
