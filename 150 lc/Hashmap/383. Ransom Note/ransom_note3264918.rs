// https://leetcode.com/problems/ransom-note/solutions/3264918/functional-rust/
impl Solution {
    pub fn can_construct(mut ransom_note: String, mut magazine: String) -> bool {
        use std::ops::ControlFlow;
        let mut char_map = [0;26];
        magazine
            .drain(..)
            .for_each(|c| char_map[c as usize - 97] += 1);
        match ransom_note
            .drain(..)
            .try_for_each(|c| {
                let indice = c as usize - 97;
                match char_map[indice] {
                    0 => ControlFlow::Break(false),
                    _ => {
                        char_map[indice] -= 1;
                        ControlFlow::Continue(())
                    },
                }
            }) {
                ControlFlow::Break(_) => false,
                ControlFlow::Continue(()) => true,
            }
    }
}