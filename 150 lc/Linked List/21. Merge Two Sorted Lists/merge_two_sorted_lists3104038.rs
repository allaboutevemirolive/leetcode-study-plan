// https://leetcode.com/problems/merge-two-sorted-lists/solutions/3104038/rust-iterative-merge-from-head/
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

use std::cmp::Ordering;

impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sentinel = Box::new(ListNode {
            val: -1, 
            next: None
        });
        let mut curr = &mut sentinel;
        loop {
            match (list1, list2) {
                (None, None) => break,
                (Some(node), None) | (None, Some(node)) => {
                    curr.next = Some(node);
                    break;
                },
                (Some(mut node1), Some(mut node2)) => {
                    let (mut node1, node2) = match node1.val.cmp(&node2.val) {
                        Ordering::Less => (node1, node2),
                        _ => (node2, node1)                        
                    };
                    list1 = node1.next.take();
                    list2 = Some(node2);
                    curr.next = Some(node1);
                    curr = curr.next.as_mut().unwrap();
                }
            };
        }
        sentinel.next
    }
}