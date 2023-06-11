// https://leetcode.com/problems/reverse-nodes-in-k-group/solutions/3535646/rust-simpliest-safe-solution/
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
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 1 {
            return head;
        }

        let mut group = Vec::with_capacity(k as usize);
        let mut new_head = None;
        let mut new_tail = &mut new_head;

        while let Some(mut node) = head.take() {
            head = node.next.take();
            group.push(node);
            if group.len() == k as usize {
                while let Some(vec_node) = group.pop() {
                    *new_tail = Some(vec_node);
                    new_tail = &mut new_tail.as_mut().unwrap().next;
                }
            }
        }

        for vec_node in group.into_iter() {
            *new_tail = Some(vec_node);
            new_tail = &mut new_tail.as_mut().unwrap().next;
        }

        new_head
    }
}