// https://leetcode.com/problems/average-of-levels-in-binary-tree/solutions/3554230/rust/
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut res = vec![];
    let mut ans:Vec<f64> =  vec![];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0,root.clone()));
        while queue.len() > 0 {
                let (level,node) = queue.pop_front().unwrap();
                res.push((level ,node.clone().unwrap().borrow().val));
                if let Some(l) = node.clone().unwrap().borrow().left.clone() {
                    queue.push_back((level + 1 ,Some(l.clone())));
                }
                if let Some(r) = node.clone().unwrap().borrow().right.clone(){
                    queue.push_back((level + 1 ,Some(r.clone())));
                }
        }
        let mut hm:std::collections::HashMap<i32,Vec<i64>> = std::collections::HashMap::new();
        for r in res.into_iter(){
            hm.entry(r.0).or_insert(Vec::new()).push(r.1 as i64);
        }
        let mut hm_vec:Vec<(i32,Vec<i64>)> = hm.into_iter().collect();
        hm_vec.sort_by(|a,b| a.0.cmp(&b.0));
        for (_,val) in hm_vec.into_iter(){
            let sum:i64 = val.clone().into_iter().sum();
            let count = val.clone().into_iter().count() as i64;
            println!("{:?} {:?}",sum,count);
            ans.push( (sum as f64/ count as f64 ).to_string().parse::<f64>().unwrap());
        }
        ans
    }
}