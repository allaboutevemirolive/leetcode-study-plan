// https://leetcode.com/problems/spiral-matrix/solutions/3504909/rust-iterators-with-no-while-loop/
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();
        let k = (m >> 1).min(n >> 1);

        (0..=k)
        .flat_map(|offset| {
            let m_reduced = m - (offset<<1);
            let n_reduced = n - (offset<<1);
            (0..n_reduced).map(move |y| (offset, y+offset))
            .chain(
                (1..m_reduced).map(move |x| (x+offset, n_reduced+offset-1))
            )
            .chain(
                (0..n_reduced-1).rev().map(move |y| (m_reduced+offset-1, y+offset))
            )
            .chain(
                (1..m_reduced-1).rev().map(move |x| (x+offset, offset))
            )
        })
        .take(m*n)
        .map(|(x, y)| matrix[x][y])
        .collect()
    }
}