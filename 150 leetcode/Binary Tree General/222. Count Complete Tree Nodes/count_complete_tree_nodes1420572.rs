// https://leetcode.com/problems/count-complete-tree-nodes/solutions/1420572/short-recursive-approach-in-c-using-queue-data-structure/
int countNodes(TreeNode* root) {
        if(!root) return 0;
        queue<TreeNode*> q;
        int c=0;
        TreeNode* temp;    
        q.push(root);
        while(!q.empty())
        {
            temp = q.front();
            q.pop();
            if(temp->left)
                q.push(temp->left);
            if(temp->right)
                q.push(temp->right);
            if((temp->left) or (temp->right))
                c++;
            if(!(temp->left) and !(temp->right))
                c++;
        }
        
        return c;
    }