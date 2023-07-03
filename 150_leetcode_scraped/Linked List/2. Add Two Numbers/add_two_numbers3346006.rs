// https://leetcode.com/problems/add-two-numbers/solutions/3346006/rust-iterative-solution-feel-like-playing-a-challenge-game-with-the-compiler/
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
        let mut dummy = ListNode::new(0);
        let mut p1 = l1;
        let mut p2 = l2;
        
        let mut cur = &mut dummy;
        let mut carry = 0;
        while p1.is_some() || p2.is_some() {
            let mut v1 = 0;
            let mut v2 = 0;
            if p1.is_some() {
                v1 = p1.as_ref().unwrap().val;
                p1 = p1.as_mut().unwrap().next.take();
            }

            if p2.is_some() {
                v2 = p2.as_ref().unwrap().val;
                p2 = p2.as_mut().unwrap().next.take();
            }

            let sum = v1 + v2 + carry;
            let node = ListNode::new(sum % 10);
            carry = sum / 10;
            cur.next = Some(Box::new(node));
            cur = cur.next.as_mut().unwrap();
        }

        if carry > 0 {
            cur.next = Some(Box::new(ListNode::new(carry)));
        }
        return dummy.next;
        
    }
}