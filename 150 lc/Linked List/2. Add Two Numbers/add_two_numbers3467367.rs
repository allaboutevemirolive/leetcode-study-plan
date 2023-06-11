// https://leetcode.com/problems/add-two-numbers/solutions/3467367/a-simple-rust-solution/
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result : Option<Box<ListNode>> = None;
        let mut result_tail : Option<&mut Box<ListNode>> = None;
        let mut carry = 0;
        
        while l1.is_some() || l2.is_some() || carry != 0
        {
            let mut sum = if l1.is_some() { l1.as_ref().unwrap().val }else{ 0 } + 
                          if l2.is_some() { l2.as_ref().unwrap().val }else{ 0 } +
                          carry;
            carry = sum / 10;
            sum = sum % 10;

            if (result_tail.is_none())
            {
                result = Some(Box::new(ListNode{next: None, val: sum}));
                result_tail = Some(result.as_mut().unwrap());
            }
            else
            {
                let tmp = result_tail.unwrap();
                tmp.next = Some(Box::new(ListNode{next: None, val: sum}));
                result_tail = tmp.next.as_mut();
            }

            l1 = if l1.is_some() { l1.unwrap().next }else{ None };
            l2 = if l2.is_some() { l2.unwrap().next }else{ None };
        }

        return result;
    }
}