// https://leetcode.com/problems/maximum-sum-circular-subarray/solutions/3066012/daily-leetcoding-challenge-january-day-18/
class Solution {
public:
    int maxSubarraySumCircular(vector<int>& nums) {
	
        int n=nums.size();
		//First we would apply the kadanes algo in the normal array
		//for that we will have a keep count of continuous sum in the sum 
		//variable and if the cumulative sum gets less than zero we will assign sum to value zero and 
		//continue our iteration
        int sum=0,ans=-1e9;
       
        
        for(int i=0;i<n;i++){
            sum+=nums[i];
			//on every point after adding an element we are storing the maximum of ans and sum in the ans
            ans=max(ans,sum);
            if(sum<0){
                sum=0;
            }
        }
        
        sum=0;
		// we will have to find the sum that could be in case 2 so for that we 
		//will make an vector which will store the max value till the index
        vector<int>maxTill(n);
        
        for(int i=0;i<n;i++){
            sum+=nums[i];
            maxTill[i]=max(i>0?maxTill[i-1]:nums[i],sum);
        }
        // now we will start traversing from the last and add value to the cur variable and 
		//check if maxTill[i-1]+cur is greater than ans or not
        int cur=0;
        for(int i=n-1;i>=1;i--){
            cur+=nums[i];
            ans=max(ans,maxTill[i-1]+cur);
        }
        //returning the ans
        return ans;
    }
};