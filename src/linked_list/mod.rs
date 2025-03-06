#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;
    let mut head = Box::new(ListNode::new(0));
    let tail = &mut head;
    let carry = 0;

    while l1.is_some() || l2.is_some() {
        let mut sum = carry;
        if let Some(node) = l1 {
            sum += node.val;
            l1 = node.next;
        }
        if let Some(node) = l2 {
            sum += node.val;
            l2 = node.next;
        }
    }

    if carry > 0 {
        tail.next = Some(Box::new(ListNode::new(carry)));
    }
    return head.next;
}

/**
 * leetcode：https://leetcode-cn.com/problems/reverse-nodes-in-k-group/
 * 给你链表的头节点 head ，每 k 个节点一组进行翻转，请你返回修改后的链表。
 * k 是一个正整数，它的值小于或等于链表的长度。如果节点总数不是 k 的整数倍，那么请将最后剩余的节点保持原有顺序。
 * 注意：你不能只是单纯的改变节点内部的值，而是需要实际进行节点交换。
 */
pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut next_head = &mut head;
    // 获取下一轮头结点
    for _ in 0..k {
        if let Some(node) = next_head.as_mut() {
            next_head = &mut node.next;
        } else {
            return head;
        }
    }
    // 获取除本轮结果
    let mut new_head = reverse_k_group(next_head.take(), k);
    // 翻转本轮k个节点
    for _ in 0..k {
        if let Some(mut node) = head {
            head = node.next.take();
            node.next = new_head.take();
            new_head = Some(node);
        }
    }
    new_head
}

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    if head.is_none() || head.as_ref().unwrap().next.is_none() {
        return true;
    }
    fn is_palindrome_help(
        origin_head: Option<Box<ListNode>>,
        new_head: Option<Box<ListNode>>,
    ) -> bool {
        match (origin_head, new_head) {
            (None, None) | (None, Some(_)) | (Some(_), None) => true,
            (Some(origin_head), Some(new_head)) => {
                if origin_head.val == new_head.val {
                    is_palindrome_help(origin_head.next, new_head.next)
                } else {
                    false
                }
            }
        }
    }

    fn revert_link(
        pre: Option<Box<ListNode>>,
        mut cur: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if cur.is_none() {
            return pre;
        }
        use std::borrow::BorrowMut;
        let next = std::mem::replace(cur.as_mut().unwrap().next.borrow_mut(), pre);
        revert_link(cur, next)
    }

    let (mut slow, mut fast) = (&head, &head);

    while fast.is_some() {
        fast = fast
            .as_ref()
            .unwrap()
            .next
            .as_ref()
            .map(|x| &x.next)
            .unwrap_or(&None);
        slow = slow.as_ref().map(|x| &x.next).unwrap();
    }
    let s = slow as *const Option<Box<ListNode>> as *mut Option<Box<ListNode>>;
    let slow = unsafe { (*s).take() };

    let new_head = revert_link(None, slow);

    is_palindrome_help(head, new_head)
}
