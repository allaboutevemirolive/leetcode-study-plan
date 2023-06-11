// https://leetcode.com/problems/palindrome-number/solutions/3317747/c-solution-accepted/
class Solution {
public:
    bool isPalindrome(int x) {
        //Converting number to string
        std::string x_in_str = std::to_string(x);
        //Converting string to array
        vector<char> v(x_in_str.begin(), x_in_str.end());

        //Getting the size of the array
        int n = v.size();
        //Decrementing the size because arrays start counting
        //at 1
        n = n - 1;

        //Going through the array
        int i = 0;
        for(i = 0; i < n; i++) {
            //checks if the given member of the array is the 
            //same of the one at the same distance
            if(v[i] == v[n]) {
                n--;
            }
            else{
                return false;
            }
        }

        return true;
        
    }
};