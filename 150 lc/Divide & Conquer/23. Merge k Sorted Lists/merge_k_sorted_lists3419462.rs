// https://leetcode.com/problems/merge-k-sorted-lists/solutions/3419462/rust-solution-with-vec/
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

use std::{cmp, mem};

type LNode = Option<Box<ListNode>>;

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<LNode>) -> Option<Box<ListNode>> {
        // First estimation with ~5 elements per list might be too much but need less realloc
        let mut aux = Vec::with_capacity(lists.len() * 5);
        // concat all lists and insert all values into auxiliary Vec
        let (mut lists, mut list) = take_first_list(lists)?;
        let mut cur = &mut list;
        aux.push(cur.val);
        while let Some(ref mut n) = cur.next {
            aux.push(n.val);
            cur = n;
        }
        for l in lists {
            cur.next = l;
            while let Some(ref mut n) = cur.next {
                aux.push(n.val);
                cur = n;
            } 
        }

        // Sort the vector
        aux.sort_unstable();

        // Iterate over list and set the correct values
        let mut cur = &mut list;
        for val in aux {
            cur.val = val;
            // SAFETY: We know that aux.len() is the total length of the list
            // this improves runtime by ~50%
            cur = unsafe { cur.next.as_mut().unwrap_unchecked() };
        }
        Some(list)
    }
}

fn take_first_list(mut lists: Vec<LNode>) -> Option<(Vec<LNode>, Box<ListNode>)> {
    while let Some(l) = lists.pop() {
        if let Some(l) = l {
            return Some((lists, l));
        }
    }
    None
}
