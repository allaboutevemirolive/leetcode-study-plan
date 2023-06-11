// https://leetcode.com/problems/plus-one/solutions/3182638/rust-solution-beats-100-using-a-loop/
impl Solution {
   pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut last:usize = digits.len()-1;
        let mut answer = Vec::from(digits);
        answer[last] +=1;
        while(answer[last] %10==0){

            if last ==0{
                answer[last] = 1;
                answer.push(0);
                break;
            }
            else {
                answer[last] = 0;
                answer[last-1] +=1;
                last -=1;
            }
            
            
        }
        answer
    }
}
