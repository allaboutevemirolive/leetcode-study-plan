// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/solutions/3002679/o-n-log-n-in-c-rust-go/
class Solution {
public:
    int findMinArrowShots(std::vector<std::vector<int>> & points) {
        std::sort(points.begin(), points.end(), [](const auto & p1, const auto & p2) {
            return p1[1] < p2[1] || p1[1] == p2[1] && p1[0] < p2[0];
        });

        int result = 1, edge = points[0][1];
        for (const auto & point : points) {
            if (point[0] > edge) {
                edge = point[1];
                ++result;
            }
        }
        return result;
    }
};