// https://leetcode.com/problems/median-of-two-sorted-arrays/solutions/3467992/o-log-min-m-n-solution-with-detailed-inline-explanation-of-how-and-why-it-works-rust/
impl Solution {
    fn get_median_values(half_len: usize, nums1: &[i32], ln1: usize, nums2: &[i32]) -> Result<(i32, i32), Direction> {
        // Let's imagine that we already have combined and sorted array that
        // contains all elements of `nums1` and `nums2`. For simplicity,
        // assume that this combined array have even length.
        //
        //     nums1: [1, 3]
        //     nums2: [2, 4]
        //     combined: [1, 2, 3, 4]
        //
        //     combined.len() = nums1.len() + nums2.len()
        //
        // By definition, median splits our combined array right at the
        // middle into two halves of equal length.
        //
        //     median = 2.5
        //     [1, 2 ! 3, 4]
        //
        // As both `nums1` and `nums2` are already sorted, we are able to
        // split each of them into exactly two parts such that all elements
        // from the left parts will be less than or equal to the median
        // and thus may go into the combined array to the left of the
        // median, and all elements from the right parts will be greater
        // than or equal to the median and thus may go into the combined
        // array to the right of the median.
        //
        //     median = 2.5
        //     [1 ! 3], [2 ! 4]
        //      |    \  /    |
        //      |     X      |
        //      |    /  \    |
        //     [1,  2  ! 3,  4]
        //
        // In some cases, such split positions may be at the edges:
        //
        //     nums1: [1, 2]
        //     nums2: [3, 4]
        //     combined: [1, 2, 3, 4]
        //     median = 2.5
        //     [1, 2 !], [! 3, 4]
        //      |  |        |  |
        //      |  |        |  |
        //      |  |        |  |
        //     [1, 2   !    3, 4]
        //
        // Let's name some variables:
        //
        //     `ln1` and `ln2` - lengths of left parts of `nums1` and
        //                       `nums2` respectively after split is made.
        //     `rn1` and `rn2` - lengths of right parts of `nums1` and
        //                       `nums2` respectively.
        //
        // Examples:
        //
        //     [1 ! 3], [2 ! 4]
        //     |     |  |     \--- rn2 = 1
        //     |     |  \--------- ln2 = 1
        //     |     \------------ rn1 = 1
        //     \------------------ ln1 = 1
        //
        //     [1, 2 !], [! 3, 4]
        //     |      |  |      \- rn2 = 2
        //     |      |  \-------- ln2 = 0
        //     |      \----------- rn1 = 0
        //     \------------------ ln1 = 2
        //
        // Because median splits combined array into halves of equal length
        // and because combined array consists of elements of `nums1` and
        // `nums2`, we are always able to make a split such that following
        // is true:
        //
        //     / ln1 + ln2 = rn1 + rn2
        //     { ln1 + rn1 = nums1.len()
        //     \ ln2 + rn2 = nums2.len()
        //
        // `nums1` and `nums2` split positions are definitely related to
        // each other. And so we can calculate split position for `nums2`
        // (`ln2`) if we know split position for `nums1` (`ln1`):
        //
        //     ln1 + ln2 = rn1 + rn2
        //     ln2 = rn1 + rn2 - ln1
        //     ln2 + ln2 = rn1 + (ln2 + rn2) - ln1                 # ln2 + rn2 = nums2.len()
        //     2 * ln2 = rn1 + nums2.len() - ln1
        //     ln2 = (rn1 + nums2.len() - ln1) / 2                 # ln1 + rn1 = nums1.len() => rn1 = nums1.len() - ln1
        //     ln2 = (nums1.len() - ln1 + nums2.len() - ln1) / 2
        //     ln2 = (nums1.len() + nums2.len() - 2 * ln1) / 2
        //     ln2 = (nums1.len() + nums2.len()) / 2 - ln1
        //
        // As an optimization, we can compute half of the combined length
        // once at the start.
        let ln2 = half_len - ln1;

        // Now, we need to get values from `nums1` and `nums2` around the
        // split. Since split can be made at the edge of an array, we need
        // some dummy values that simultaneously do not appear in the real
        // input and do not violate the sorting order of already sorted
        // arrays. Like `-Inf` and `+Inf`. In a case of `i32` we may use
        // `i32::MIN` and `i32::MAX` because both are way beyond of the
        // specified input range for this task (-10^6 <= nums1[i], nums2[i] <= 10^6).
        // Also, we may safely underflow `usize` to create a non-existing
        // array index as maximum combined length for this task is just 2000.
        //
        //     lv1 - maximal (rightmost) value of the left part
        //           (less than or equal to median) of `nums1`
        //     rv1 - minimal (leftmost) value of the right part
        //           (greater than of equal to median) of `nums1`
        //     lv2 - maximal (rightmost) value of the left part
        //           (less than or equal to median) of `nums2`
        //     rv2 - minimal (leftmost) value of the right part
        //           (greater than of equal to median) of `nums2`
        //
        // Examples:
        //
        //     [1 ! 3], [2 ! 4]
        //      |   |    |   \---- rv2 = 4
        //      |   |    \-------- lv2 = 2
        //      |   \------------- rv1 = 3
        //      \----------------- lv1 = 1
        //
        //     [1, 2 !], [! 3, 4]
        //         |  |  |  \----- rv2 = 3
        //         |  |  \-------- lv2 = -2_147_483_648
        //         |  \----------- rv1 = 2_147_483_647
        //         \-------------- lv1 = 2
        let lv1 = nums1.get(ln1.overflowing_sub(1).0).unwrap_or(&i32::MIN);
        let rv1 = nums1.get(ln1).unwrap_or(&i32::MAX);
        let lv2 = nums2.get(ln2.overflowing_sub(1).0).unwrap_or(&i32::MIN);
        let rv2 = nums2.get(ln2).unwrap_or(&i32::MAX);

        // Now, we need a way to check whether we actually found the median
        // for any arbitrary split position.
        match (lv1 <= rv2, lv2 <= rv1) {
            // Consider following example again:
            //
            //     median = 2.5
            //     [1 ! 3], [2 ! 4]
            //      |   |    |   \---- rv2 = 4
            //      |   |    \-------- lv2 = 2
            //      |   \------------- rv1 = 3
            //      \----------------- lv1 = 1
            //
            // This split position is clearly right, we have found the median!
            // Here both left parts contain values less than median and both
            // right parts contain values greater than median.
            (true, true) => {
                // We do not know in which order these elements from `nums1`
                // and `nums2` appear in the combined array, so we must sort
                // them manually to find which one is closer to the median:
                let left = lv1.max(lv2);
                let right = rv1.min(rv2);

                // And now we have values around the median.
                Ok((*left, *right))
            },

            // At this point, we can simply check all possible split positions
            // for `nums1` to find the right one, but this is not an optimal
            // solution.

            // Now, let's see what will happen if our split position of choice
            // is slightly left of the true median.
            //
            //     median = 2.5
            //     [! 1, 3], [2, 4 !]
            //     |  |          |  \- rv2 = 2_147_483_647
            //     |  |          \---- lv2 = 4
            //     |  \--------------- rv1 = 1
            //     \------------------ lv1 = -2_147_483_648
            //
            // `lv1` is still less than `rv2`, but `lv2` is now greater
            // than `rv1`.
            //
            // Whenever we move our `nums1` split position further to the
            // left, both `lv1` and `rv1` get smaller. Split position for
            // `nums2` moves in the opposite direction and so `lv2` and
            // `rv2` get bigger. This means that we found a condition that
            // always holds when median is somewhere at the right of our
            // current split position.
            (true, false) => Err(Direction::Right),

            // Condition for "median to the left" is opposite:
            //
            //     median = 2.5
            //     [1, 3 !], [! 2, 4]
            //         |  |  |  \- rv2 = 2
            //         |  |  \---- lv2 = -2_147_483_648
            //         |  \--------------- rv1 = 2_147_483_647
            //         \------------------ lv1 = 3
            (false, true) => Err(Direction::Left),

            // Now, we know where median is for any given split position even
            // if position itself is wrong. This gives us the ability to use
            // binary search instead of the brute force.

            // The remaining question is: is it even possible that both
            // `lv1 > rv2` and `lv2 > rv1`? Yes, but only if both arrays are
            // not sorted around split positions, which breaks the constraints
            // of this task:
            //
            //     [3 ! 1], [4 ! 2]
            //      |   |    |   \---- rv2 = 2
            //      |   |    \-------- lv2 = 4
            //      |   \------------- rv1 = 1
            //      \----------------- lv1 = 3
            (false, false) => unreachable!("Initial arrays are not sorted"),
        }

        // You may also notice that for some initial `nums1` and `nums2` there
        // are more than one split position that gives us median. Here is an
        // example:
        //
        //     nums1: [0, 0]
        //     nums2: [0, 0]
        //
        // Obviously, this happens because equal values from `nums1` and
        // `nums2` may occupy different positions inside our imaginary
        // combined sorted array while still being otherwise equal.
        //
        // This does not affect our reasoning.
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // We are going to search for the right split position on just one of
        // the arrays so it is beneficial to choose shorter array for this.
        // This way we will need less iterations.
        let (short, long) = if nums1.len() <= nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };

