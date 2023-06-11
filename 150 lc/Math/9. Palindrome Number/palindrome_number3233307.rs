// https://leetcode.com/problems/palindrome-number/solutions/3233307/palindromes-solved-in-rust/
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
    let str_num = x.to_string();
    let rev_str: String = str_num.chars().rev().collect();
    if str_num == rev_str{
    println!("palindrome found!");
    true
    }
    else {
        println!("This is not a palindrome!");
        false
    }
    
    
    }
}