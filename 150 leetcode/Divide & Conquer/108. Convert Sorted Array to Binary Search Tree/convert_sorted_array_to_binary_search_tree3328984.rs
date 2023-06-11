// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/solutions/3328984/rust-0ms-best-idiomatic/
use std::{cell::RefCell, rc::Rc};

impl Solution {
    pub fn sorted_array_to_bst(nums: impl AsRef<[i32]>) -> Option<Rc<RefCell<TreeNode>>> {
        split_the_middle(nums.as_ref()).map(|(left, middle, right)| {
            let node = TreeNode {
                val: *middle,
                left: Self::sorted_array_to_bst(left),
                right: Self::sorted_array_to_bst(right),
            };

            Rc::new(RefCell::new(node))
        })
    }
}

fn split_the_middle<T>(slice: &[T]) -> Option<(&[T], &T, &[T])> {
    if slice.is_empty() {
        return None;
    }

    let middle_index = slice.len() / 2;
    let (left, right_with_middle) = slice.split_at(middle_index);

    // Unwrap safety:
    //     we checked that the slice is not empty, so it's guaranteed
    //     that `right_with_middle` has at least one element.
    let (middle, right) = right_with_middle.split_first().unwrap();

    Some((left, middle, right))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_the_middle() {
        // Use `some()` to make the type system understand slices instead of arrays
        type SplitTuple<'a> = (&'a [i32], &'a i32, &'a [i32]);
        fn some(input: SplitTuple) -> Option<SplitTuple> {
            Some(input)
        }

        assert_eq!(split_the_middle(&[1, 2, 3, 4, 5]), some((&[1, 2], &3, &[4, 5])));
        assert_eq!(split_the_middle(&[1, 2, 3, 4]), some((&[1, 2], &3, &[4])));
        assert_eq!(split_the_middle(&[1, 2, 3]), some((&[1], &2, &[3])));
        assert_eq!(split_the_middle(&[1, 2]), some((&[1], &2, &[])));
        assert_eq!(split_the_middle(&[1]), some((&[], &1, &[])));
        assert_eq!(split_the_middle::<i32>(&[]), None);
    }
}