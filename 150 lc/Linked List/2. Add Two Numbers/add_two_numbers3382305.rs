// https://leetcode.com/problems/add-two-numbers/solutions/3382305/non-recursive-solution-in-rust-using-simple-mutable-references/
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ret = Box::new(ListNode::new(0));
        let mut next_l1 = &l1;
        let mut next_l2 = &l2;
        let mut prev_node: Option<&mut Box<ListNode>> = None;
        let mut carry = 0;
        loop {
          let mut val = 0;
            match (&mut next_l1, &mut next_l2) {
                (Some(n1), Some(n2)) => {
                    val = n1.val + n2.val + carry;
                    next_l1 = &n1.next;
                    next_l2 = &n2.next;
                },
                (Some(n1), None) => {
                    val = n1.val + carry;
                    next_l1 = &n1.next;
                },
                (None, Some(n2)) => {
                    val = n2.val + carry;
                    next_l2 = &n2.next;
                },
                _ => break,
            };
          if val > 9 {
            val -= 10;
            carry = 1;
          } else {
            carry = 0;
          }
          if prev_node != None {
            let mut node = prev_node.unwrap();
            node.next = Some(Box::new(ListNode::new(val)));
            prev_node = node.next.as_mut();
          } else {
            ret.val = val;
            prev_node = Some(&mut ret);
          }
        }
        if carry == 1 {
            prev_node.unwrap().next = Some(Box::new(ListNode::new(1)));
        }
        Some(ret)
    }
}

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
