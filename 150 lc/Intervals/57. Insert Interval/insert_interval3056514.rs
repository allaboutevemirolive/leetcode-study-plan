// https://leetcode.com/problems/insert-interval/solutions/3056514/rust-simple-solution-clean-code/
impl Solution {
    pub fn insert(vals: Vec<Vec<i32>>, mut val: Vec<i32>) -> Vec<Vec<i32>> {
        let mut vec = Vec::new();

        let n = vals.len();
        let mut i = 0;

        // While we have not yet reached the insert position of val
        while i < n && vals[i][1] < val[0] {
            vec.push(vals[i].clone());
            i += 1;
        }

        // While we yet have items overlapping with val
        while i < n && (vals[i][0] <= val[1] && val[0] <= vals[i][1]) {
            val[0] = val[0].min(vals[i][0]);
            val[1] = val[1].max(vals[i][1]);
            i += 1;
        }

        vec.push(val);

        // The rest of items in vals that do not overlap with val
        while i < n {
            vec.push(vals[i].clone());
            i += 1;
        }

        vec
    }
}
