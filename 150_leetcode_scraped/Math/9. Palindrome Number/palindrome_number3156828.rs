// https://leetcode.com/problems/palindrome-number/solutions/3156828/vector-solution/
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x<0 {
            false
        }else if x < 10{
            true
        }else{
            let mut _multiplier=1;
            let mut _x = x;
            loop{
                if _x/10==0{
                    break;
                }
                _x/=10;
                _multiplier*=10;
            }
            let mut vec=vec![];
            _x=x;
            loop{
                vec.push((_x/_multiplier)%10);
                _multiplier/=10;
                if _multiplier==0{
                    break;
                }
            }

            let mut index=0;
            loop{
                if vec[index]!=vec[vec.len()-1-index]{
                    return false;
                }
                if index>=vec.len()/2{
                    break;
                }
                index+=1;
            }
            true
        }
        
    }
}