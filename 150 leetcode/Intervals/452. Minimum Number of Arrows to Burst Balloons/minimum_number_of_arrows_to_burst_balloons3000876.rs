// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/solutions/3000876/rust-sort-interval-solution/
impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_by_key(|x| x[1]);
        let mut last_end = points[0][1];

        points.iter().fold(1, |mut count, point| {
            if point[0] > last_end {
                count += 1;
                last_end = point[1];
            }
            count
        })
    }
}