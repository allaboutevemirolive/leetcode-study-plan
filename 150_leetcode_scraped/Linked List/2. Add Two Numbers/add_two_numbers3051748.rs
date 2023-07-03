// https://leetcode.com/problems/add-two-numbers/solutions/3051748/rust-solution/
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
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
 let mut curr = &mut dummy;
 let mut carry = 0;
 while l1.is_some() || l2.is_some() || carry != 0 {
    let x = match &l1 {
        Some(x1) => x1.val,
        None => 0
    };

    let y = match &l2 {
        Some(y1) => y1.val,
        None => 0
    }; 

    let sum = x + y + carry;
    carry = sum / 10;
    curr.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum % 10)));
    curr = &mut curr.as_mut().unwrap().next;
    if l1.is_some() {
        l1 = l1.unwrap().next;
    }
    if l2.is_some() {
        l2 = l2.unwrap().next;
    }
 }

 dummy.unwrap().next
    }
}