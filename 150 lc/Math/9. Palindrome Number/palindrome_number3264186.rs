// https://leetcode.com/problems/palindrome-number/solutions/3264186/rust-loop-beats-83-for-speed-96-for-memory/
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        /* 
            We stringify the integer, and compare each index
            0 , 1 , 2 , ... , (n-1)-2 , (n-1)-1 , (n-1)-0
            Where n is the length of the String
            x[0] vs. x[(n-1)-0];
            x[1] vs. x[(n-1)-1];
            ...
            We then see the formula:
                x[i] == x[(n-1)-i]
            Where i = (0 , 1 , 2 , 3 , ... , n/2)
            n/2 is max(i) because the centre of the string is the final value
        */

        // BASE CASE: no negative int is a palindrome
        if x < 0 {
            return false;
        }

        let x = x.to_string();
        let n = x.len();
        for i in 0..&n/2 {
            //           v nth(i) deploys the iterator at index i 
            if x.chars().nth(i).unwrap() != x.chars().nth(&n-1-i).unwrap()
            //                  ^ unwrap() error handles nth() return value (of type Option<i32>)        
            {
                return false;
             }
        }
        true    // if the code gets this far, it is a palindrome
    }
}