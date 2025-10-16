//! 数论专题
pub mod gcd;
pub mod lcm;
pub mod power;

pub use gcd::*;
pub use lcm::*;
pub use power::*;

/**
 * how to find the prime factors of a number
 */
pub fn prime_factors(n: i32) -> Vec<i32> {
    let mut factors = vec![];
    let mut n = n;

    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 1;
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

/**
 * 952. 按公因数计算最大组件大小
 * https://leetcode.cn/problems/largest-component-size-by-common-factor/description/
 */
pub fn largest_component_size(nums: Vec<i32>) -> i32 {
    struct UnionFind {
        father: Vec<usize>,
        size: Vec<usize>,
    }

    impl UnionFind {
        pub fn new(n: usize) -> Self {
            Self {
                father: (0..n).collect(),
                size: vec![1; n],
            }
        }

        pub fn find(&mut self, x: usize) -> usize {
            if x != self.father[x] {
                self.father[x] = self.find(self.father[x]);
            }
            self.father[x]
        }

        pub fn union(&mut self, x: usize, y: usize) {
            let fx = self.find(x);
            let fy = self.find(y);
            if fx != fy {
                self.father[fx] = fy;
                self.size[fy] += self.size[fx];
            }
        }
    }

    let n = nums.len();
    let mut union_find = UnionFind::new(n);
    let mut fators_index = vec![-1; 100_001];

    nums.iter().enumerate().for_each(|(index, &num)| {
        let mut i = 2;
        let mut x = num as usize;
        while (i * i) as i32 <= num {
            if x % i == 0 {
                if fators_index[i] == -1 {
                    fators_index[i] = index as i32;
                } else {
                    union_find.union(fators_index[i] as usize, index);
                }
                while x % i == 0 {
                    x /= i;
                }
            }
            i += 1;
        }
        if x > 1 {
            if fators_index[x] == -1 {
                fators_index[x] = index as i32;
            } else {
                union_find.union(fators_index[x] as usize, index);
            }
        }
    });

    *union_find.size.iter().max().unwrap() as i32
}

/**
 * 204. 计数质数
 * https://leetcode.cn/problems/count-primes/description/
 */
pub fn count_primes(n: i32) -> i32 {
    let n = n as usize;
    let mut d = vec![true; n];
    let mut count = 0;
    for i in 2..n {
        if d[i] {
            count += 1;
            let mut j = i * i;
            while j < n {
                d[j] = false;
                j += i;
            }
        }
    }
    count
}
