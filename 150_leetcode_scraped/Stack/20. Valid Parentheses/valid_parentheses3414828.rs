// https://leetcode.com/problems/valid-parentheses/solutions/3414828/c-easy-solution/
class Solution {
public:
    bool isValid(string s) {
        stack<char> steck;
        for(int i=0; i<s.length(); ++i){
            if(steck.empty()){
                steck.push(s[i]);
            }
            else if((steck.top()=='(' && s[i]==')') || (steck.top()=='{' && s[i]=='}') || (steck.top()=='[' && s[i]==']')){
                steck.pop();
            }
            else{
                steck.push(s[i]);
            }
        }
        return steck.empty();
    }
};