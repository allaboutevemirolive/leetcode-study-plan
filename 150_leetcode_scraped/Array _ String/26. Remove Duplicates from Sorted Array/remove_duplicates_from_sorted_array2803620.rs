// https://leetcode.com/problems/remove-duplicates-from-sorted-array/solutions/2803620/rust-runtime-0ms-beat-100-o-n/
impl Solution {
    pub fn remove_duplicates(given: &mut Vec<i32>) -> i32 {

        let mut final_idx: usize = 1;
        let mut last: i32 = given[0];

        for index in 1..given.len(){
            if given[index] == last{
                continue;
            }
            last = given[index];
            given[final_idx] = given[index];
            final_idx+=1;
        }

        given.truncate(final_idx);
        // println!("{:?}", given);
        return final_idx as i32;
    }
}