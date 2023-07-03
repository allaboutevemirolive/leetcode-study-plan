// https://leetcode.com/problems/remove-nth-node-from-end-of-list/solutions/2500898/rust-solution-explained-line-by-line/
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
    // make the head mutable so we can freely use it
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut node=&mut head;
        // calculate the length of the linked list by passing through it
        let mut len:i32=1;
        while match node.as_mut().unwrap().next {Some(_) => true,None => false} {
            node=&mut node.as_mut().unwrap().next;
            len+=1;
        }
        // return None if list has a length of 1
        if len==1 {
            return None;
        }
        // return the list without the head if asked to remove it
        if len-n==0 {
            return head.unwrap().next;
        }
        // goes to the node before the one we want to remove
        node=&mut head;
        for a in 1..(len-n) {
            node=&mut node.as_mut().unwrap().next;
        }
        // copies it to satisfy the borrow checker
        let mut transition_node=node.clone().unwrap().next;
        // change the next field of the node before with the next field of the node we want to remove, therefore removing it
        node.as_mut().unwrap().next=transition_node.unwrap().next;
        head
    }
}