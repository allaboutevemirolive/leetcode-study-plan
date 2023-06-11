// https://leetcode.com/problems/merge-two-sorted-lists/solutions/2901720/solution-using-recursion-in-rust/
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
        match list1 {
            Some(mut list1_head) => {
                match list2 {
                    Some(mut list2_head) => {
                        if list1_head.val < list2_head.val {
                            list1_head.next = Solution::merge_two_lists(list1_head.next, Some(list2_head));
                            return Some(list1_head);
                        } else {
                            list2_head.next = Solution::merge_two_lists(Some(list1_head), list2_head.next);
                            return Some(list2_head)
                        }
                    },
                    None => Some(list1_head)
                }
            },
            None => list2,
        }
    }
}
