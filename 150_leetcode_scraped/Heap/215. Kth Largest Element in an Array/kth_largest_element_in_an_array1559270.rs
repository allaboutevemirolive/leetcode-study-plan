// https://leetcode.com/problems/kth-largest-element-in-an-array/solutions/1559270/rust-0ms-binaryheap-min-heap/
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from(nums);       
        let mut counter = k;
        while counter > 1 {
            heap.pop();
            counter-=1;
        }

        return *heap.peek().unwrap();

    }
}