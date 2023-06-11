// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/solutions/2948563/2-pointer-o-n-c-c-python-java-rust-with-explanation/
arr[] = {2,4,5,6,7,8,9,11,15}; int target=12;
  Step-1: start=2, end=15, mid(index=4)=7
  Step-2: target > (start+mid). Move to right. start=8, end=15 ..
  
  Cannot find sum. Move to next element ie start=4