// https://leetcode.com/problems/powx-n/solutions/3333920/c-solution/
class Solution {
public:
    double myPow(double x, int n) {
        if(x > -100.0 && x < 100.0 && n >= -2147483648 && n <= 2147483647){
            if(pow(x,n) >= -10000 && pow(x, n) <= 10000){
                double answer = pow(x, n);
                return answer;
            }
            else{
                return 0;
            }

        }
        else{
            return 0;
        }
        
    }
};