// https://leetcode.com/problems/word-search/solutions/2845634/backtracking-readable-simple-rust-easy-to-understand/
class Solution {
public:
    bool findWord(vector<vector<char>>& board, string word,int i,int j,int c){
        int n = board.size();
        int m = board[0].size();
        //index out of matrix and string condition
        if(i<0 || j<0 || i>=n || j>=m || c>=word.size()) return false;
        //checking the last character of word matches with board[i][j]
        if(c == word.size()-1 && word[c] == board[i][j]) return true;
        //any other character of word matches with board[i][j]
        if(word[c] == board[i][j]){
            char k = board[i][j];
            board[i][j] = '*';
            bool a = findWord(board,word,i+1,j,c+1);
            bool b = findWord(board,word,i,j+1,c+1);
            bool d = findWord(board,word,i-1,j,c+1);
            bool e = findWord(board,word,i,j-1,c+1);
            board[i][j] = k;
            return a || b || d || e;
        }
        return false;
    }
    bool exist(vector<vector<char>>& board, string word) {
        int n = board.size();
        int m = board[0].size();
        // backtracking for all elements of matrix
        for(int i=0;i<n;i++){
            for(int j=0;j<m;j++){
                bool res = findWord(board,word,i,j,0);
                if(res) return true;
            }
        }
        return false;
    }
};