pub mod maximum_sum_of_subarrays;
pub mod search;
pub mod sort;

/**
 * 1371. 每个元音包含偶数次的最长子字符串
 * https://leetcode.cn/problems/find-the-longest-substring-containing-vowels-in-even-counts/description/
 */
pub fn find_the_longest_substring(s: String) -> i32 {
    let mut map = vec![-2; 32];
    map[0] = -1;
    let mut ans = 0;
    let mut status = 0;

    for (i, c) in s.chars().enumerate() {
        let m = f(c);
        if m != -1 {
            status ^= 1 << m;
        }
        if map[status] != -2 {
            ans = ans.max(i as i32 - map[status]);
        } else {
            map[status] = i as i32;
        }
    }

    fn f(c: char) -> i32 {
        match c {
            'a' => 0,
            'e' => 1,
            'i' => 2,
            'o' => 3,
            'u' => 4,
            _ => -1,
        }
    }

    ans
}
