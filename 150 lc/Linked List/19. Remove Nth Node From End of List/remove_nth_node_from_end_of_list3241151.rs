// https://leetcode.com/problems/remove-nth-node-from-end-of-list/solutions/3241151/rust-not-optimized-solution/
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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, mut n: i32) -> Option<Box<ListNode>> {
        let mut lenth = 0;
        let mut link = head.clone();
        while let Some(val) = link {
            lenth += 1;
            link = val.next;
        }
        if lenth == 1 {
            return None
        }
        n = lenth - n;
        let mut node = (1..n).fold(head.as_mut().unwrap(), |cur, _| {
            cur.next.as_mut().unwrap()
        });
         if lenth - n == 1 {
            node.next = None;
            return head
        }
        if n == 0 {
            return head.unwrap().next
        }
        node.next = node.next.clone().unwrap().next;
        head
    }
}