// https://leetcode.com/problems/min-stack/solutions/3211868/rust-solution-with-dynamic-programming/
struct MinStack {
    stack: Vec<i32>,
    min_values: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        Self {
            stack: vec![],
            min_values: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        match self.min_values.get(self.min_values.len() - 1) {
            None => self.min_values.push(val),
            Some(min_value) => {
                if min_value > &val {
                    self.min_values.push(val);
                } else {
                    self.min_values.push(*min_value)
                }
            }
        }
    }

    fn pop(&mut self) {
        self.min_values.pop();
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack[self.stack.len() - 1]
    }

    fn get_min(&self) -> i32 {
        self.min_values[self.min_values.len() - 1]
    }

}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */