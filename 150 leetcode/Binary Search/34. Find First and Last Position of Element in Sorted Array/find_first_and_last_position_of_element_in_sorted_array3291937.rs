// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/solutions/3291937/rust-binary-search-solution/
use std::cmp::Ordering;
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i: i32 = 0;
        let mut j: i32 = (nums.len() - 1) as i32;
        while i<=j {
            let middle: i32 = i + (j-i)/2;
            match nums[middle as usize].cmp(&target) {
                Ordering::Equal =>{
                    let mut i =middle;
//expand to the left while left == middle
                    while i-1>=0 && nums[i as usize -1]==nums[i as usize] {
                        i-=1;
                    }
                    let mut j =middle;
//expand to the right while right == middle
                    while j+1< nums.len() as i32 && nums[j as usize+1]==nums[j as usize] {
                        j+=1;
                    }
                    return vec![i,j];
                },
//if curr element is bigger than target
                Ordering::Greater => j=middle-1,
//if curr element is smaller than target
                Ordering::Less => i=middle+1,
            }
        }
        vec![-1,-1]
    }
}