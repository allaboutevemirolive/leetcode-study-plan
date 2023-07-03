// https://leetcode.com/problems/merge-two-sorted-lists/solutions/3088194/rust-2-pointers-o-n/
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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = None;
        let mut head = &mut new_head;
        let mut h1 = &list1;
        let mut h2 = &list2;

        while h1.is_some() && h2.is_some() {
            let l1 = h1.as_ref().unwrap();
            let l2 = h2.as_ref().unwrap();
            let v1 = l1.val;
            let v2 = l2.val;
            if v1 < v2 {
                h1 = &l1.next;
                *head = Some(Box::new(ListNode::new(v1)));
            } else {
                h2 = &l2.next;
                *head = Some(Box::new(ListNode::new(v2)));
            }
            head = &mut head.as_mut().unwrap().next;
        }
        if h1.is_some() {
            *head = h1.clone();
        }
        if h2.is_some() {
            *head = h2.clone();
        }
        new_head
    }
}