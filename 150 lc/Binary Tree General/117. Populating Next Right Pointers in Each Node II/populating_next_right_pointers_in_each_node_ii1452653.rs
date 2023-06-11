// https://leetcode.com/problems/populating-next-right-pointers-in-each-node-ii/solutions/1452653/c-level-order-traversal-solution/
public class Solution {
    public Node Connect(Node root) {
        if(root==null) return null;
        var queue = new Queue<Node>();
        queue.Enqueue(root);
        var width = 1;
        while(queue.Count>0){
            var count = 0;
            var nc = 0;
            Node prev =null;
            while(count < width){
                var n = queue.Dequeue();
                if(prev ==null){
                    prev = n;
                }else{
                    prev.next = n;
                    prev = n;
                }
                if(n.left!=null){
                    queue.Enqueue(n.left);
                    nc++;
                }
                if(n.right!=null){
                    queue.Enqueue(n.right);
                    nc++;
                }
                count++;
            }
            width = nc;
        }
        return root;
    }
}