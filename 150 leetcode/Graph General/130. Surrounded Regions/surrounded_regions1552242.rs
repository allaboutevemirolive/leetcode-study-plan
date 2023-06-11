// https://leetcode.com/problems/surrounded-regions/solutions/1552242/rust-solution-applying-union-find-algo-no-recursion-not-dfs-100-faster-etc/
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) 
    {
        let     (m, n) = (board.len(), board[0].len());
        let mut groups = Groups::new(m * n + 1);
        let     id     = |x, y| y * n + x + 1;
        let mut os     = vec![];
        
        // Scan the matrix to add all the connections to `groups`.
        for y in 0..m {
            for x in 0..n {
                if board[y][x] == 'O' {
                    os.push((x, y));
                    if x < n - 1 && board[y][x + 1] == 'O' {
                        groups.join(id(x, y), id(x + 1, y));
                    }
                    if y < m - 1 && board[y + 1][x] == 'O' {
                        groups.join(id(x, y), id(x, y + 1));
                    }
                    if y == m - 1 ||  x == n - 1 || 
                       y == 0     ||  x == 0 
                    {
                        // ID = 0 is a virtual cell that all other
                        // cells on the edges are connected to.
                        groups.join(id(x, y), 0);
                    }
                }
            }
        }
        // Go through all the 'O's and flip them to 'X' if they're not
        // connected directly, or indirectly, to ID = 0.
        for (x, y) in os {
            if !groups.connected(id(x, y), 0) {
                board[y][x] = 'X';
            }
        }
    }
}

// A class implementing "Union-Find" as described in 
// "Algorithms, Fourth Edition" - Sedgewick.
struct Groups {
    id: Vec<usize>,
    sz: Vec<usize>,
}

impl Groups {
    fn new(n: usize) -> Self {
        let id = (0..n).collect::<Vec<_>>();
        let sz = vec![1; n];
        Self { id, sz }
    }
    // Finds the root ID of the group `p` belongs to.
    fn find(&self, mut p: usize) -> usize {
        while p != self.id[p] { p = self.id[p]; }
        p
    }
    // Joins two ID's to the same group.
    fn join(&mut self, p: usize, q: usize) {
        let mut i = self.find(p);
        let mut j = self.find(q);
        if i != j {
            let (id, sz) = (&mut self.id, &mut self.sz);
            if sz[i] < sz[j] { id[i] = j; sz[j] += sz[i]; }
            else             { id[j] = i; sz[i] += sz[j]; }
        }
    }
    // Returns `true` if `p` and `q` are part of the same
    // connected group.
    fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }
}