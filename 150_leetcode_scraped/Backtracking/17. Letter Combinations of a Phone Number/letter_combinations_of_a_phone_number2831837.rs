// https://leetcode.com/problems/letter-combinations-of-a-phone-number/solutions/2831837/rust-solution-1-ms/
use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let map: HashMap<char, Vec<char>> = HashMap::from([
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z']),
        ]);
        let mut result = vec![];
        for c in digits.chars() {
            let new = map.get(&c).unwrap();
            if result.is_empty() {
                result.extend(new.iter().map(|x| x.to_string()));
                continue;
            }
            let temp = result.clone();
            result.clear();
            for t in temp {
                for n in new {
                    result.push(format!("{t}{n}"));
                }
            }
        }
        result
    }
}