        // Previously mentioned optimization: compute half of the combined
        // length once.
        let combined_len = short.len() + long.len();
        let half_len = combined_len / 2;

        // Here we finally start our binary search:
        let mut range = 0..=short.len();
        loop {
            // First, we find split position somewhere at the middle of a
            // current search range.
            let split = range.start() + (range.end() - range.start()) / 2;

            // Second, we check whether we chose right split position for
            // the median.
            match Self::get_median_values(half_len, &short, split, &long) {
                // Miss, but we know direction to the median.
                //
                // Median at the left => narrow down search to the left half
                // of the current search range, excluding current split
                // position.
                Err(Direction::Left) => range = *range.start()..=split - 1,
                // Median at the right => narrow down search to the right half
                // of the current search range, excluding current split
                // position.
                Err(Direction::Right) => range = split + 1..=*range.end(),

                // Hit!
                Ok((left, right)) => break if combined_len % 2 == 0 {
                    // For an even-length array we simply return the average of
                    // values around the median.
                    (left as f64 + right as f64) / 2.0
                } else {
                    // But what happens when we have odd-length combined array?
                    //
                    // First of all, we can simply duplicate each element af
                    // each input array so that we will get an even-length
                    // array again. This way our existing algorithm will be
                    // able to work without any changes at all.
                    //
                    // But this is not needed. Our reasoning about two
                    // "halves" of a combined array is still valid, but one of
                    // the "halves" will be one element longer than other. We
                    // just need to choose which one. As we already use integer
                    // division for `half_len`, left "half" will be shorter.
                    // And this means that we always have our median at the
                    // right of the split position.
                    right as f64
                },
            }
        }
    }
}

enum Direction {
    Left,
    Right,
}

type Range = core::ops::RangeInclusive<usize>;