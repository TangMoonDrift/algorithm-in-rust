fn is_odd(num: i32) -> bool {
    num & 1 == 1
}

fn is_even(num: i32) -> bool {
    num & 1 == 0
}

/**
 * 零一翻转
 * 1变0，0变1
 */
fn flip(num: i32) -> i32 {
    (num ^ 1) & 1
}

fn sign(num: i32) -> i32 {
    flip((num >> 31) & 1)
}

pub fn get_greater(x: i32, y: i32) -> i32 {
    let z = x - y;
    let return_x = sign(z);
    let return_y = flip(return_x);
    return_x * x + return_y * y
}

pub fn get_lesser(x: i32, y: i32) -> i32 {
    let z = x - y;
    let return_x = flip(sign(z));
    let return_y = flip(return_x);
    return_x * x + return_y * y
}

pub fn get_max(x: i32, y: i32) -> i32 {
    let z = x.saturating_sub(y);
    let sign_x = sign(x);
    let sign_y = sign(y);
    let sign_z = sign(z);

    let diff_x_y = sign_x ^ sign_y;
    let same_x_y = flip(diff_x_y);

    let return_x = diff_x_y * sign_x + same_x_y * sign_z;
    let return_y = flip(return_x);
    return_x * x + return_y * y
}

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
 * 268. 丢失的数字
 * https://leetcode.cn/problems/missing-number/description/
 */
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut xor_all = 0;
    let mut xor_has = 0;
    for (i, num) in nums.into_iter().enumerate() {
        xor_all ^= i;
        xor_has ^= num;
    }
    xor_all ^= n;
    xor_all as i32 ^ xor_has
}

/**
 * 136. 只出现一次的数字
 * https://leetcode.cn/problems/single-number/description/
 */
pub fn single_number_i(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |acc, num| acc ^ num)
}

/**
 * 137. 只出现一次的数字 II
 * https://leetcode.cn/problems/single-number-ii/
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

fn brian_kernighan(n: i32) -> i32 {
    n & -n
}

/**
 * 260. 只出现一次的数字 III
 * https://leetcode.cn/problems/single-number-iii/description/
 */
pub fn two_single_number_iii(nums: Vec<i32>) -> Vec<i32> {
    let eor_1 = nums.iter().fold(0, |r, v| r ^ v);
    let right_one = brian_kernighan(eor_1);
    let mut eor_2 = 0;
    for num in &nums {
        if (right_one & num) == 0 {
            eor_2 ^= num;
        }
    }
    vec![eor_1 ^ eor_2, eor_2]
}

/**
 * 231. 2 的幂
 * https://leetcode.cn/problems/power-of-two/
 */
pub fn is_power_of_two(n: i32) -> bool {
    n > 0 && n == (n & -n)
}

/**
 * 326. 3 的幂
 * https://leetcode.cn/problems/power-of-three/description/
*/
pub fn is_power_of_three(n: i32) -> bool {
    n > 0 && 1162261467 % n == 0
}

/**
 * 比n大，最接近n的2次幂。
 */
pub fn near_2_power(n: i32) -> i32 {
    let mut n = n;
    if n < 0 {
        return 1;
    }

    n -= 1;
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n + 1
}

/**
 * 201. 数字范围按位与
 * https://leetcode.cn/problems/bitwise-and-of-numbers-range/description/
 */
pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
    let mut ans = right;
    while left < ans {
        ans -= ans & -ans;
    }
    ans
}

/**
 * 190. 颠倒二进制位
 * https://leetcode.cn/problems/reverse-bits/
 */
pub fn reverse_bits(x: u32) -> u32 {
    let mut n = x;
    n = ((n & 0xaaaaaaaa) >> 1) | ((n & 0x55555555) << 1);
    n = ((n & 0xcccccccc) >> 2) | ((n & 0x33333333) << 2);
    n = ((n & 0xf0f0f0f0) >> 4) | ((n & 0x0f0f0f0f) << 4);
    n = ((n & 0xff00ff00) >> 8) | ((n & 0x00ff00ff) << 8);
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
 * 52. N 皇后 II
 * 非位运算版本
 * https://leetcode.cn/problems/n-queens-ii/
*/
// pub fn total_n_queens(n: i32) -> i32 {
//     if n < 1 {
//         return 0;
//     };

//     let n = n as usize;
//     fn can_place(path: &mut Vec<i32>, i: usize, j: usize) -> bool {
//         for k in 0..i {
//             if j == path[k] as usize || (i as i32 - k as i32).abs() == (j as i32 - path[k]).abs() {
//                 return false;
//             }
//         }
//         true
//     }

//     fn f(i: usize, path: &mut Vec<i32>, n: usize) -> i32 {
//         if i == n {
//             return 1;
//         };
//         let mut ans: i32 = 0;
//         for j in 0..n {
//             if can_place(path, i, j) {
//                 path[i] = j as i32;
//                 ans += f(i + 1, path, n);
//             }
//         }
//         ans
//     }
//     let mut path = vec![0; n];
//     f(0, &mut path, n)
// }

/**
 * 52. N 皇后 II
 * https://leetcode.cn/problems/n-queens-ii/
*/
pub fn total_n_queens(n: i32) -> i32 {
    if n < 1 {
        return 0;
    };
    let limit = (1 << n) - 1;
    fn f(limit: i32, col: i32, left_bottom: i32, right_bottom: i32) -> i32 {
        if col == limit {
            return 1;
        };
        let mut ans = 0;

        let ban = col | left_bottom | right_bottom;
        let mut candidate = limit & !ban;
        while candidate != 0 {
            let place = candidate & (!candidate + 1);
            candidate ^= place;
            ans += f(
                limit,
                col | place,
                (left_bottom | place) >> 1,
                (right_bottom | place) << 1,
            );
        }
        ans
    }
    f(limit, 0, 0, 0)
}
