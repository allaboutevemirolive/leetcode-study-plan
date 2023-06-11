// https://leetcode.com/problems/merge-k-sorted-lists/solutions/3586820/speed-kills-rust-is-your-friend/
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
use std::collections::BTreeMap;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // Going to cheat and make a frequency list of all items in each list.
        // Then we'll just iterate through the frequency list and make a new list.
        let mut frequency_map = BTreeMap::new();
        for list in lists {
            // not sure why this is option, but lets safe unpack
            if let Some(list) = list {
                // for each item in this ListNode, we want to record the frequency of the value
                let mut iter = &list;
                while {
                    let entry = frequency_map.entry(iter.val).or_insert(0);
                    *entry += 1;
                    
                    match &iter.next {
                        Some(node) => {
                            iter = node;
                            true
                        },
                        None => false
                    }
                } { }
            }
        }
        
        // Start construction of return value
        // `head_node` is just a holder so I can keep this loop simple
        let mut head_node = ListNode::new(0);
        let mut temp_node = &mut head_node;
        for (value, frequency) in frequency_map {
            for _ in 0..frequency {
                temp_node.next = Some(Box::new(ListNode::new(value)));
                temp_node = temp_node.next.as_mut().unwrap();
            }
        }
        
        // Head node is just a temp node to hold the reference to the actual result
        head_node.next
    }
}