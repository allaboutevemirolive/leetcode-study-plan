// https://leetcode.com/problems/two-sum/solutions/3498124/80-time-85-memory-rust-python-js-learn-rust-with-me/
if h.contains(v){ 
return vec![ nums.iter().position(|&a| a == (&target-v)).unwrap() as i32 ,i as i32];
 }
 h.insert(target-v);} 