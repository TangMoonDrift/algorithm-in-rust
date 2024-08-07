//! 双指针专题

/**
 * 475. 供暖器
 * https://leetcode.cn/problems/heaters/
 */
pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
    let n = houses.len();
    let mut houses = houses.clone();
    houses.sort();
    let mut heaters = heaters.clone();
    heaters.sort();

    let mut l = 0;
    let mut r = 0;
    let mut ans = 0;

    fn absolute_value(x: i32) -> i32 {
        if x < 0 {
            -x
        } else {
            x
        }
    }

    fn is_best(houses: &Vec<i32>, heaters: &Vec<i32>, i: usize, j: usize) -> bool {
        j == heaters.len() - 1
            || absolute_value(houses[i] - heaters[j]) < absolute_value(houses[i] - heaters[j + 1])
    }

    while l < n {
        while !is_best(&houses, &heaters, l, r) {
            r += 1;
        }
        let dis = absolute_value(houses[l] - heaters[r]);
        ans = ans.max(dis);
        l += 1;
    }

    ans
}
