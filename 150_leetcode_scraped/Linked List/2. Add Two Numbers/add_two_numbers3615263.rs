// https://leetcode.com/problems/add-two-numbers/solutions/3615263/non-recursive-solution-3ms-2-1mb/
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
/// Function to add two vectored numbers together
pub fn add_vecs(mut left: Vec<i32>, mut right: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();

    left.reverse(); // put them in push order
    right.reverse(); 

    if left.len() > right.len() { // swap them to always be in least -> greatest order
        let temp = left;
        left = right;
        right = temp;
    }

    let mut carry = 0; // will hold the carry for every addition
    for i in 0..right.len() { // iterate through each element and add the numbers and propogate the carry 

        let mut this = 0; // holds the current value
        if i < left.len() { // We account for different length arrays with this one
            this = left[i] + right[i] + carry; // add and add carry

            if this >= 10 { // if the result is double digits, add to the carry
                carry = this / 10;
                this = this - (carry * 10);
            } else { // otherwise the carry is 0
                carry = 0;
            }

        } else { // Here we will just transfer over all of the remaining values
            this = right[i] + carry;

            if this >= 10 { // still have to account for a carry from the last iteration
                carry = this / 10;
                this = this - (carry * 10);
            } else {
                carry = 0;
            }
        }

        res.push(this);
    }

    if carry > 0 { // This is for those edge cases where there would typically be overflow
    res.push(carry);
    }

    println!("{:?}", res.clone());

    res
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
        /// Will be used to convert the box inputs into vectors
        let box_to_vec = |mut temp: Option<Box<ListNode>>| {
            let mut vec: Vec<i32> = Vec::new();
            while temp != None {
                let num = temp.clone().unwrap().val;
                vec.insert(0, num);
                temp = temp.unwrap().next;
            };

            vec
        };

        let num1 = box_to_vec(l1.clone()); // Convert boxes to vecs
        let num2 = box_to_vec(l2.clone());

        let mut num = Solution::add_vecs(num1.clone(), num2.clone()); // run them through the addition function

        let mut ind = 0;
        let mut vec: Vec<Option<Box<ListNode>>> = Vec::new();
        num.reverse();

        // Here's where we rebuild our linked list
        for x in num {
            if ind == 0 {
                vec.push(Some(Box::new(ListNode {
                    val: x as i32,
                    next: None,
                })));
            } else {
                vec.push(Some(Box::new(ListNode {
                    val: x as i32,
                    next: vec[ind-1].clone(),
                })));
            }
            ind += 1;
        }

        // The LeetCode version of the rust compiler does NOT like unwrapping the result of an option, so I had to match the result
        match vec.pop() {
            Some(n) => n,
            None => Some(Box::new(ListNode::new(0)))
        }
    }
}







