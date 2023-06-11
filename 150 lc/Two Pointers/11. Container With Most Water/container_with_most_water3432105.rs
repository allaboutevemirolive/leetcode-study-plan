// https://leetcode.com/problems/container-with-most-water/solutions/3432105/o-n-in-c-go-rust/
class Solution {
public:
    int maxArea(const std::vector<int> & height) {
        int result = 0;
        for (int i = 0, j = height.size() - 1; i < j;) {
            result = std::max(result, (j - i) * std::min(height[i], height[j]));
            (height[i] < height[j]) ? ++i : --j;
        }
        return result;
    }
};