// https://leetcode.com/problems/letter-combinations-of-a-phone-number/solutions/2589562/rust-iterator-solution/
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
    
        use std::collections::HashMap;
        //define phone mapping
        let mut phone_mapping: HashMap<char, Vec<char>> = HashMap::with_capacity(8);
        phone_mapping.insert('2', vec!['a','b','c']);
        phone_mapping.insert('3', vec!['d','e','f']);
        phone_mapping.insert('4', vec!['g','h','i']);
        phone_mapping.insert('5', vec!['j','k','l']);
        phone_mapping.insert('6', vec!['m','n','o']);
        phone_mapping.insert('7', vec!['p','q','r','s']);
        phone_mapping.insert('8', vec!['t','u','v']);
        phone_mapping.insert('9', vec!['w','x','y','z']);
        
        //answer to be returned
        let mut answer: Vec<String> = vec![];
        
        //map the digit character to the phone mapping hash map
        digits.chars()
              .map(|digit|match phone_mapping.clone().get(&digit){
                Some(mapping) => {mapping.clone()},
                None => panic!("Invalid Digit: {}", digit)
              })
            //for each mapping
              .for_each(|char_vec| {
                match answer.is_empty(){
                    //if the answer is empty, map the character array to a string array
                    true => {answer = char_vec.iter().map(|c| c.to_string()).collect(); }
                    /*
                        if the answer is not empty, then for each character in the map
						concatenate what is currently in the answer and collect it into a vector of strings
                        then flatten it and collect the answer.
                        
                        example 23
                        answer = ["a","b","c"]
                        char_vec = ['d','e','f']
                        
                        char_vec.iter().map
                        convert c into an iterator and map for each character
                        d
                        e
                        f 
                        
                        answer.iter().map...collect()
                        [a,b,c] d -> [ad,bd,cd]
                        [a,b,c] e -> [ae,be,ce]
                        [a,b,c] f -> [af,bf,cf]
                        
                        flatten
                        [ad,bd,cd]
                        [ae,be,ce] -> [[ad,bd,cd],[ae,be,ce],[af,bf,cf]]
                        [af,bf,cf]
                        
                        collect
                        [[ad,bd,cd],[ae,be,ce],[af,bf,cf]] -> [ad,bd,cd,ae,be,ce,af,bf,cf]
                    */
                    false => {answer = char_vec.iter().map(|c| {answer.iter().map(move |s| s.clone() + &c.to_string()).collect::<Vec<String>>()}).flatten().collect();}
                }
              });

        
        answer
    }
}