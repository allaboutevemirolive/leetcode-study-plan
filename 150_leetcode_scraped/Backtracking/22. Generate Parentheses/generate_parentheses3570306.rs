// https://leetcode.com/problems/generate-parentheses/solutions/3570306/rust-0ms/
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let s = "(".to_string();
        let mut ans: Vec<String> = vec![];
        let stack = 1;
        fn help(s: String, stack: i32, left_cnt: i32, limit: i32, v: &mut Vec<String>) {
            match (s.len() < limit as usize * 2, stack > 0, left_cnt < limit) {
                (true, true, true) => {
                    help(s.clone()+"(", stack+1, left_cnt+1, limit, v);
                    help(s+")", stack-1, left_cnt, limit, v);
                },
                (true, true, false) => help(s+")", stack-1, left_cnt, limit, v),
                (true, false, true) => help(s+"(", 1, left_cnt+1, limit, v),
                (false, _, _) => v.push(s),
                _ => ()
            }
        }    
        help(s, stack, 1, n, &mut ans)     ;
        ans        
    }
}