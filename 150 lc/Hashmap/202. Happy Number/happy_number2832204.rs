// https://leetcode.com/problems/happy-number/solutions/2832204/fast-and-slow-pointers-solution-in-rust/
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut slow = n;
        let mut fast = calculate_next_number(n);

        while slow != fast {
            slow = calculate_next_number(slow);
            fast = calculate_next_number(calculate_next_number(fast));
        }

        slow == 1
    }
}

fn calculate_next_number(n: i32) -> i32 {
    n.to_string()
        .chars()
        .map(|digit| digit.to_digit(10).unwrap() as i32)
        .map(|digit| digit * digit)
        .sum()
}
