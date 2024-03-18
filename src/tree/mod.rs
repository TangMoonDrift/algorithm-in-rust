use std::cell::RefCell;
use std::collections::VecDeque;
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
 * https://leetcode.cn/problems/binary-tree-level-order-traversal/description/
 * 给你二叉树的根节点 root ，返回其节点值的 层序遍历 。 （即逐层地，从左到右访问所有节点）。
 */
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let mut queue = VecDeque::new();
    if let Some(x) = root {
        queue.push_back(x);
    }
    while !queue.is_empty() {
        let size = queue.len();
        let mut level = Vec::with_capacity(size);

        for _ in 0..size {
            if let Some(node) = queue.pop_front() {
                let mut x = node.borrow_mut();
                level.push(x.val);
                if let Some(left) = x.left.take() {
                    queue.push_back(left);
                }
                if let Some(right) = x.right.take() {
                    queue.push_back(right);
                }
            }
        }
        ans.push(level);
    }
    ans
}

/**
 * https://leetcode.cn/problems/maximum-width-of-binary-tree/description/
 */
pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut ans = 1;

    let mut queue = vec![(0, root)];
    while !queue.is_empty() {
        ans = ans.max(queue.last().unwrap().0 - queue[0].0 + 1);
        let mut tmp = vec![];
        for (i, r) in queue {
            let r = r.as_ref().unwrap().borrow();
            if r.left.is_some() {
                tmp.push((i * 2, r.left.clone()));
            }
            if r.right.is_some() {
                tmp.push((i * 2 + 1, r.right.clone()));
            }
        }
        queue = tmp;
    }
    ans
}
