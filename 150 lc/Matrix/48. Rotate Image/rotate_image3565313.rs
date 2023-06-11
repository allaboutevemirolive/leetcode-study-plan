// https://leetcode.com/problems/rotate-image/solutions/3565313/java-easy-understanding-solution/
class Solution {
    public void rotate(int[][] matrix) {
        int n = matrix.length;
        int maxLayer = n / 2;
        int layer = 0;
        int top, right, bottom, left;
        int pos = n;
        while (layer <= maxLayer) {
            for (int i = 0; i < n - 1; i++) {
                top = matrix[layer][layer + i];
                right = matrix[layer + i][pos - 1];
                bottom = matrix[pos - 1][pos - 1 - i];
                left = matrix[pos - 1 - i][layer];

                matrix[layer + i][pos - 1] = top;
                matrix[pos - 1][pos - 1 - i] = right;
                matrix[pos - 1 - i][layer] = bottom;
                matrix[layer][layer + i] = left;
            }
            n -= 2;
            layer++;
            pos--;
        }
    }
}