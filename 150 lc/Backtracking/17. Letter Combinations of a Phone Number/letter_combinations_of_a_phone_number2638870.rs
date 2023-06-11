// https://leetcode.com/problems/letter-combinations-of-a-phone-number/solutions/2638870/rust-dfs-approach-with-mapped-digits/
use std::collections::HashMap;

impl Solution {
    pub fn dfsCombinations(digits: &Vec<u32>, map: &HashMap<u32, Vec<char>>, index: usize, mut output: String, finalCombinations: &mut Vec<String>) {
        if index >= digits.len() {
            if !output.is_empty() {
                finalCombinations.push(output);
            }

            return;
        }

        if let Some(chars) = map.get(&digits[index]) {
            for ch in chars {
                output.push(*ch);

                Self::dfsCombinations(digits, map, index + 1, output.clone(), finalCombinations);

                output.pop();
            }
        }
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut map: HashMap<u32, Vec<char>> = HashMap::new();
        map.insert(0, vec![]);
        map.insert(1, vec![]);
        map.insert(2, vec!['a', 'b', 'c']);
        map.insert(3, vec!['d', 'e', 'f']);
        map.insert(4, vec!['g', 'h', 'i']);
        map.insert(5, vec!['j', 'k', 'l']);
        map.insert(6, vec!['m', 'n', 'o']);
        map.insert(7, vec!['p', 'q', 'r', 's']);
        map.insert(8, vec!['t', 'u', 'v']);
        map.insert(9, vec!['w', 'x', 'y', 'z']);

        let digits: Vec<u32> = digits.chars().map(|ch| ch.to_digit(10).unwrap()).collect();

        let mut finalCombinations: Vec<String> = Vec::new();

        Self::dfsCombinations(&digits, &map, 0, String::new(), &mut finalCombinations);

        finalCombinations
    }
}