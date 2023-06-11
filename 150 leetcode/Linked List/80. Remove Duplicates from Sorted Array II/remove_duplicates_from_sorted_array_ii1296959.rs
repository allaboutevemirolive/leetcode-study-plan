// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/solutions/1296959/c-brutal-force-solution/
public class Solution {
    public int RemoveDuplicates(int[] nums) {
        var count = 1;
        var ans = nums.Length;//this controls the length of result array
        for(var i =1; i<ans; i++){
            if(nums[i]==nums[i-1]){
                count++;
            }
            if(nums[i]!=nums[i-1]){
                count=1;
            }
            if(count>2){
                var j = i;
                while(j+1<ans){
                    nums[j] = nums[j+1];
                    j++;
                }
                ans--;//upon remove any element, reduce length by one
                i--;//move back to current position
            }
        }
        return ans;
    }
}