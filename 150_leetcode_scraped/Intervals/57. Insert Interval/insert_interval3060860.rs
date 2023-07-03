// https://leetcode.com/problems/insert-interval/solutions/3060860/binary-search-o-n-in-c-rust/
class Solution {
public:
    std::vector<std::vector<int>> insert(
        std::vector<std::vector<int>> & intervals,
        std::vector<int> & newInterval
    ) {
        const auto [leIt, gtIt] = std::equal_range(intervals.begin(), intervals.end(),
            newInterval, [](const auto & existingInterval, const auto & newInterval) {
                return existingInterval[1] < newInterval[0];
            });

        if (leIt == gtIt) {
            intervals.insert(gtIt, std::move(newInterval));
        } else {
            (*leIt)[0] = std::min(newInterval[0], (*leIt)[0]),
            (*leIt)[1] = std::max(newInterval[1], (*std::prev(gtIt))[1]);
            intervals.erase(std::next(leIt), gtIt);
        }

        return std::move(intervals);
  }
};