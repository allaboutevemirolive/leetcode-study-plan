// https://leetcode.com/problems/min-stack/solutions/2172573/rust-1-line-per-op-10ms/
use std::cmp;

struct MinStack {
    stack: Vec<(i32, i32)>
}


impl MinStack {

    fn new() -> Self {
        Self{stack: Vec::new()}
    }
    
    fn push(&mut self, val: i32) {
        self.stack.push((val, cmp::min(self.stack.last().unwrap_or(&(0, i32::MAX)).1, val)));
    }
    
    fn pop(&mut self) {
        self.stack.pop();
    }
    
    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }
    
    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}