// https://leetcode.com/problems/happy-number/solutions/2767291/rust-solution/
impl Solution {
    fn digits_squares(n: i32) -> i32 {
        let mut numbers: Vec<i32> = vec![];

        for char in n.to_string().chars() {
            numbers.push(char.to_digit(10).unwrap().pow(2) as i32);
        }
        numbers.iter().sum()
    }

    pub fn is_happy(n: i32) -> bool {
        let mut n = n;
        // a decimal unhappy number will repeat this cycle only
        // https://en.wikipedia.org/wiki/Happy_number
        let cycle_members = [4, 16, 37, 58, 89, 145, 42, 20];

        while n != 1 && !cycle_members.contains(&n) {
            n = Self::digits_squares(n);
        }

        if n == 1 {
            return true;
        }

        false
    }
}