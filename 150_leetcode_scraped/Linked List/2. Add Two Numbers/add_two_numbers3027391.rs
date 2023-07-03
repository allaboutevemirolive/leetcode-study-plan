// https://leetcode.com/problems/add-two-numbers/solutions/3027391/rust-concise-solution/
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list: ListNode = ListNode::new(0);
        let mut item: &mut ListNode = &mut list;
        let mut list1 = &l1;
        let mut list2 = &l2;
        let mut sum = 0;

        while list1.is_some() || list2.is_some() || sum > 0 {
            if let Some(node) = list1 {
                sum += node.val;
                list1 = &node.next;
            }
            if let Some(node) = list2 {
                sum += node.val;
                list2 = &node.next;
            }
            item.next = Some(Box::new(ListNode::new(sum % 10)));
            item = item.next.as_mut().unwrap();
            sum /= 10;
        }
        list.next
    }
}