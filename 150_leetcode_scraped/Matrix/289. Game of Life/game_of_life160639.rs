// https://leetcode.com/problems/game-of-life/solutions/160639/python-readable-and-100-solution/
def friends_fn(board):
    "Fn returning sum for given cell."
    xmax = len(board)
    ymax = len(board[0])
    
    nbs = [
        (x, y) 
        for x in range(-1, 2) for y in range(-1, 2)
        if not (x == 0 and y == 0)
    ]

    def helper(x, y):
        return sum(
            board[x + nx][y + ny]
            for nx, ny in nbs
            if (x + nx >=0 and x + nx < xmax) and
               (y + ny >= 0 and y + ny < ymax)
        )
    
    return helper

def nbs_to_gen(current, v):
    "Given current cell and sum of neighbors return next gen for cell."
    if current:
        if v < 2:
            return 0
        elif v < 4:
            return 1
        elif v > 4:
            return 0
    else:
        if v == 3:
            return 1
        
    return 0

class Solution(object):
    def gameOfLife(self, board):
        """
        :type board: List[List[int]]
        :rtype: void Do not return anything, modify board in-place instead.
        """
        if not board:
            return board
        
        new_board = [[0 for _ in range(len(board[0]))] for _ in range(len(board))]
        friends = friends_fn(board)
        for x in range(len(board)):
            for y in range(len(board[0])):
                new_board[x][y] = nbs_to_gen(board[x][y], friends(x, y))
                
        board[:] = new_board
	```