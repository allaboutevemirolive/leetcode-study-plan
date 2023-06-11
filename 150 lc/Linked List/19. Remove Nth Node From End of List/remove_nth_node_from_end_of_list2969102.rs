// https://leetcode.com/problems/remove-nth-node-from-end-of-list/solutions/2969102/rust-list-management/
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let thing = Self::remove_helper(head, n);
        if thing.0 == n {
            return if let Some(n) = thing.1 {n.next} else {None};
        } else {
            return thing.1;
        }
    }

    // recursively travel to the end of the list, then count backwards until we find the node to remove
    fn remove_helper(node: Option<Box<ListNode>>, n: i32) -> (i32, Option<Box<ListNode>>) {
        match node {
            None => { // end node case
                return (0, None);
            }
            Some(mut inner_node) => { // standard node case
                let next_node_indexed = Self::remove_helper(inner_node.next, n);

                if n == next_node_indexed.0 { // remove the next node by setting to the next next
                    inner_node.next = if let Some(next) = next_node_indexed.1 {next.next} else {None};
                } else { // set the next node, because we gave it away in the recursive function
                    inner_node.next = next_node_indexed.1;
                }
                return (next_node_indexed.0 + 1, Some(inner_node));
            }
        };
    }
}