// https://leetcode.com/problems/add-two-numbers/solutions/3577772/0-ms-rust-solution-beats-100-2-1-mb-memory/
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


impl ListNode {
    fn append(&mut self, val: i32) {
        let mut node = self;
        while let Some(ref mut n) = node.next {
            node = n;
        }
        node.next = Some(Box::new(ListNode::new(val)));
    }
}

impl Iterator for ListNode {
    type Item = Box<ListNode>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            Some(ref mut node) => {
                let res = node.clone();
                self.next = node.next.take();
                Some(res)
            }
            None => None,
        }
    }
}


impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let l1 = l1.unwrap();

        let l2 = l2.unwrap();

        let mut carry = 0;

        let mut f = l1.val + l2.val;

        if f >= 10 {
            f -= 10;
            carry = 1;
        }

        let mut res = ListNode::new(f);

       let mut prev = l1.next;
       let mut prev2 = l2.next;

       while prev.is_some() || prev2.is_some() {
            let mut sum = carry;

            if let Some(node) = prev {
                sum += node.val;
                prev = node.next;
            }
            
            if let Some(node) = prev2 {
                sum += node.val;
                prev2 = node.next;
            }

            carry = sum / 10;

            res.append(sum % 10);
        }

        if carry == 1 {
            res.append(1);
        }

        Some(Box::new(res))
    }
}
