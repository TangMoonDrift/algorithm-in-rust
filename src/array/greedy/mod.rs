/**
 * 题目描述：https://leetcode.cn/problems/largest-number/description/
 * 给定一组非负整数 nums，重新排列每个数的顺序（每个数不可拆分）使之组成一个最大的整数。
 */
pub fn largest_number(nums: Vec<i32>) -> String {
    let mut result = nums.clone();
    result.sort_by(|a, b| format!("{}{}", b, a).cmp(&format!("{}{}", a, b)));

    if result[0] == 0 {
        return "0".to_string();
    }

    result
        .into_iter()
        .map(|i| i.to_string())
        .collect::<String>()
}
