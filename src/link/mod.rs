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
    let mut tail = &mut head;
    let mut carry = 0;

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
pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut remain = head;
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;
    while remain.is_some() {
        let (new_head, new_remain) = reverse_one(remain, k);
        remain = new_remain;
        tail.next = new_head;
        while tail.next.as_ref().is_some() {
            tail = tail.next.as_mut().unwrap();
        }
    }

    dummy.next
}

// 反转一次，返回反转后的head和remain
// 如果为最后一次不足以反转，remain为None
fn reverse_one(
    head: Option<Box<ListNode>>,
    k: i32,
) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    let mut pre = head.as_ref();
    for _ in 0..k {
        if pre.is_none() {
            return (head, None);
        }
        pre = pre.unwrap().next.as_ref();
    }

    let mut remain = head;
    let mut dummy = ListNode::new(0);
    for _ in 0..k {
        if let Some(mut n) = remain {
            remain = n.next.take();
            n.next = dummy.next.take();
            dummy.next = Some(n);
        }
    }

    (dummy.next, remain)
}
