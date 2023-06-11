// https://leetcode.com/problems/add-two-numbers/solutions/3510205/rust-o-n-solution-with-explanation/
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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        return Solution::add_lists(l1, l2, 0);
    }

    pub fn add_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() {
            if carry > 0 {
                let mut list_node = ListNode::new(carry);
                list_node.next = None;
                return Some(Box::new(list_node));
            }
            return None;
        }
        // Getting the values of l1 and l2 or 0 if any lists are None
        let l1_val = l1.as_ref().unwrap_or(&Box::new(ListNode::new(0))).val;
        let l2_val = l2.as_ref().unwrap_or(&Box::new(ListNode::new(0))).val;
        let sum = l1_val + l2_val + carry;

        // Creating the list node and continuing the sum of the lists
        let mut list_node = ListNode::new(sum % 10);
        let l1_next = l1.unwrap_or(Box::new(ListNode::new(0))).next;
        let l2_next = l2.unwrap_or(Box::new(ListNode::new(0))).next;
        list_node.next = Solution::add_lists(l1_next, l2_next, sum / 10);
        Some(Box::new(list_node))
    }
}
