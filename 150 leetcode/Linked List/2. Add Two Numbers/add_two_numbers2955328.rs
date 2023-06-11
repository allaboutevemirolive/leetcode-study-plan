// https://leetcode.com/problems/add-two-numbers/solutions/2955328/rust-recursive-solution/
pub fn add_numbers_rec(a: Option<Box<ListNode>>, b: Option<Box<ListNode>>, r: Box<ListNode>, c: i32) -> Box<ListNode> {
    let mut r = r;
    let d = match (a, b) {
        (Some(a), Some(b)) => add_numbers_rec(a.next.clone(), b.next.clone(), Box::new(ListNode::new((a.val + b.val + c)%10)), if a.val + b.val + c > 9 { 1 } else { 0 }),
        (Some(a), None) => add_numbers_rec(a.next.clone(), None, Box::new(ListNode::new((a.val + c)%10)), if a.val + c > 9 { 1 } else { 0 }),
        (None, Some(b)) => add_numbers_rec(None, b.next.clone(), Box::new(ListNode::new((b.val + c)%10)), if b.val + c > 9 { 1 } else { 0 }),
        (None, None) => if c == 0 { return r; } else { Box::new(ListNode::new(c)) }
    };
    if r.val == -1 {
        r = d.clone();
    } else {
        r.next.replace(d);
    }
    return r;
}
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Some(add_numbers_rec(l1, l2, Box::new(ListNode::new(-1)), 0))
    }
}