// https://leetcode.com/problems/maximum-sum-circular-subarray/solutions/3066664/rust-functional-style-solution/
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        Some(nums.iter().fold((0, None, None), |(s, mx, mn): (i32, Option<(i32, i32)>, Option<(i32, i32)>), &el| (
            s+el, 
            mx.map(|(mx, sum)| if sum<0 {(mx.max(el), el)} else {(mx.max(sum+el), sum+el)}).or(Some((el, el))),
            mn.map(|(mn, sum)| if sum>0 {(mn.min(el), el)} else {(mn.min(sum+el), sum+el)}).or(Some((el, el)))
        )))
        .map(|(s, mx, mn)| (s, mx.map(|(mx, _)| mx), mn.map(|(mn, _)| mn)))
        .map(|(s, mx, mn)| mx.zip(mn).map(|(mx, mn)| if s==mn { mx } else { mx.max(s-mn) }).unwrap())
        .unwrap()
    }
}