// https://leetcode.com/problems/isomorphic-strings/solutions/2815763/short-solution-using-hash-sets-in-c-and-rust/
class Solution {
public:
    bool isIsomorphic(string s, string t) {
        set<tuple<char, char>> set;
        unordered_set<char> ch_set_s, ch_set_t;
        
        for (int i = 0; i < s.size(); i++) {
            ch_set_s.insert(s[i]);
            ch_set_t.insert(t[i]);
            
            tuple<char, char> tup(s[i], t[i]);
            set.insert(tup);
        }
        
        return s.size() == t.size() &&
               ch_set_s.size() == ch_set_t.size() &&
               ch_set_s.size() == set.size();
    }
};