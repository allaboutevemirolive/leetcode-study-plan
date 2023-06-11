// https://leetcode.com/problems/remove-nth-node-from-end-of-list/solutions/3452018/rust-no-extra-allocation-iterative/
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut size = 0;
        let mut node = head.as_ref();
        while node.is_some() {
            size += 1;
            node = node.unwrap().next.as_ref();
        }

        let mut dummy_node = ListNode::new(0);
        dummy_node.next = head;

        let mut current_node = &mut dummy_node;

        for _iter in 0..size - n {
            current_node = current_node.next.as_mut().map(|node| &mut **node).unwrap();
        }

        current_node.next = current_node.next.take().unwrap().next;
        dummy_node.next
    }
}