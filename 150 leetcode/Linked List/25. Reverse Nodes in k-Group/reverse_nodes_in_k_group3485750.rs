// https://leetcode.com/problems/reverse-nodes-in-k-group/solutions/3485750/safe-o-n-o-1-easy-to-implement/
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        fn reverse_and_attach(
            head: Option<Box<ListNode>>,
            tail: Option<Box<ListNode>>
        ) -> Option<Box<ListNode>> {
            let mut head = head;
            let mut tail = tail;
            while let Some(mut node) = head {
                head = node.next.take();
                node.next = tail;
                tail = Some(node);
            }
            tail
        }
        let mut head = head;
        let mut new_head = None;
        loop {
            let mut k_flipped = None;
            let mut has_k = true;
            for _ in 0..k {
                if head.as_ref().is_none() {
                    has_k = false;
                    break;
                }
                let next = head.as_mut().unwrap().next.take();

                let mut node = head.unwrap();
                node.next = k_flipped;
                k_flipped = Some(node);

                head = next;
            }
            if !has_k {
                k_flipped = reverse_and_attach(k_flipped, None);
            }

            new_head = reverse_and_attach(k_flipped, new_head);
            if !has_k {
                break;
            }
        }
        reverse_and_attach(new_head, None)
    }
}