// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/solutions/1133093/java-recursive/
/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode() {}
 *     TreeNode(int val) { this.val = val; }
 *     TreeNode(int val, TreeNode left, TreeNode right) {
 *         this.val = val;
 *         this.left = left;
 *         this.right = right;
 *     }
 * }
 */

class Solution {
    public TreeNode buildTree(int[] preorder, int[] inorder) {
        // Empty array
        if (preorder == null || preorder.length == 0)
            return null;
        
        // First element of preorder is root
        TreeNode root = new TreeNode(preorder[0]);
       
        // Find the root element in inorder array, that would give us left and right children
        int splitpoint = 0;
        for (int i = 0; i < inorder.length; i++) {
            if (inorder[i] == preorder[0])
                splitpoint = i;
        }
        
        // Send new subarrays divided on splitpoint; divide and conquer!
        root.left = buildTree(Arrays.copyOfRange(preorder, 1, splitpoint + 1), Arrays.copyOfRange(inorder, 0, splitpoint));
        
        root.right = buildTree(Arrays.copyOfRange(preorder, splitpoint + 1, preorder.length), Arrays.copyOfRange(inorder, splitpoint + 1, inorder.length));
        
        return root;
    }
}