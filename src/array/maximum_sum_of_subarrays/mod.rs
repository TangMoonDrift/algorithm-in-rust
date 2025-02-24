//! 子数组最大累加和专题

pub fn rob(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    match n {
        1 => nums[0],
        2 => nums[0].max(nums[1]),
        _ => {
            let mut pre_pre = nums[0];
            let mut pre = nums[0].max(nums[1]);

            for i in 2..n {
                let curr = nums[i].max(pre).max(pre_pre + nums[i]);
                pre_pre = pre;
                pre = curr;
            }
            pre
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
        assert_eq!(rob(vec![1, 2, 3, 1, 1, 3]), 6);
    }
}
