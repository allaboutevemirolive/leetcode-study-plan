// https://leetcode.com/problems/reverse-linked-list-ii/solutions/1292955/simple-c-solution/
public class Solution {
    public ListNode ReverseBetween(ListNode head, int left, int right) {
        ListNode current = null;
        var pos = 0;
        //Move to one position between start position
        while(pos+1 < left){
            if(pos == 0){
                current = head;
            }else{
                current = current.next;
            }
            pos++;
        }
        var pre = current;
        var stack = new Stack<ListNode>();
        //handling if left is the head node
        if(current == null){
            stack.Push(head);
            current = head;
            pos++;
        }
        //push all nodes in interval to stack.
        while(pos<right){
            stack.Push(current.next);
            current = current.next;
            pos++;
        }
        //holds the head of rest of linked list
        var nx = current.next;
        //if left is head, get new head;
        if(pre == null){
            head = stack.Pop();
            pre = head;
        }
        //reverse the portion of list
        while(stack.Count>0){
            pre.next = stack.Pop();
            pre = pre.next;
        }
        //hook up the linked list
        pre.next = nx;
        return head;
    }
}