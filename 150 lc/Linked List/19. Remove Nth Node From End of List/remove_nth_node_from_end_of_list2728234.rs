// https://leetcode.com/problems/remove-nth-node-from-end-of-list/solutions/2728234/rust-no-clone-0ms/
impl Solution {
    pub fn remove_nth_from_end(
        mut head: Option<Box<ListNode>>,
        n: i32,
    ) -> Option<Box<ListNode>> {
		// find length
        let mut curr = head.as_ref();
        let mut len = 0;
        while curr.is_some() {
            curr = curr.unwrap().next.as_ref();
            len += 1;
        }
		
		// find removed node
        let mut i = 0;
        let mut curr = &mut head;
        while i < len - n {
            curr = &mut curr.as_mut().unwrap().next;
            i += 1;
        }
		
		// remove node
        let node = curr.take().unwrap();
        *curr = node.next;

        head
    }
}