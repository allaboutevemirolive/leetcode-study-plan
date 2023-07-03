// https://leetcode.com/problems/set-matrix-zeroes/solutions/1583862/c-solution/
public class Solution {
    public void SetZeroes(int[][] matrix) {
        var hs = new Stack<(int, int)>();
        for(var i = 0; i<matrix.Length; i++){
            for(var j =0; j<matrix[i].Length; j++){
                if(matrix[i][j]==0){
                    hs.Push((i, j));
                }
            }
        }
        while(hs.Count>0){
            var p = hs.Pop();
            for(var i = 0; i<matrix.Length; i++){
                matrix[i][p.Item2] = 0;
            }
            for(var i = 0; i<matrix[p.Item1].Length; i++){
                matrix[p.Item1][i] = 0;
            }
        }
    }
}