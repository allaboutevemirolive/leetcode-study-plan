// https://leetcode.com/problems/maximal-square/solutions/1547521/rust-solution-based-on-85-maximal-rectangle/
pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let mut grid = vec![vec![0; matrix[0].len()]; matrix.len()];
    for r in 0..matrix.len() {
        for c in 0..matrix[0].len() {
            let mut cell = 0;

            if matrix[r][c] == '1' {
                cell = 1;

                if r > 0 {
                    cell += grid[r - 1][c];
                }
            }

            grid[r][c] = cell;
        }
    }

    // for row in grid.iter() {
    //     println!("{:?}", row);
    // }

    // we can reuse those allocations for every row
    let mut reuse_widths = vec![0; matrix[0].len()];
    let mut stack = vec![];

    let mut max_square = 0;
    for row in grid.iter() {
        let square = largest_square(row, &mut reuse_widths, &mut stack);
        max_square = max_square.max(square);
    }

    max_square
}

fn largest_square(heights: &[i32], widths: &mut [usize], stack: &mut Vec<usize>) -> i32 {
    assert_eq!(heights.len(), widths.len());
    stack.clear();

    // find right boundary - the first smaller element to the right
    for (idx, &h) in heights.iter().enumerate() {
        while let Some(&pos) = stack.last() {
            if h >= heights[pos] {
                break;
            }

            widths[pos] = idx;
            stack.pop();
        }

        stack.push(idx);
    }

    // there are no smaller elements to the right than those
    while let Some(pos) = stack.pop() {
        widths[pos] = heights.len();
    }

    // find left boundary - the first smaller element to the left
    for (idx, &h) in heights.iter().enumerate().rev() {
        while let Some(&pos) = stack.last() {
            if h >= heights[pos] {
                break;
            }

            widths[pos] -= idx + 1;
            stack.pop();
        }

        stack.push(idx);
    }

    // there are no smaller elements to the left than those
    stack.clear();

    // println!();
    // println!("H: {:?}", heights);
    // println!("W: {:?}", widths);

    // we need to find a square, not a rectangle, so we need to
    // find the side of the square, which is the smaller of the
    // width/height
    let mut max_square = 0;
    for idx in 0..heights.len() {
        let side = heights[idx].min(widths[idx] as i32);
        max_square = max_square.max(side * side);
    }
    max_square
}