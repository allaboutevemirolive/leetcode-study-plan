// https://leetcode.com/problems/group-anagrams/solutions/3541425/rust/
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.len() <= 1 {
            return vec![strs];
        }

        let mut res: Vec<Vec<String>> = vec![];
        'out: for sstr in strs {
            let mut flag = true;
            for i in 0..res.len() {
                if cmp_strs(sstr.as_str(), &res[i][0]) {
                    res[i].push(sstr);
                    flag = false;
                    continue 'out;
                }
            }

            if flag {
                res.push(vec![sstr]);
            }
        }

        res
    }
}

fn cmp_strs(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let a = a.as_bytes();
    let mut b = b.as_bytes().iter().map(|x| *x).collect::<Vec<_>>();

    if a.iter().map(|x| *x as u32).sum::<u32>() != b.iter().map(|x| *x as u32).sum::<u32>() {
        return false;
    }

    'out: for i in a {
        if !b.contains(i) {
            return false;
        }

        for (idx, j) in b.iter().enumerate() {
            if *i == *j {
                b.remove(idx);
                continue 'out;
            }
        }
    }

    b.is_empty()
}