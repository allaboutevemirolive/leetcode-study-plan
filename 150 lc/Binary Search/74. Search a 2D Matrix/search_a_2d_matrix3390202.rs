// https://leetcode.com/problems/search-a-2d-matrix/solutions/3390202/rust-solution-using-binary-search/
impl Solution {
  pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let n = matrix.len();
    let m = matrix[0].len();

    let mut low = 0;
    let mut high = n-1;
    while low != high {
      let mid = (low+high) / 2;
      match (matrix[mid][0]).cmp(&target) {
          std::cmp::Ordering::Less => {
              low = mid + 1;
          }
          std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
              high = mid;
          }
      }
    }

    if low == n { return false }
    if matrix[low][0] == target { return true }
    let row_index = if matrix[low][0] > target {
      if low == 0 { return false }
      low - 1
    } else {
      low
    };    

    let mut low = 0;
    let mut high = m-1;
    while low != high {
      let mid = (low+high) / 2;
      match (matrix[row_index][mid]).cmp(&target) {
          std::cmp::Ordering::Less => {
              low = mid + 1;
          }
          std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
              high = mid;
          }
      }
    }

    if low == m {
      false
    } else {
      matrix[row_index][low] == target
    }
  }
}