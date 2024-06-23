pub mod trie;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::mpsc::{self, Sender};

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

/**
 * https://leetcode.cn/problems/minimum-depth-of-binary-tree/description/
 */
pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut left = None;
    let mut right = None;
    if let Some(p) = root {
        left = p.borrow_mut().left.take();
        right = p.borrow_mut().right.take();
    }

    if left.is_none() && right.is_none() {
        return 1;
    }

    let mut l = i32::MAX;
    let mut r = i32::MAX;

    if left.is_some() {
        l = Self::min_depth(left);
    }

    if right.is_some() {
        r = Self::min_depth(right);
    }

    l.min(r) + 1
}

/** https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-tree/description/ */
pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(x) = root.as_ref() {
        if x.borrow().val == p.as_ref().unwrap().borrow().val
            || x.borrow().val == q.as_ref().unwrap().borrow().val
        {
            return Some(Rc::clone(x));
        }
        let p_cloned = Some(p.as_ref().unwrap().clone());
        let q_cloned = Some(q.as_ref().unwrap().clone());

        let left = Self::lowest_common_ancestor(x.borrow_mut().left.take(), p_cloned, q_cloned);
        let right = Self::lowest_common_ancestor(x.borrow_mut().right.take(), p, q);
        if left.is_none() {
            right
        } else if right.is_none() {
            left
        } else {
            Some(Rc::clone(x))
        }
    } else {
        None
    }
}

/**
 * https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-search-tree/description/
 */
pub fn lowest_common_ancestor_in_a_search_tree(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut ans = Some(root.as_ref().unwrap().clone());
    let p_val = p.as_ref().unwrap().borrow().val;
    let q_val = q.as_ref().unwrap().borrow().val;
    let mut root_val = ans.as_ref().unwrap().borrow().val;
    while root_val != p_val && root_val != q_val {
        if root_val < p_val.max(q_val) && root_val > p_val.min(q_val) {
            break;
        }
        ans = if ans.as_ref().unwrap().borrow().val < p_val.min(q_val) {
            ans.as_ref().unwrap().borrow_mut().right.take()
        } else {
            ans.as_ref().unwrap().borrow_mut().left.take()
        };
        root_val = ans.as_ref().unwrap().borrow().val;
    }
    ans
}

/**
 * https://leetcode.cn/problems/balanced-binary-tree/description/
 */
pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn get_heigt(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(x) = node {
            let left_height = get_heigt(&x.borrow_mut().left.take());
            if left_height == -1 {
                return -1;
            }
            let right_height = get_heigt(&x.borrow_mut().right.take());
            if right_height == -1 || (left_height - right_height).abs() > 1 {
                return -1;
            }
            return left_height.max(right_height) + 1;
        }
        0
    }

    get_heigt(&root) != -1
}

/**
 * https://leetcode.cn/problems/validate-binary-search-tree/description/
 */
pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let (tx, rx) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, tx: &Sender<i32>, tx2: &Sender<i32>) {
        if node.is_none() {
            return;
        }
        dfs(node.as_ref().unwrap().borrow().left.clone(), tx, tx2);
        tx.send(node.as_ref().unwrap().borrow().val);
        tx2.send(node.as_ref().unwrap().borrow().val);
        dfs(node.as_ref().unwrap().borrow().right.clone(), tx, tx2);
    }
    dfs(root, &tx, &tx2);
    drop(tx);
    drop(tx2);
    return rx.iter().zip(rx2.iter().skip(1)).all(|(a, b)| a < b);
}
