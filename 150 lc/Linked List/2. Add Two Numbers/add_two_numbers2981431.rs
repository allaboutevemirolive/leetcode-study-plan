// https://leetcode.com/problems/add-two-numbers/solutions/2981431/rust-solution-fast-and-minimum-allocations/
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
        let mut sum = l1;
        let mut cur = &mut sum;
        let mut add = l2;
        let mut acc = 0;

        while let Some(cur_node) = cur {
            let add_val = add.as_ref().map(|node| node.val).unwrap_or(0);
            let val = cur_node.val + add_val + acc;
            acc = val / 10;

            cur_node.val = val % 10;

            add = add.and_then(|node| node.next);
            if cur_node.next.is_none() {
                if add.is_some() {
                    cur_node.next = add;
                    add = None;
                } else if acc != 0 {
                    cur_node.next = Some(Box::new(ListNode::new(0)));
                }
            }
            cur = &mut cur_node.next;
        }
        sum
    }
}