// https://leetcode.com/problems/isomorphic-strings/solutions/2764552/check-out-my-0ms-rust-solution-using-128-length-array-o-n/
pub fn is_isomorphic(s: String, t: String) -> bool {
        let s = s.into_bytes();
        let t = t.into_bytes();
        
        let mut sTable: [usize; 128] = [129; 128];
        let mut tTable: [usize; 128] = [129; 128];
        
        let n = s.len();
        
        for i in 0..n{
            let bs = s[i] as usize;
            let bt = t[i] as usize;
            
            if sTable[bs] == 129 {
                if tTable[bt] == 129 {
                    sTable[bs] = bt;
                    tTable[bt] = bs;
                }else{
                    return false;
                }
            }else if sTable[bs] != bt {
                return false;
            }
        }
        
        true
    }