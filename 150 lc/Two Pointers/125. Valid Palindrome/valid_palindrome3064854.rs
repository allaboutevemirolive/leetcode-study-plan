// https://leetcode.com/problems/valid-palindrome/solutions/3064854/fastest-c-solution-3ms-using-two-pointer-method/
class Solution {
public:
    #define pb push_back
    bool isPalindrome(string s) {
        string str{""};
        for(int i=0;i<s.size();i++){
            if(isalnum(s[i])) str.pb(tolower(s[i]));
        }
        int l =0;
        int r = str.length() -1;
        while(l < r){
            if(str[l] != str[r]) return false;
            l++;
            r--;
        }
        return true;
    }
};