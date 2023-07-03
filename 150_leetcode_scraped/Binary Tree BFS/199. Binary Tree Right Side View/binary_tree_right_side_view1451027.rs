// https://leetcode.com/problems/binary-tree-right-side-view/solutions/1451027/c-level-order-traversal/
public class Solution {
    public IList<int> RightSideView(TreeNode root) {
        var res = new List<int>();
        if(root ==null) return res;
        var queue = new Queue<TreeNode>();
        var width = 1;
        queue.Enqueue(root);
        while(queue.Count>0){
            var count = 0;
            var nc = 0;
            while(count < width){
                var node = queue.Dequeue();
                if(count == width-1){
                    res.Add(node.val);
                }
                if(node.left!=null){
                    queue.Enqueue(node.left);
                    nc++;
                }
                if(node.right!=null){
                    queue.Enqueue(node.right);
                    nc++;
                }
                count++;
            }
            width = nc;
        }
        return res;
    }
}