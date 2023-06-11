// https://leetcode.com/problems/remove-nth-node-from-end-of-list/solutions/2633583/rust-concise-two-pass-with-iterators-with-comments/
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let cnt = std::iter::successors(head.as_ref(), |last| last.next.as_ref()).count();
        let mut dummy = Some(Box::new(ListNode{val: 0, next: head}));
        let mut prev = (0..cnt - n as usize).fold(dummy.as_mut(), |curr, _| curr.unwrap().next.as_mut() );
        prev.unwrap().next = prev.as_mut().unwrap().next.as_mut().unwrap().next.take();
        dummy.unwrap().next
    }
}