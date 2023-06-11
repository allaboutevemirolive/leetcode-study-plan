// https://leetcode.com/problems/generate-parentheses/solutions/1812755/rust-non-recursive-solution/
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut vv = vec![vec![String::new()]];

        let n = n as usize;
        while vv.len() < n + 1 {
            let v = vv
                .iter()
                .zip(vv.iter().rev())
                .flat_map(|(v1, v2)| {
                    v1.iter().flat_map(move |s1| {
                        v2.iter()
                            .map(move |s2| format!("({}){}", s1.clone(), s2.clone()))
                    })
                })
                .collect();

            vv.push(v);
        }
        vv.remove(n)
    }
}