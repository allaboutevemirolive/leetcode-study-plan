// https://leetcode.com/problems/linked-list-cycle/solutions/2077321/rust-typescript-two-pointers/
impl Solution { 
    fn has_cycle(head: Option<Box<ListNode>>) -> bool { 
        let (fast, slow) = (&head, &head);
        
        while fast.is_some()
			&& slow.is_some()
			&& fast.as_ref().unwrap().next.is_some() 
            && fast.as_ref().unwrap().next.as_ref().unwrap().next.is_some()
            && slow.next.is_some() { 
            
            if fast == slow {
                return true;
            }
            fast = &fast.as_ref().unwrap().next;
            slow = &slow.as_ref().unwrap().next;
        } 
        false 
    }
}