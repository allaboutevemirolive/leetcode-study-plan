// https://leetcode.com/problems/median-of-two-sorted-arrays/solutions/3557120/rust-solution-100-time-85-memory/
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total_len: usize = nums1.len() + nums2.len();
        let is_odd_len = total_len % 2 == 1;
        let mut median_idx: usize = total_len / 2;

        let mut median_1: f64 = 0.0;
        let mut median_2: f64 = 0.0;

        let iter = SortedArrayIterator::new(&nums1, &nums2);
        for (i, num) in iter.enumerate() {
            if i == median_idx - 1 {
                median_1 = num as f64;
            }
            if i == median_idx {
                median_2 = num as f64;
                break;
            }
        }

        if is_odd_len {
            median_2
        } else {
            (median_1 + median_2) / 2.0
        }
    }
}

struct SortedArrayIterator<'a> {
    ptr1: usize,
    ptr2: usize,
    nums1: &'a Vec<i32>,
    nums2: &'a Vec<i32>,
}

impl<'a> SortedArrayIterator<'a> {
    fn new(nums1: &'a Vec<i32>, nums2: &'a Vec<i32>) -> SortedArrayIterator<'a> {
        SortedArrayIterator {
            ptr1: 0,
            ptr2: 0,
            nums1,
            nums2,
        }
    }
}

impl<'a> Iterator for SortedArrayIterator<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr1 < self.nums1.len() && self.ptr2 < self.nums2.len() {
            if self.nums1[self.ptr1] < self.nums2[self.ptr2] {
                self.ptr1 += 1;
                Some(self.nums1[self.ptr1 - 1])
            } else {
                self.ptr2 += 1;
                Some(self.nums2[self.ptr2 - 1])
            }
        } else if self.ptr1 < self.nums1.len() {
            self.ptr1 += 1;
            Some(self.nums1[self.ptr1 - 1])
        } else if self.ptr2 < self.nums2.len() {
            self.ptr2 += 1;
            Some(self.nums2[self.ptr2 - 1])
        } else {
            None
        }
    }
}
