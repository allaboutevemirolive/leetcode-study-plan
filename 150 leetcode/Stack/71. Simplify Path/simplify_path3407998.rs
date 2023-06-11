// https://leetcode.com/problems/simplify-path/solutions/3407998/rust-iterator-approach/
impl Solution {
    pub fn simplify_path(path: String) -> String {
        path.split("/").fold(String::from("/"), |mut acc, e| {
            match e {
                // If the substring is empty or ".", no action is needed.
                "" | "." => acc,
                // Remove the last directory.
                ".." => {
                    // Keep removing the last character until '/' is reached.
                    while let Some(c) = acc.pop() {
                        if c == '/' {
                            if acc.is_empty() {
                                acc.push(c);
                            }
                            break;
                        }
                    }
                    acc
                }
                // Handle directory names.
                _ => match acc.as_str() {
                    // Check if the last directory is the root.
                    "/" => acc + e,
                    _ => acc + "/" + e
                }
            }
        })
    }
}