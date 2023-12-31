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

/**
 * 题目描述：https://leetcode.cn/problems/two-city-scheduling/description/
 * 公司计划面试 2n 人。给你一个数组 costs ，其中 costs[i] = [aCosti, bCosti] 。第 i 人飞往 a 市的费用为 aCosti ，飞往 b 市的费用为 bCosti 。
 * 返回将每个人都飞到 a 、b 中某座城市的最低费用，要求每个城市都有 n 人抵达。
 */
pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
    let n = costs.len();
    let m = n / 2;

    let mut diff = costs
        .iter()
        .map(|prices| prices[1] - prices[0])
        .collect::<Vec<i32>>();
    diff.sort_unstable_by(|a, b| a.cmp(b));

    let sorted_diff = &diff[..m].iter().sum::<i32>();

    costs.iter().map(|prices| prices[0]).sum::<i32>() + sorted_diff
}
