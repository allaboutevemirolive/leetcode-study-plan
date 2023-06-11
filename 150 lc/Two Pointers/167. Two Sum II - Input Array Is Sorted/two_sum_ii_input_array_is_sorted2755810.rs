// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/solutions/2755810/faster-than-100-rust-submissions/
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..numbers.len() {
            let second_target = target - numbers[i];
            let mut low = i + 1;
            let mut high = numbers.len();
            while low < high {
                let mid = low + (high - low) / 2;
                if numbers[mid] < second_target {
                    low = mid + 1;
                } else if numbers[mid] > second_target {
                    high = mid;
                } else {
                    return vec![i as i32 + 1, mid as i32 + 1];
                }
            }
        }
        return vec![];
    }