// https://leetcode.com/problems/number-of-islands/solutions/3027286/rust-c-11m-s-9-2-mb-union-by-rank/
#[derive(Debug, Clone)]
struct DisjointSet { 
    parent: Vec<usize>,
    rank: Vec<usize>
}

impl DisjointSet { 
    fn new(n: usize) -> Self { 
        let mut parent = vec![0; n];
        let mut rank = vec![0; n ];
        for i in 0..n { 
            parent[i] = i;
            rank[i] = 0;
        }
        Self { parent, rank }
        
    }
    fn find(&mut self, n: usize) -> usize { 
        if self.parent[n] == n { 
            return n
        }
        self.parent[n] = self.find(self.parent[n]);
        self.parent[n]
    }
    fn merge(&mut self, n: usize, m: usize) -> bool { 
        let n_root = self.find(n);
        let m_root = self.find(m);
        
        if n_root == m_root { return false; }
        
        if self.rank[n_root] < self.rank[m_root] { 
            self.parent[n_root] = m_root;
            
        } else if self.rank[m_root] < self.rank[n_root] { 
            self.parent[m_root] = n_root;
        
        } else { 
            self.parent[m_root] = n_root;
            self.rank[n_root] +=1;
        }
        
        return true;
    }
}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        
        let (row, col) = (grid.len(), grid[0].len());
        let mut ds = DisjointSet::new(row * col);
        let mut counted_islands = 0;
        
        for r in 0..row { 
            for c in 0..col { 
               if grid[r][c] == '1' { 
                   counted_islands +=1; 
                    let directions = vec![(r, c + 1), (r, c - 1), (r + 1, c), (r - 1, c)];
                    for &(dr, dc) in &directions {
                        let (curr, next) = (dr * col + dc, col * r + c);
                        
                        if dr < row && dc < col && grid[dr][dc] == '1' {
                            if ds.find(curr) != ds.find(next) {
                                if ds.merge(curr, next) {
                                    counted_islands -=1;

                                }
                            }
                        }
                        
                    }
                }
            }
        }
        counted_islands
    }
}