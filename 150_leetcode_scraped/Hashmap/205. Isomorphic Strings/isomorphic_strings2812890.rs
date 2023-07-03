// https://leetcode.com/problems/isomorphic-strings/solutions/2812890/rust-solution-with-100/
use std::collections::HashMap;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
     let mut is_isomorphic = true;
    // CASE 1 Check whteher both string are equal if not return false and exit
    if s.len() != t.len() {
        is_isomorphic = false;
    }

    // making variable  t as iterator to get individual characters
    let mut t = t.chars();

    //declaration of new hash map which is mutuable
    let mut n_map = HashMap::new();

    //declaration of vectors to store all values
    let mut list: Vec<char> = vec![];

    //Starting inserting chars as keys and values in hash map
    for val in s.chars() {
        //Getting the next vale of varaible t and storing
        let val_t = t.next().unwrap();
        //inserting values in map and storing opption<T> in varaiable
        let has_key = n_map.insert(val, val_t);

        //Checking wheter the key was previously existing
        if let Some(x) = has_key {
            if x != val_t {
                is_isomorphic = false;
                break;
            } 
        } else {
            if list.contains(&val_t) {
                is_isomorphic = false;
                break;
            }
            list.push(val_t);
        }
    }
    is_isomorphic
    }
}