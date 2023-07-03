// https://leetcode.com/problems/single-number/solutions/2314591/easy-solution/
class Solution {
    public int singleNumber(int[] nums) {
        int value = 0;
        for(int num:nums)
            value^=num;
        return value;
    }
}