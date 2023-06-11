// https://leetcode.com/problems/rotate-list/solutions/3552484/rust-raw-pointers/
impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut nodes: Vec<*mut ListNode> = vec![];
        
        let mut h = head.as_mut();
        while let Some(x) = h {
            nodes.push(&mut **x);
            h = x.next.as_mut();
        }

        let len = nodes.len();
        let k = k as usize % len;
        if k == 0 {
            return head;
        }

        unsafe{
            (**nodes.last_mut().unwrap_unchecked()).next = head;
            nodes[len - k - 1].as_mut().unwrap_unchecked().next.take()
        }
    }
}