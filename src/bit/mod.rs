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

/**
 * https://leetcode.cn/problems/bitwise-and-of-numbers-range/description/
 * 给你两个整数 left 和 right ，表示区间 [left, right] ，返回此区间内所有数字 按位与 的结果（包含 left 、right 端点）。
 */
pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
    let mut ans = right;
    while left < ans {
        ans -= ans & -ans;
    }
    ans
}

/**
 * https://leetcode.cn/problems/reverse-bits/
 * 颠倒给定的 32 位无符号整数的二进制位。
 */
pub fn reverse_bits(x: i32) -> i32 {
    let mut n = x;
    n = ((n & 0xaaaaaaaa_u32 as i32) >> 1) | ((n & 0x55555555) << 1);
    n = ((n & 0xcccccccc_u32 as i32) >> 2) | ((n & 0x33333333) << 2);
    n = ((n & 0xf0f0f0f0_u32 as i32) >> 4) | ((n & 0x0f0f0f0f) << 4);
    n = ((n & 0xff00ff00_u32 as i32) >> 8) | ((n & 0x00ff00ff) << 8);
    n = (n >> 16) | (n << 16);
    n
}

pub fn binary_add(mut a: i32, mut b: i32) -> i32 {
    let mut ans = a;
    while b != 0 {
        ans = a ^ b;
        b = (a & b) << 1;
        a = ans;
    }
    ans
}

pub fn binary_minus(a: i32, b: i32) -> i32 {
    let neg = |x: i32| {
        return binary_add(!x, 1);
    };
    binary_add(a, neg(b))
}

pub fn binary_mul(mut a: i32, mut b: i32) -> i32 {
    let mut ans = 0;
    while b != 0 {
        if (b & 1) != 0 {
            ans = binary_add(ans, a);
        }
        b >>= 1;
        a <<= 1;
    }
    ans
}

pub fn binary_div(a: i32, b: i32) -> i32 {
    let neg = |x: i32| {
        return binary_add(!x, 1);
    };
    let mut x = if a > 0 { a } else { neg(a) };
    let y = if b > 0 { b } else { neg(b) };
    let mut ans = 0;
    let mut i = 30;
    while i >= 0 {
        if (x >> i) >= y {
            ans |= 1 << i;
            x = binary_minus(x, y << i);
        }
        i = binary_minus(i, 1);
    }
    let ans = if (a < 0) ^ (b < 0) { neg(ans) } else { ans };
    ans
}

/**
 * 非位运算版本
 * https://leetcode.cn/problems/n-queens-ii/
*/
pub fn total_n_queens(n: i32) -> i32 {
    if n < 1 {
        return 0;
    };

    let n = n as usize;
    fn can_place(path: &mut Vec<i32>, i: usize, j: usize) -> bool {
        for k in 0..i {
            if j == path[k] as usize || (i as i32 - k as i32).abs() == (j as i32 - path[k]).abs() {
                return false;
            }
        }
        true
    }

    fn f(i: usize, path: &mut Vec<i32>, n: usize) -> i32 {
        if i == n {
            return 1;
        };
        let mut ans: i32 = 0;
        for j in 0..n {
            if can_place(path, i, j) {
                path[i] = j as i32;
                ans += f(i + 1, path, n);
            }
        }
        ans
    }
    let mut path = vec![0; n];
    f(0, &mut path, n)
}
