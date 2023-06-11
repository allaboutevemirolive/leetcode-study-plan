// https://leetcode.com/problems/rotate-list/solutions/1838884/simple-o-n-solution-non-recursive/
def rotateRight(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        curr = head
        length = 0
        while curr:
            length += 1
            curr = curr.next

        if length<=1:
            return head

        rotations = k % length
        if rotations==0:
            return head
        pos = length-1-rotations

        new_head = None
        curr = head
        i = 0
        while i<pos:
            i+=1
            curr = curr.next

        new_head = curr.next
        curr.next = None
        tail = new_head
        while tail.next:
            tail = tail.next

        tail.next = head
        return new_head