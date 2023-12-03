pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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
