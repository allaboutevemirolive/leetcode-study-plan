// https://leetcode.com/problems/letter-combinations-of-a-phone-number/solutions/2588969/straightforward-0m-rust-solution-via-recursion-inlined/
impl Solution {

    #[inline(always)]
    fn letter_recursion(digits: &[u8], cur_comb: &mut Vec<char>, res: &mut Vec<String>) {
        // creating array of letters per digit. It's static to reuse among function calls
        static letters: [&[char];8] = [
            &['a','b','c'],
            &['d','e','f'],
            &['g','h','i'],
            &['j','k','l'],
            &['m','n','o'],
            &['p','q','r','s'],
            &['t','u','v'],
            &['w','x','y','z'],
        ];
        if digits.len() == 0 { 
            res.push(cur_comb.iter().collect());
            return;
        };
        // digit char subtracted '2' as u8 gives us index in the letter array, "2" is 0, "3" is 1, etc
        for i in letters[ (digits[0]-'2' as u8) as usize] {
            cur_comb.push(*i);
            Solution::letter_recursion(&digits[1..], cur_comb.as_mut(), res);
            cur_comb.pop();
        }
    }
    #[inline(always)]
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digit_bytes = digits.as_bytes();
        let mut res: Vec<String> = Vec::with_capacity(digits.len() * 4);
        if digits.len() == 0 { return res; }
        let mut cur_combination = Vec::with_capacity(digits.len());
        Solution::letter_recursion(digit_bytes, &mut cur_combination, &mut res);

        res
    }
}