// https://leetcode.com/problems/palindrome-number/solutions/3369789/solution-for-beginner/
class Solution {
public:
    bool isPalindrome(int x) {
        if(x<0){
            return{false};
        }
        else{
            string str=to_string(x);
            for(int i=0; i<ceil(static_cast<double>(str.length())/2); ++i){
                if(str[i]!=str[str.length()-(i+1)]){
                    return{false};
                }
            }
        }
        return{true};
    }
};