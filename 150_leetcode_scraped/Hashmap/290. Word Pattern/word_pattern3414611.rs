// https://leetcode.com/problems/word-pattern/solutions/3414611/rust-lazy-flow-with-chain/
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        use std::collections::HashMap;
        let mut m1 = HashMap::new();
        let mut m2 = HashMap::new();
        s
        .split(" ")
        .chain(
            std::iter::repeat("")
        )
        .zip(pattern.chars().into_iter().chain(
            std::iter::repeat('\0')
        ))
        .take_while(|(word, ch)|
            word.len() != 0 || *ch != '\0'
        )
        .map(|(word, ch)|{
            if ch == '\0' || word.len() == 0 {
                return false;
            }
            let w = String::from(word);
            let e1 = m1.entry(w.clone()).or_insert(ch.clone());
            let e2 = m2.entry(ch.clone()).or_insert(w.clone());
            // println!("{} {:?} {} {}", ch, e1, w, *e1==ch);
            *e1 == ch && *e2 == w
        })
        .any(|x| !x)
        .eq(&false)
    }
}