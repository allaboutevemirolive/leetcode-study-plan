// https://leetcode.com/problems/summary-ranges/solutions/1805370/rust-solution/
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut res: Vec<String> = vec![];

        if nums.is_empty() {
            return res;
        }

        let mut head = nums[0];

        for i in 1..nums.len() {
            let curr = nums[i];
            let prev = nums[i - 1];

            if curr != prev + 1 {
                res.push(Self::convert(head, prev));
                head = curr;
            }
        }

        res.push(Self::convert(head, *nums.last().unwrap()));

        res
    }

    fn convert(start: i32, end: i32) -> String {
        if start == end {
            format!("{}", &start)
        } else {
            format!("{}->{}", &start, &end)
        }
    }
}