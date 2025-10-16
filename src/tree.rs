pub mod trie;

use std::cell::RefCell;
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
 * 102. 二叉树的层序遍历
 * https://leetcode.cn/problems/binary-tree-level-order-traversal/description/
 */
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let mut queue: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![None; 2001];
    let (mut l, mut r) = (0, 0);

    if let Some(x) = root {
        queue[r] = Some(Rc::clone(&x));
        r += 1;
    }

    while l < r {
        let size = r - l;
        let mut level = Vec::with_capacity(size);
        for _ in 0..size {
            if let Some(node) = queue[l].take() {
                let mut x = node.borrow_mut();
                level.push(x.val);
                if let Some(left) = x.left.take() {
                    queue[r] = Some(Rc::clone(&left));
                    r += 1;
                }
                if let Some(right) = x.right.take() {
                    queue[r] = Some(Rc::clone(&right));
                    r += 1;
                }
            }
            l += 1;
        }
        ans.push(level);
    }

    ans
}

/**
 * 662. 二叉树最大宽度
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
 * 111. 二叉树的最小深度
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
        l = min_depth(left);
    }

    if right.is_some() {
        r = min_depth(right);
    }

    l.min(r) + 1
}

/**
 * 236. 二叉树的最近公共祖先
 * https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-tree/description/
 */
pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() || p == root || q == root {
        return root;
    }
    let x = root.as_ref().unwrap();

    let l = lowest_common_ancestor(x.borrow_mut().left.take(), p.clone(), q.clone());
    let r = lowest_common_ancestor(x.borrow_mut().right.take(), p, q);

    if l.is_some() && r.is_some() {
        return root;
    }

    if l.is_some() {
        l
    } else {
        r
    }
}

/**
 * 235. 二叉搜索树的最近公共祖先
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
 * 110. 平衡二叉树
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
 * 98. 验证二叉搜索树
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
