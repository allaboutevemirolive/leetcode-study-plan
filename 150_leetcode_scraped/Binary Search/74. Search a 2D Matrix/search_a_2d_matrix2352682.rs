// https://leetcode.com/problems/search-a-2d-matrix/solutions/2352682/rust-solution-in-o-n-time-constant-memory/
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        // What row is greater than number
        let mut row = 0;
        // What col is greater than number
        let mut col = 0;
        // Place where there are more rows than columns or vice-versa
        let end_diagonal = std::cmp::min(matrix.len(), matrix[0].len());

        // Search diagonal for when target is smaller than diagonal number
        for i in 0..end_diagonal {
            if matrix[i][i] == target {
                return true;
            }
            if matrix[i][i] > target {
                row = i;
                col = i;
                break;
            }
        }

        // If diagonal is never larger, check whatever is longer, #rows or #columns
        if row == 0 {
            // If row == col == 0, need to move "diagonal" right or down instead of down the
            // diagonal
            // See if we can move down (row wise)
            row = end_diagonal - 1;
            col = end_diagonal - 1;
            if matrix.len() > end_diagonal {
                for i in end_diagonal..matrix.len() {
                    if matrix[i][end_diagonal - 1] == target {
                        return true;
                    }
                    if matrix[i][end_diagonal - 1] > target {
                        row = i;
                        break;
                    }
                }
            }
            // See if we can move right (col wise)
            else if matrix[0].len() > end_diagonal {
                for i in end_diagonal..matrix[0].len() {
                    if matrix[end_diagonal - 1][i] == target {
                        return true;
                    }
                    if matrix[end_diagonal - 1][i] > target {
                        col = i;
                        break;
                    }
                }
            }
        }

        // -- Check very number between larger and previous diagonal -- //
        // Check previous row first from diagonal on
        if row > 0 {
            for i in col..matrix[0].len() {
                if matrix[row - 1][i] == target {
                    return true;
                }
            }
        }
        // Check current row
        for i in 0..col {
            if matrix[row][i] == target {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let correct = [1, 3, 5, 7, 10, 11, 16, 20, 23, 30, 34, 60];

        for c in correct {
            println!("{}", c);
            assert!(Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                c
            ));
        }
    }

    #[test]
    fn test2() {
        assert!(!Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            13
        ));
    }

    #[test]
    fn test3() {
        let correct = [-10, -10, -9, -9, -8, -6, -4, -2, 0, 1, 3, 3, 5, 5, 6, 8];
        for c in correct {
            println!("{}", c);
            assert!(Solution::search_matrix(
                vec![
                    vec![-10, -10],
                    vec![-9, -9],
                    vec![-8, -6],
                    vec![-4, -2],
                    vec![0, 1],
                    vec![3, 3],
                    vec![5, 5],
                    vec![6, 8]
                ],
                c
            ));
        }
    }

    #[test]
    fn test4() {
        assert!(!Solution::search_matrix(vec![vec![1]], 0));
    }
}