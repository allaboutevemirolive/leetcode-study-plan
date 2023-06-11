// https://leetcode.com/problems/partition-list/solutions/1799179/two-easy-solutions-o-n/
def partition(self, head: Optional[ListNode], x: int) -> Optional[ListNode]:
        curr = head
        less = []
        greater = []
        while curr:
            if curr.val<x:
                less.append(curr.val)
            else:
                greater.append(curr.val)
            curr = curr.next
        
        curr = head
        for i in less:
            curr.val = i
            curr = curr.next
        
        for i in greater:
            curr.val = i 
            curr = curr.next
        
        return head