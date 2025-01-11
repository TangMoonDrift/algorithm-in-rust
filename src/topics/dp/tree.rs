//! 树型DP专题
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/**
 * 1373. 二叉搜索子树的最大键值和
 * https://leetcode.cn/problems/maximum-sum-bst-in-binary-tree/description/
 */
pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    struct Info {
        pub max: i32,
        pub min: i32,
        pub sum: i32,
        pub is_bst: bool,
        pub max_bst_sum: i32,
    }

    impl Info {
        fn build(max: i32, min: i32, sum: i32, is_bst: bool, max_bst_sum: i32) -> Self {
            Self {
                max,
                min,
                sum,
                is_bst,
                max_bst_sum,
            }
        }
    }
    fn f(x: Option<Rc<RefCell<TreeNode>>>) -> Info {
        if x.is_none() {
            return Info::build(i32::MIN, i32::MAX, 0, true, 0);
        }

        let binding = x.unwrap();
        let x = binding.borrow();
        let left = x.left.clone();
        let right = x.right.clone();

        let info_left = f(left);
        let info_right = f(right);
        let max = x.val.max(info_left.max).max(info_right.max);
        let min = x.val.min(info_left.min).min(info_right.min);
        let sum = info_left.sum + info_right.sum + x.val;
        let is_bst = info_left.is_bst
            && info_right.is_bst
            && info_left.max < x.val
            && x.val < info_right.min;
        let mut max_bst_sum = info_left.max_bst_sum.max(info_right.max_bst_sum);
        if is_bst {
            max_bst_sum = max_bst_sum.max(sum);
        }

        Info::build(max, min, sum, is_bst, max_bst_sum)
    }

    f(root).max_bst_sum
}

/**
 * 543. 二叉树的直径
 * https://leetcode.cn/problems/diameter-of-binary-tree/description/
 */
pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    pub struct Info {
        h: i32,
        d: i32,
    }

    impl Info {
        fn new(h: i32, d: i32) -> Self {
            Self { h, d }
        }
    }
    fn f(x: Option<Rc<RefCell<TreeNode>>>) -> Info {
        if x.is_none() {
            return Info::new(0, 0);
        }

        let binding = x.unwrap();
        let x = binding.borrow();
        let info_left = f(x.left.clone());
        let info_right = f(x.right.clone());
        let h = info_left.h.max(info_right.h) + 1;
        let mut d = info_left.d.max(info_right.d);
        d = d.max(info_left.h + info_right.h);
        Info::new(h, d)
    }

    f(root).d
}

/**
 * 979. 在二叉树中分配硬币
 * https://leetcode.cn/problems/distribute-coins-in-binary-tree/description/
 */
pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    pub struct Info {
        cnt: i32,
        sum: i32,
        step: i32,
    }

    impl Info {
        pub fn new(cnt: i32, sum: i32, step: i32) -> Self {
            Self { cnt, sum, step }
        }
    }
    fn f(x: Option<Rc<RefCell<TreeNode>>>) -> Info {
        if x.is_none() {
            return Info::new(0, 0, 0);
        }
        let binding = x.unwrap();
        let x = binding.borrow();
        let info_left = f(x.left.clone());
        let info_right = f(x.right.clone());

        let cnts = info_left.cnt + info_right.cnt + 1;
        let sums = info_left.sum + info_right.sum + x.val;
        let steps = info_left.step
            + info_right.step
            + (info_left.sum - info_left.cnt).abs()
            + (info_right.sum - info_right.cnt).abs();

        Info::new(cnts, sums, steps)
    }

    f(root).step
}

/**
 * 337. 打家劫舍 III
 * https://leetcode.cn/problems/house-robber-iii/description/
 */
pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn f(node: Option<Rc<RefCell<TreeNode>>>, yes: i32, no: i32) -> (i32, i32) {
        match node {
            Some(binding) => {
                let node = binding.borrow();
                let (mut y, mut n) = (node.val, 0);
                let (yes, no) = f(node.left.clone(), yes, no);
                y += no;
                n += yes.max(no);
                let (yes, no) = f(node.right.clone(), yes, no);
                y += no;
                n += yes.max(no);
                (y, n)
            }
            None => (0, 0),
        }
    }
    let (yes, no) = f(root, 0, 0);

    yes.max(no)
}

/**
 * 968. 监控二叉树
 * https://leetcode.cn/problems/binary-tree-cameras/description/
 */
pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn f(x: Option<Rc<RefCell<TreeNode>>>, ans: i32) -> (i32, i32) {
        match x {
            Some(binding) => {
                let x = binding.borrow();
                let (ans_left, left) = f(x.left.clone(), ans);
                let (ans_right, right) = f(x.right.clone(), ans);
                let ans = ans_left + ans_right;
                if left == 0 || right == 0 {
                    return (ans + 1, 2);
                }
                if left == 1 && right == 1 {
                    return (ans, 0);
                }
                (ans, 1)
            }
            None => (ans, 1),
        }
    }

    let (ans, status) = f(root, 0);
    if status == 0 {
        return ans + 1;
    }
    return ans;
}

/**
 * 437. 路径总和 III\
 * https://leetcode.cn/problems/path-sum-iii/description/
 */
pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
    let mut pre_sum: HashMap<i64, i32> = HashMap::new();
    pre_sum.insert(0, 1);

    fn f(
        x: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
        mut sum: i64,
        pre_sum: &mut HashMap<i64, i32>,
        mut ans: i32,
    ) -> i32 {
        match x {
            Some(binding) => {
                let x = binding.borrow();
                sum += x.val as i64;
                ans += pre_sum.get(&(sum - target as i64)).unwrap_or(&0);
                let new_sum = pre_sum.get(&sum).unwrap_or(&0);
                pre_sum.insert(sum, new_sum + 1);
                ans = f(x.left.clone(), target, sum, pre_sum, ans);
                ans = f(x.right.clone(), target, sum, pre_sum, ans);
                let old_sum = pre_sum.get(&sum).unwrap() - 1;
                pre_sum.entry(sum).and_modify(|s| *s = old_sum);
                ans
            }
            None => ans,
        }
    }

    f(root, target_sum, 0, &mut pre_sum, 0)
}
