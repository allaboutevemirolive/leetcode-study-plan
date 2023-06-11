// https://leetcode.com/problems/min-stack/solutions/1559896/rust-simple-single-stack-two-examples-faster-than-98/
struct MinStack {
    min     : i32,
    stack   : Vec<i32>,
}
impl MinStack {
    fn new() -> Self {
        Self { stack: Vec::with_capacity(10_000), min: 0 }
    }
    fn push(&mut self, val: i32) {
        if self.stack.is_empty() {
            self.min = val;
            self.stack.push(val);
        } else {
            if val <= self.min {
                self.stack.push(self.min);
                self.min = val;
            } 
            self.stack.push(val);
        }
    }
    fn pop(&mut self) {
        if let Some(val) = self.stack.pop() {
            if val == self.min {
                if let Some(val) = self.stack.pop() {
                    self.min = val;
                }}}
    }
    fn top(&self) -> i32 {
        *self.stack.last().unwrap_or(&0)
    }
    fn get_min(&self) -> i32 {
        self.min
    }
}