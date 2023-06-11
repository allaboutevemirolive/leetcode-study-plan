// https://leetcode.com/problems/add-two-numbers/solutions/3288623/rust-recursive-solution-beats-100/
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Solution::add_numbers(l1, l2, false)
    }

    pub fn add_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        of: bool,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => match of {
                true => Some(Box::new(ListNode::new(1))),
                false => None,
            },
            (None, Some(l2)) => {
                let sum = l2.val + i32::from(of);

                Some(Box::new(ListNode {
                    val: sum % 10,
                    next: Solution::add_numbers(None, l2.next, sum / 10 >= 1),
                }))
            }
            (Some(l1), None) => {
                let sum = l1.val + i32::from(of);

                Some(Box::new(ListNode {
                    val: sum % 10,
                    next: Solution::add_numbers(l1.next, None, sum / 10 >= 1),
                }))
            }
            (Some(l1), Some(l2)) => {
                let sum = l1.val + l2.val + i32::from(of);

                Some(Box::new(ListNode {
                    val: sum % 10,
                    next: Solution::add_numbers(l1.next, l2.next, sum / 10 >= 1),
                }))
            }
        }
    }
}