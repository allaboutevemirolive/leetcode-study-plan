// https://leetcode.com/problems/number-of-1-bits/solutions/2926061/count-number-of-1-after-convert-into-binary-by-rustam/
class Solution {
public:
    int hammingWeight(uint32_t n) {
        int count=0;
        while(n){
            if(n&1){
                count++;
            }

            n = n>>1;
        }
            
    return count;
    }
};