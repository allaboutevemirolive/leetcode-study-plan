// https://leetcode.com/problems/binary-tree-level-order-traversal/solutions/3068021/c-easy-solution-based-on-breath-first-search/
#include <queue>

class Solution {
public:
    vector<vector<int>> levelOrder(TreeNode* root) {
        
        std::vector<std::vector<int>> result;
        std::queue<TreeNode*> nodes;

        if (root == nullptr) {
            return result;
        }

        nodes.push( root );

        while (!nodes.empty()) {

            int size = nodes.size(), count = 0;
            std::vector<int> levelNodes(size);

            while (count < size) {

                auto node = nodes.front();
                nodes.pop();

                levelNodes.at(count) = node->val;

                if ( node->left ) {
                    nodes.push(node->left);
                }
            
                if ( node->right ) {
                    nodes.push(node->right);
                }

                count++;
            }

            result.push_back(std::move(levelNodes));
        }

        return result;
    }
};