// https://leetcode.com/problems/merge-two-sorted-lists/solutions/3318247/rust/

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut curr = &mut head;
        let mut curr_list1 = list1;
        let mut curr_list2 = list2;
        loop {
            match (curr_list1.as_ref(), curr_list2.as_ref()) {
                (Some(l1), Some(l2)) => {
                    if l1.val < l2.val {
                        curr.next = Some(Box::new(ListNode::new(l1.val)));
                        curr_list1 = curr_list1.unwrap().next;
                    } else {
                        curr.next = Some(Box::new(ListNode::new(l2.val)));
                        curr_list2 = curr_list2.unwrap().next;
                    }
                    curr = curr.next.as_mut().unwrap().as_mut();
                },
                (None, _) => {
                    curr.next = curr_list2;
                    break;
                },
                _ => {
                    curr.next = curr_list1;
                    break;
                }
            }
        }
        head.next
    }
}