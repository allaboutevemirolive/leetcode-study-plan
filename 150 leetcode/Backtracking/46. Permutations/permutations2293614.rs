// https://leetcode.com/problems/permutations/solutions/2293614/c-rust-golang-0ms/
Solution {
public:
	vector<vector<int>> permute(vector<int>& nums) {
		vector<vector<int>> ans;
		f(0,nums.size(),nums,ans);
		return ans;
	}
	void f(int i, int n, vector<int>& nums, vector<vector<int>>& ans){
		if(i>=n){
			ans.push_back(nums);
			return;
		}
		for(int j=i;j<n;j++){
			swap(nums[i],nums[j]);
			f(i+1,n,nums,ans);
			swap(nums[i],nums[j]);
		}
	}
};