// https://leetcode.com/problems/maximum-subarray/solutions/3353632/c-dynamic-programming-kadane-s-algo/
class Solution {
public:
    int maxSubArray(vector<int>& nums) {
        int max_sum = INT_MIN, cur_sum  = 0;
        for (const int &n : nums) {
            cur_sum = max(cur_sum + n, n);
            max_sum = max(cur_sum, max_sum);
        }
        return max_sum;
    }
};