// https://leetcode.com/problems/remove-nth-node-from-end-of-list/solutions/2634553/rust-idiomatic-pointers-no-allocation-no-clone-100-runtime/
// // Definition for singly-linked list.
// #![allow(dead_code)]
// struct Solution;
//
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

type Obn = Option<Box<ListNode>>;
impl Solution {
    pub fn remove_nth_from_end(mut head: Obn, n: i32) -> Option<Box<ListNode>> {
        let mut a = &head as *const Obn;
        for _ in 0..n {
            a = &unsafe { &*a }.as_ref()?.next as *const _;
        }

        if unsafe { &*a }.is_none() {
            return head.unwrap().next;
        }

        let mut b = &mut head as *mut Obn;
        while unsafe { &*a }.is_some() && unsafe { &*a }.as_ref().unwrap().next.is_some() {
            a = &unsafe { &*a }.as_ref()?.next;
            b = &mut unsafe { &mut *b }.as_mut()?.next;
        }

        unsafe { &mut *b }.as_mut()?.next = unsafe { &mut *b }.as_mut()?.next.take()?.next;
        head
    }
}