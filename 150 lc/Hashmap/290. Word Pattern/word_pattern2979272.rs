// https://leetcode.com/problems/word-pattern/solutions/2979272/two-hashmaps-in-c-rust-go/
class Solution {
public:
    bool wordPattern(const std::string & pattern, std::string s) {
        std::unordered_map<char, size_t> p2i;
        std::unordered_map<std::string, size_t> w2i;
        size_t idx = 0;
        std::istringstream in(std::move(s));
        for (std::string word; in >> word; ++idx) {
            if (idx == pattern.size() || p2i[pattern[idx]] != w2i[word])
                return false;
            p2i[pattern[idx]] = w2i[word] = idx + 1;
        }
        return idx == pattern.size();
    }
};