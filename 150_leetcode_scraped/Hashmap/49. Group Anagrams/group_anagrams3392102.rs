// https://leetcode.com/problems/group-anagrams/solutions/3392102/rust-0ms-easy-understanding/
impl Solution {
  pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    let mut map: HashMap<[u8;26], Vec<String>> = HashMap::with_capacity(strs.len());
    let offset = 'a' as usize;

    for str in strs.into_iter() {
      let mut chars: [u8; 26] = [0; 26];

      for char in str.chars() {
        chars[char.to_ascii_lowercase() as usize - offset] += 1;
      }

      map.entry(chars)
        .and_modify(|v| v.push(str.clone()))
        .or_insert(vec![str]);
    }
    
    let mut arr: Vec<Vec<String>> = Vec::new();
    for v in map.into_values() {
      arr.push(v);
    }
    arr
  }
}