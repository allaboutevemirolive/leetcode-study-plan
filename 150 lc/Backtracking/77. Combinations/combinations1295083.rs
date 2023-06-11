// https://leetcode.com/problems/combinations/solutions/1295083/c-solution-using-stack/
public class Solution {
    public IList<IList<int>> Combine(int n, int k) {
        var res = new List<IList<int>>();
        var stack = new Stack<List<int>>();
        for(var i=1; i<=n; i++){
            stack.Push(new List<int>(){i});
        }
        while(stack.Count>0){
            var item = stack.Pop();
            if(item.Count==k){
                res.Add(item);
                continue;
            }
            for(var i=item[item.Count-1]+1; i<=n; i++){
                var s = new List<int>(item);
                s.Add(i);
                stack.Push(s);
            }
        }
        return res;
    }
}