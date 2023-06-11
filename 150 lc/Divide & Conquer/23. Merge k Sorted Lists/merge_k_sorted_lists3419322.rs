// https://leetcode.com/problems/merge-k-sorted-lists/solutions/3419322/rust-zero-allocation-but-not-as-fast/
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
        let mut node = poll_next_smallest(&mut lists)?;
        let mut cur = &mut node;
        while let Some(n) = poll_next_smallest(&mut lists) {
            cur.next = Some(n);
            cur = cur.next.as_mut().unwrap();
        }

        Some(node)
    }
}

fn poll_next_smallest(lists: &mut [LNode]) -> Option<Box<ListNode>> {
    let smallest = lists.iter_mut().filter(|opt| opt.is_some())
        .min_by(|n0, n1| cmp_lnode(n0.as_ref().unwrap(), n1.as_ref().unwrap()));
    match smallest {
        Some(mut small) => {
          let mut next = if let Some(s) = small {
            mem::take(&mut s.next)
          } else {
            None
          };
          mem::swap(&mut next, &mut small);
          next
        }
        _ => None,
    }
}

fn cmp_lnode(node0: &Box<ListNode>, node1: &Box<ListNode>) -> cmp::Ordering {  
  node0.val.cmp(&node1.val)
}
