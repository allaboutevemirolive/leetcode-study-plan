// https://leetcode.com/problems/partition-list/solutions/1224391/c-solution/
public class Solution {
    public ListNode Partition(ListNode head, int x) {
            ListNode Head = null;
            if(head == null){
                return Head;
            }
            ListNode Fisrt =null;
            ListNode SecondHead = null;
            ListNode Second = null;
            while(head!=null){
                if(head.val < x){
                    if(Fisrt == null){
                        Fisrt = head;
                        Head = Fisrt;
                    }else{
                        Fisrt.next = new ListNode(head.val);
                        Fisrt = Fisrt.next;
                    }
                }
                if(head.val >= x){
                    if(Second == null){
                        Second = new ListNode(head.val);
                        SecondHead = Second;
                    }else{
                        Second.next = new ListNode(head.val);
                        Second = Second.next;
                    }
                }
                head = head.next;
            }
            //every node is equal/greater than x
            if(Head == null && SecondHead != null){
                return SecondHead;
            }
            Fisrt.next = SecondHead;
            return Head;
    }
}