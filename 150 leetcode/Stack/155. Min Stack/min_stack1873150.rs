// https://leetcode.com/problems/min-stack/solutions/1873150/rust-solution-100-faster-0ms/
struct MinStack {
    stack: Vec<i32>,
    min: i32,
    len: i32,
}

impl MinStack {

    fn new() -> Self {
        MinStack{
            stack: Vec::new(),
            min:2147483647,
            len: 0,
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        self.len += 1;
        if val < self.min {
            self.min = val;
        }
    }

    fn pop(&mut self) {
        match self.stack.pop() {
            Some(val) => {
                self.len -= 1;
                if val == self.min {
                    if self.len > 0 {
                        self.min = 2147483647;
                        let mut temp_vec = self.stack.clone();
                        for i in temp_vec {
                            if i<self.min {
                                self.min=i;
                            }
                        }
                    }
                    else {
                        self.min = 2147483647;
                    }
                }
            },
            None => {}
        } ;
    }

    fn top(&mut self) -> i32 {
        self.stack[self.len as usize - 1]
    }

    fn get_min(&self) -> i32 {
        self.min
    }
}