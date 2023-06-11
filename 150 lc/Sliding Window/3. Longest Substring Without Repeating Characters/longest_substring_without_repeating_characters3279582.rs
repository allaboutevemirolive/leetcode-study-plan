// https://leetcode.com/problems/longest-substring-without-repeating-characters/solutions/3279582/rust-hash-map-solution/
use std::collections::HashMap;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len_string:i32 =0 ; 
        let first_pointer :i32 = 0 ;
        let next_pointer :i32 = 0 ; 
        let mut finder = HashMap::new();
        if s.len()==0{
            return 0;
        }
        if s.len() == 1{
            return 1;
        }
       for (first_pointer,letter) in s.chars().enumerate(){
           let new_string = &s[first_pointer..s.len()];
           for (next_pointer,new_letter )in new_string.chars().enumerate(){
           
            match finder.get(&new_letter){
               Some(x)=>{
                   if max_len_string < finder.len() as i32{
                       max_len_string= finder.len() as i32;
                       finder.clear();
                        break;
                   }
                   finder.clear();
                   break;
               }
               None=>{
                   finder.insert(new_letter,1);
                   
               }
           }
           }
       }
       max_len_string
    }
}