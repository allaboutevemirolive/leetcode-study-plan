// https://leetcode.com/problems/add-two-numbers/solutions/3545536/rust-0-ms-2-mb-reuse-of-list-2-as-a-return-list/
impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut tail = &mut l2;
        let mut carry = 0;
        let mut end_of_l2 = false;
        loop {
            let mut val = carry;
            if let Some(node) = l1 {
                val += node.val;
                l1 = node.next;
            }   
            if let Some(node) = tail {
                val += node.val;
                node.val = val % 10;
                if node.next.is_none() {
                    node.next = Some(Box::new(ListNode::new(0)));
                    end_of_l2 = true;
                }
                tail = &mut node.next; 
            }
            carry = val / 10;
            if l1.is_none() && end_of_l2 && carry == 0 {
                *tail = None;
                break;
            }
        }
        return l2;
    }
}