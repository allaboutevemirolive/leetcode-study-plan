// https://leetcode.com/problems/valid-parentheses/solutions/3447061/o-n-in-c-rust-go/
class Solution {
public:
    bool isValid(std::string_view s) {
        std::stack<char, std::string> stack;
        for (const char c : s) {
            switch (c) {
                case '(': stack.push(')'); continue;
                case '[': stack.push(']'); continue;
                case '{': stack.push('}'); continue;
                default:
                    if (stack.empty() || stack.top() != c) {
                        return false;
                    }
                    stack.pop();
            }
        }
        return stack.empty();
    }
};