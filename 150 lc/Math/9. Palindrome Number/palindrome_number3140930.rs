// https://leetcode.com/problems/palindrome-number/solutions/3140930/s/
class Solution {
    public boolean isPalindrome(int x) {
       
        
        

        String few = x + "";
        char hio;
        StringBuilder sb = new StringBuilder();

        for (int s= few.length()- 1; s >= 0; s--){
          hio =  few.charAt(s);
          sb.append(hio);

        }
        String reverse = sb.toString();
        if (reverse.equals(few)){
            return true;

        }
        else {
            return false;
        }


    }
}
        
    
