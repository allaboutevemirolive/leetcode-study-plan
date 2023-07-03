// https://leetcode.com/problems/maximum-sum-circular-subarray/solutions/1025977/c-o-n/
public class Solution 
{
    public static  int Max_SubarraySum(Int32 [] arr,Int32 n)
    {
        Int32 res=arr[0];
        Int32 max_ending=arr[0];
        for(int i=1;i<n;i++)
        {
           max_ending= Math.Max(max_ending+arr[i],arr[i]);
           res=Math.Max(res,max_ending);
        }
        return res;
    }
    
    public int MaxSubarraySumCircular(int[] A) 
    {
        int n=A.Length;
        int max=Max_SubarraySum(A,n);
        if(max<0)
        {
            return max;
        }
        else
        {
            int arr_sum=0;
            for(int i=0;i<n;i++)
            {
                arr_sum+=A[i];
                A[i]=-A[i];
            }
            int maxsum_circular= arr_sum+ Max_SubarraySum(A,n);
            return Math.Max(maxsum_circular,max);
        }
    }
}