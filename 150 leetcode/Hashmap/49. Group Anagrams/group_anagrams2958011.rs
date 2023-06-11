// https://leetcode.com/problems/group-anagrams/solutions/2958011/c-go-rust-with-complexity/
class Solution {
public:
    std::vector<std::vector<std::string>> groupAnagrams(const std::vector<std::string> & strs) {
        std::unordered_map<std::string, std::vector<std::string>> map;
        for (const auto & str : strs) {
            std::string key = str;
            std::sort(key.begin(), key.end());
            map[key].emplace_back(str);
        }

        std::vector<std::vector<std::string>> result;
        result.reserve(map.size());
        for (auto && [_, el] : map)
            result.emplace_back(el);
        return result;
    }
};