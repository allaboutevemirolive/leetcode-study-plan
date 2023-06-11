// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/solutions/3002663/rust-sort-then-iterate-over-balloons/

use std::collections::BinaryHeap;
use std::iter::FromIterator;

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {

        // Get a heap sorted by ascending x1,
        // then descending x2.
        
        let mut heap = BinaryHeap::from_iter(
            points
                .iter()
                .map(|arr| (-arr[0], arr[1]))
        );

        // Iterate through and count all overlapping
        // balloons with the current balloon.
        // If a balloon overlaps, but is thinner, use it as
        // the current balloon.

        let mut curr = heap.pop().map(|(x1, x2)| (-x1, x2)).unwrap();
        let mut count = 1;

        while let Some((x1, x2)) = heap.pop() {
            let x1 = -x1;
            if (x1 > curr.1 || x2 < curr.0) {
                count += 1;
                curr = (x1, x2);
            } else if (x2 < curr.1) {
                curr.1 = x2;
            }
        }

        return count;

    }
}