// https://leetcode.com/problems/letter-combinations-of-a-phone-number/solutions/2264301/rust-0ms-2-1mb/
pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.len() == 0 {
        return vec![]; 
    }
    let mut digital2str = vec![(0, 0); 10];
    init_vec(&mut digital2str);
    let mut ans: Vec<String> = vec![];
    let mut s = "".to_string();
    iter_letter(&digital2str, &digits, &mut s, &mut ans, 0);
    ans
}

fn iter_letter(d2s: &Vec<(u8, u8)>, digits: &str, s: &mut String, ans: &mut Vec<String>, ind: usize) {
    let i: usize = digits[ind..ind+1].parse().unwrap();
    let (begin, end) = (d2s[i].0, d2s[i].1);
    if ind == digits.len() - 1 {
        for c in 0..end {
            let c = (begin + c) as char;
            ans.push(format!("{}{}", s, c));
        }
        return ;
    }
    for c in 0..end {
        let c = (begin + c) as char;
        s.push(c);
        iter_letter(d2s, digits, s, ans, ind+1);
        s.pop();
    }    
}
fn init_vec(digital2str: &mut Vec<(u8, u8)>) {
    let mut a = 97u8;
    for i in 2..7 {
        digital2str[i] = (a, 3u8);
        a += 3;
    } 
    digital2str[7] = (112, 4);
    digital2str[8] = (116, 3);
    digital2str[9] = (119, 4);

}