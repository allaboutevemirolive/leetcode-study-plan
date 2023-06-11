// https://leetcode.com/problems/happy-number/solutions/2608905/rust-using-a-hashset-and-iterator-not-fast-but-relatively-easy-to-read/
use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        
        let mut cycle_detection: HashSet<u64> = Default::default();
        let mut number = n as u64;
        
        loop {
            number = 
                format!("{}", number)
                .as_bytes()
                .iter()
                .map(|digit| digit - '0' as u8)
                .map(|digit| (digit * digit) as u64)
                .sum::<u64>();
            
            //println!("{:?} {:?}", cycle_detection, number);
            
            if let Some(_) = cycle_detection.get(&number) {
                return false;
            }
            
            cycle_detection.insert(number);
            
            if number == 1 {
                return true;
            }
        }
        
        unreachable!()
    }
}