// https://leetcode.com/problems/maximum-subarray/solutions/3067689/kadane-s-alghoritm-o-n-in-c-rust-go/
class Solution {
public:
    int maxSubArray(const std::vector<int> & nums) {
        return std::accumulate(std::next(nums.cbegin()), nums.cend(), nums[0],
            [localMax = nums[0]](const int max, const int num) mutable {
                localMax = std::max(localMax + num, num);
                return std::max(max, localMax);
            });
    }
};