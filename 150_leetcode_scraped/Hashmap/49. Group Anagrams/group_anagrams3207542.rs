// https://leetcode.com/problems/group-anagrams/solutions/3207542/rust-hashmap-o-n/
use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut v = Vec::with_capacity(1000);
        if strs.len() == 1 {
            v.push(strs);
            return v;
        }
        let mut group: HashMap<[i32; 26], Vec<&str>> = HashMap::with_capacity(5000);
        for i in strs.iter() {
            let ar = Solution::help(i);
            group.entry(ar).and_modify(|x| x.push(i)).or_insert({
                let mut x = Vec::with_capacity(100);
                x.push(i.as_str());
                x
            });
        }
        for (k, val) in group {
            let mut temp = Vec::with_capacity(100);
            for i in val {
                temp.push(i.to_string());
            }
            v.push(temp);
        }
        v
    }
    fn help(s: &str) -> [i32; 26] {
        let mut res = [0; 26];
        for i in s.chars() {
            res[i as usize - b'a' as usize] += 1;
        }
        res
    }
}
