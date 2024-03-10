pub fn print_binary(num: i32) {
    let mut i = 31;
    let mut result = String::new();
    while i >= 0 {
        match num & (1 << i) {
            0 => {
                result += "0";
            }
            _ => {
                result += "1";
            }
        }
        i -= 1;
    }
    println!("{}", result);
}

/**
 * https://leetcode.cn/problems/missing-number/description/
 * 给定一个包含 [0, n] 中 n 个数的数组 nums ，找出 [0, n] 这个范围内没有出现在数组中的那个数。
 */
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    let mut eor_all = 0;
    let mut eor_has = 0;
    for i in 0..n {
        eor_all ^= i;
        eor_has ^= nums[i as usize];
    }
    eor_all ^= n;
    eor_all ^ eor_has
}
