// https://leetcode.com/problems/plus-one/solutions/3437057/rust/
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let length = digits.len();
        let mut arr = vec![0;length + 1];
        let reverted:Vec<i32> = digits.into_iter().rev().collect();
        let mut reminder = 1;
        for i in 0..length {
            if reverted[i] +  reminder >= 10 {
                arr[i] = 0;
                reminder = (reverted[i] +  reminder) % 9;
            }
            else {
                arr[i] = reverted[i] + reminder;
                reminder = 0;

            }
        }
    arr[length] = reminder;
    arr = arr.into_iter().rev().collect();
    if arr[0] == 0 {
        return (&arr[1..length + 1]).to_vec();
    }
    else {
        return (&arr).to_vec();
    }
}
}