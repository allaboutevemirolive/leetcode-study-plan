// https://leetcode.com/problems/merge-two-sorted-lists/solutions/3138081/rust-recursion/
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
        fn rec(
            mut list1: Option<Box<ListNode>>,
            mut list2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            if list1.is_none() {
                list2
            } else if list2.is_none() {
                list1
            } else {
                let mut l2 = list2.as_mut()?;
                let mut l1 = list1.as_mut()?;
                if l1.val < l2.val {
                    l1.next = rec(l1.next.take(), list2);
                    list1
                } else {
                    l2.next = rec(list1, l2.next.take());
                    list2
                }
            }
        }
        rec(list1, list2)     
    }
}