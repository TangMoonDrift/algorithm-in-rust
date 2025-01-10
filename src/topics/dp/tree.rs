//! 树型DP专题
use std::cell::RefCell;
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
