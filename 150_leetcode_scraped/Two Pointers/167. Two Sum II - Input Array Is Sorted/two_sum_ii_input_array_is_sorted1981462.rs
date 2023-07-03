// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/solutions/1981462/rust-two-iterators/
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let size = numbers.len();
    let mut forward = numbers.iter().enumerate().peekable();
    let mut backward = numbers.iter().rev().enumerate().peekable();

    use std::cmp::Ordering;

    loop {
        match (forward.peek(), backward.peek()) {
            (Some((ind, &a)), Some((bac_ind, &b))) => match target.cmp(&(b + a)) {
                Ordering::Equal => return vec![(ind + 1) as i32, (size - bac_ind) as i32],
                Ordering::Less => backward.next(),
                Ordering::Greater => forward.next(),
            },
            _ => unreachable!(),
        };
    }
}