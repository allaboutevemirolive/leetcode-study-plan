// https://leetcode.com/problems/simplify-path/solutions/3409273/rust-0ms-beats-100-solution/
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut result: Vec<&str> = vec![];
        path
        .split("/")
        .for_each(|s|{
            if (s == ".."){
                if(!result.is_empty()){
                    result.pop();
                }
            }else if(s != "." && s != ""){
                result.push(s);
            }
        });
        let mut f: String = result.join("/");
        f.insert_str(0, "/");
        f
    }
}