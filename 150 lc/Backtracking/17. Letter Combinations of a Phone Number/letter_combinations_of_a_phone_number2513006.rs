// https://leetcode.com/problems/letter-combinations-of-a-phone-number/solutions/2513006/recursive-rust-solution-with-0ms-100-runtime-using-hashset-union/
use std::collections::HashSet;

fn rec_letter_combinations(prefix: &str, next_letters: &str) -> HashSet<String> {
    println!("{} {}", prefix, next_letters);
    
    if next_letters.len() == 0 {
        let mut set = HashSet::new();
        set.insert(prefix.to_string());
        return set;
    }
    
    let owned_prefix = prefix.to_owned();

    let sets = match next_letters.chars().next().unwrap() {
        '2' => (format!("{}{}", prefix, 'a'), format!("{}{}", prefix, 'b'), format!("{}{}", prefix, 'c'), None),
        '3' => (format!("{}{}", prefix, 'd'), format!("{}{}", prefix, 'e'), format!("{}{}", prefix, 'f'), None),
        '4' => (format!("{}{}", prefix, 'g'), format!("{}{}", prefix, 'h'), format!("{}{}", prefix, 'i'), None),
        '5' => (format!("{}{}", prefix, 'j'), format!("{}{}", prefix, 'k'), format!("{}{}", prefix, 'l'), None),
        '6' => (format!("{}{}", prefix, 'm'), format!("{}{}", prefix, 'n'), format!("{}{}", prefix, 'o'), None),
        '7' => (format!("{}{}", prefix, 'p'), format!("{}{}", prefix, 'q'), format!("{}{}", prefix, 'r'), Some(format!("{}{}", prefix, 's'))),
        '8' => (format!("{}{}", prefix, 't'), format!("{}{}", prefix, 'u'), format!("{}{}", prefix, 'v'), None),
        '9' => (format!("{}{}", prefix, 'w'), format!("{}{}", prefix, 'x'), format!("{}{}", prefix, 'y'), Some(format!("{}{}", prefix, 'z'))),
        _ => panic!("unexpected input"),
    };
    
    let recs = (rec_letter_combinations(&sets.0, &next_letters[1..]), rec_letter_combinations(&sets.1, &next_letters[1..]), rec_letter_combinations(&sets.2, &next_letters[1..]), sets.3.map(|set| rec_letter_combinations(&set, &next_letters[1..])));
    
    let res: HashSet<_> = recs.0.union(&recs.1).cloned().collect();
    let res: HashSet<_> = res.union(&recs.2).cloned().collect();
    if let Some(rec3) = recs.3 {
        return res.union(&rec3).cloned().collect();
    }
    
    res
}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        rec_letter_combinations("", &digits).into_iter().filter(|v| v.len() > 0).collect()
    }
}