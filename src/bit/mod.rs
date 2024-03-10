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

/**
 * https://leetcode.cn/problems/single-number/description/
 * 给你一个 非空 整数数组 nums ，除了某个元素只出现一次以外，其余每个元素均出现两次(或偶数次)。找出那个只出现了一次的元素。
 * 你必须设计并实现线性时间复杂度的算法来解决此问题，且该算法只使用常量额外空间。
 */
pub fn single_number_i(nums: Vec<i32>) -> i32 {
    let mut eor = 0;
    nums.into_iter().for_each(|num| {
        eor ^= num;
    });
    eor
}

/**
 * https://leetcode.cn/problems/single-number-ii/
 * 给你一个整数数组 nums ，除某个元素仅出现 一次 外，其余每个元素都恰出现 三次 。
 * 请你找出并返回那个只出现了一次的元素。
 * 你必须设计并实现线性时间复杂度的算法且使用常数级空间来解决此问题。
 */
pub fn single_number_ii(nums: Vec<i32>) -> i32 {
    let find = |array: &Vec<i32>, m: usize| {
        let mut bits = [0; 32];
        for num in array {
            for i in 0..32 {
                bits[i] += (num >> i) & 1;
            }
        }
        let mut ans = 0;
        for i in 0..32 {
            if bits[i] % m as i32 != 0 {
                ans |= 1 << i;
            }
        }
        ans
    };

    find(&nums, 3)
}

/**
 * https://leetcode.cn/problems/single-number-iii/description/
 * 给你一个整数数组 nums，其中恰好有两个元素只出现一次，其余所有元素均出现两次。
 * 找出只出现一次的那两个元素。你可以按 任意顺序 返回答案。
 * 你必须设计并实现线性时间复杂度的算法且仅使用常量额外空间来解决此问题。
 */
pub fn two_single_number_iii(nums: Vec<i32>) -> Vec<i32> {
    let eor_1 = nums.iter().fold(0, |r, v| r ^ v);
    let right_one = eor_1 & -eor_1;
    let mut eor_2 = 0;
    for num in &nums {
        if (right_one & num) == 0 {
            eor_2 ^= num;
        }
    }
    vec![eor_1 ^ eor_2, eor_2]
}

/**
 * https://leetcode.cn/problems/power-of-two/
 * 给你一个整数 n，请你判断该整数是否是 2 的幂次方。如果是，返回 true ；否则，返回 false 。
 */
pub fn is_power_of_two(n: i32) -> bool {
    n > 0 && n == (n & -n)
}
