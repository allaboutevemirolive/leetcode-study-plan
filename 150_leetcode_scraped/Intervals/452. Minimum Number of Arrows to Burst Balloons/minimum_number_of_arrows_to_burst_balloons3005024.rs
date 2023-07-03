// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/solutions/3005024/simple-rust-solution/
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n == 0 {
            return 0;
        }
        points.sort_by_key(|p| p[1]);
        let mut end = points[0][1];
        let mut res = 1;
        for i in 1..n {
            if points[i][0] <= end {
                continue;
            }
            end = points[i][1];
            res += 1;
        }
        res 
    }
}