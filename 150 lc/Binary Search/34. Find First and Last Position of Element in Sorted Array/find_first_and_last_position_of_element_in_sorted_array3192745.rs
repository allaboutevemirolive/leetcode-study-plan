// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/solutions/3192745/rust-solution-using-binary-search/
type Target = i32;
type UseValue = i32;
fn upper_bound(arr: &Vec<Target>, x: &UseValue) -> usize {
    let mut low = 0;
    let mut high = arr.len();
    while low != high {
        let mid = (low + high) / 2;
        match arr[mid].cmp(x) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                low = mid + 1;
            }
            std::cmp::Ordering::Greater => {
                high = mid;
            }
        }
    }
    low
}
fn lower_bound(arr: &Vec<Target>, x: &UseValue) -> usize {
    let mut low = 0;
    let mut high = arr.len();
    while low != high {
        let mid = (low + high) / 2;
        match arr[mid].cmp(x) {
            std::cmp::Ordering::Less => {
                low = mid + 1;
            }
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                high = mid;
            }
        }
    }
    low
}
impl Solution {
  pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let n = nums.len();
    let li = lower_bound(&nums, &target);
    if li == n || nums[li] != target { return vec![-1,-1] }

    let ri = upper_bound(&nums, &target);
    if ri == n {
      return vec![li as i32, (n-1) as i32]
    }
    let ri = if nums[ri] == target {
      ri
    } else {
      ri - 1
    };
    vec![li as i32, ri as i32]
  }
}