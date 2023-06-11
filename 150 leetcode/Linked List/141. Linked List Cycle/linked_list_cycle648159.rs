// https://leetcode.com/problems/linked-list-cycle/solutions/648159/rust-faster-and-slow-pointer/
pub fn has_cycle(head: List<i32>, i: i32) -> bool {
   let head = head.head.as_ref();
   let mut slow = head;
   let mut fast = head;
   while fast.is_some() {
      println!(
         "start fast {} {}",
         fast.unwrap().next.is_none(),
         fast.unwrap().elem
      );
      let nx = fast.unwrap().next.as_ref();
      if nx.is_none() {
         return false;
      }
      println!("slow {} fast {}", slow.unwrap().elem, fast.unwrap().elem);
      slow = slow.unwrap().next.as_ref();
      fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
      // 如果是None，說明已經循環完了，此時需要將fast指向到i所在的位置。
      if fast.is_none() && i >= 0 {
         let mut tmp = head;
         for _ in 0..i {
            tmp = tmp.unwrap().next.as_ref();
         }
         fast = tmp;
      }
      if slow.is_none() && i >= 0 {
         let mut tmp = head;
         for _ in 0..i {
            tmp = tmp.unwrap().next.as_ref();
         }
         slow = tmp;
      }
      println!(
         "slow-next {} fast-next {}",
         slow.unwrap().elem,
         fast.unwrap().elem
      );
      if slow == fast {
         return true;
      }
   }
   return false;
}