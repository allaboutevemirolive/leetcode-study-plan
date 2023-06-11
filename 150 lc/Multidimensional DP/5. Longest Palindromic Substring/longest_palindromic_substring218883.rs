// https://leetcode.com/problems/longest-palindromic-substring/solutions/218883/rust-iterators-based-solution-20ms/
pub fn search_palindorome<'a>(left: &'a[u8], right: &'a[u8]) -> [Vec<u8>; 2] {
    (0..::std::cmp::min(left.len(), right.len()))
        .take_while(|index| left.get(left.len() - index - 1).unwrap() == right.get(*index).unwrap())
        .last()
        .map_or([vec![], vec![]], |index| [left[left.len() - index - 1..].to_vec(), right[..index + 1].to_vec()])
}

pub fn longest_palindrome(s: String) -> String {
    let bytes = s.as_bytes();
    bytes
        .iter()
        .enumerate()
        .map(|(index, byte)|
            vec![
                String::from_utf8(search_palindorome(&bytes[..index], &bytes[index..])
                        .iter()
                        .flatten()
                        .cloned()
                        .collect())
                    .unwrap(),
                {
                    let [left, right] = search_palindorome(&bytes[..index], &bytes[index + 1..]);
                    String::from_utf8(left
                            .iter()
                            .chain(::std::iter::once(byte))
                            .chain(right.iter())
                            .cloned()
                            .collect())
                        .unwrap()
                }
            ])
        .flatten()
        .max_by_key(|palindrome| palindrome.len())
        .unwrap_or("".into())
}