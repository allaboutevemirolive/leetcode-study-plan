// https://leetcode.com/problems/add-two-numbers/solutions/2951032/rust-solution-with-a-small-explanation/
impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res = Box::new(ListNode::new(0));
        let mut curr = &mut res;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() {
            let mut l1s = 0;
            let mut l2s = 0;

            match l1 {
                None => {}
                Some(val) => {
                    l1s = val.val;
                    l1 = val.next;
                }
            }

            match l2 {
                None => {}
                Some(val) => {
                    l2s = val.val;
                    l2 = val.next;
                }
            }


            let sum = l1s + l2s + carry;
            carry = sum / 10;

            curr.next = Option::from(Box::new(ListNode::new(sum % 10)));
            curr = curr.next.as_mut().unwrap();
        }

        if carry > 0 {
            curr.next = Option::from(Box::new(ListNode::new(carry)))
        }

        res.next
    }
}