// https://leetcode.com/problems/maximum-depth-of-binary-tree/solutions/3067802/c-recursive-and-iterative-simple-and-readable-solutions/
class Solution {
public:
    int maxDepth(TreeNode* root) {
        if ( root == nullptr ) {
            return 0;
        }
        return std::max(maxDepth(root->left), maxDepth(root->right)) + 1;
    }
};