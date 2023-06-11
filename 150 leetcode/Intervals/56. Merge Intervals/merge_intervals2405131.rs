// https://leetcode.com/problems/merge-intervals/solutions/2405131/easy-to-understand/
public class Solution 
{
    public int[][] Merge(int[][] intervals) 
    {
        int count = 0;
        Interval [] iVal= new Interval[intervals.Length];
        foreach(var arr in intervals)
        {
            iVal[count++] = new Interval(arr[0], arr[1]);
        }
        
        return  AppHelper.mergeIntervals(iVal,iVal.Length);
    }
}

public static class AppHelper
{
    public static int [][] mergeIntervals(Interval[] arr, int n)
    {
        Array.Sort(arr, new MyCmp());
        int res = 0;
        List<int[]> lst = new List<int[]>();
        for (int i = 1; i < n; i++)
        {
            if (arr[i].start<=arr[res].end)
            {
                arr[res].end = Math.Max(arr[res].end, arr[i].end);
                arr[res].start = Math.Min(arr[res].start, arr[i].start);
                
                
            }
            else
            {
                res++;
                arr[res] = arr[i];
            }
        }
       
        for (int i = 0; i <= res; i++)
        {
            int[] array = new int[] { arr[i].start, arr[i].end };
            lst.Add(array);
        }
        return lst.ToArray();  
    }
}
public class Interval
{
    public int start;
    public int end;

    public Interval(int s, int e)
    {
        start = s;
        end = e;
    }
}
public class MyCmp : IComparer<Interval>
{
    public int Compare(Interval x, Interval y)
    {
       return x.start - y.start;
    }      
}