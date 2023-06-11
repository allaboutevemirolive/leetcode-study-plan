// https://leetcode.com/problems/add-binary/solutions/3185396/rust-0ms-1-9-mb-simple-solution-using-if-let-and-format/
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {

        if let Ok(binouta) = i128::from_str_radix(&a, 2) {
            if let Ok(binoutb) = i128::from_str_radix(&b, 2) {
                return format!("{:b}", binouta + binoutb)
            } else { String::from("B didn't work out") }
        } else { String::from("A didn't work out") }
    }
}