// https://leetcode.com/problems/longest-substring-without-repeating-characters/solutions/3008742/solution-in-rust/
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut str_vec : Vec<char> = Vec::new();
        let parsed_string : Vec<char> = s.chars().collect();
        let mut max : i32 = 0;
        let mut count = 0;
        for i in parsed_string.iter() {
            while str_vec.contains(i) {
                str_vec.remove(0);
            }
            str_vec.push(*i);
            count = str_vec.len() as i32;
            if count > max {
                max = count;
            }
        }
        if count > max {
            max = count;
        }
        max
    }
}