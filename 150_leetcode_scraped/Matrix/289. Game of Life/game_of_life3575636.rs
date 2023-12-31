// https://leetcode.com/problems/game-of-life/solutions/3575636/java-in-place-solution-time-complexity-beats-100-space-complexity-beats-46/
class Solution {
    public void gameOfLife(int[][] board) {
        int[][] dirs = {{1, 0}, {1, 1}, {0, 1}, {1, -1}, {-1, 0}, {-1, -1}, {0, -1}, {-1, 1}};
        for (int i = 0; i < board.length; i++) {
            for (int i1 = 0; i1 < board[i].length; i1++) {
                int liveCount = 0;
                for (int[] dir : dirs) {
                    int x = i + dir[0];
                    int y = i1 + dir[1];
                    if (x < 0 || y < 0 || x >= board.length || y >= board[i].length) continue;
                    int target = board[x][y];
                    if (target > 0 && target != 30) {
                        liveCount++;
                    }
                }
                if (board[i][i1] == 0) {
                    if (liveCount == 3) {
                        board[i][i1] = 30;
                    }
                } else {
                    //equals 1
                    if (liveCount < 2) {
                        board[i][i1] = 11;
                    } else if (liveCount == 2 || liveCount == 3) {
                        board[i][i1] = 31;
                    } else {
                        board[i][i1] = 41;
                    }
                }
            }
        }
        for (int i = 0; i < board.length; i++) {
            for (int i1 = 0; i1 < board[i].length; i1++) {
                int target = board[i][i1];
                if (target == 30 || target == 31) {
                    board[i][i1] = 1;
                } else if (target == 11 || target == 41) {
                    board[i][i1] = 0;
                }
            }
        }
    }
}