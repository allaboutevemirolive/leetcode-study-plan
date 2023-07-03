// https://leetcode.com/problems/find-median-from-data-stream/solutions/2806508/easy-rust-solution-heap/

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Default)]
struct MedianFinder {
    lo: BinaryHeap<i32>,
    hi: BinaryHeap<Reverse<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Default::default()
    }

    fn add_num(&mut self, num: i32) {
        self.lo.push(num);
        self.hi.push(Reverse(self.lo.pop().unwrap()));
        if self.lo.len() < self.hi.len() {
            self.lo.push(self.hi.pop().unwrap().0);
        }
    }

    fn find_median(&self) -> f64 {
        if self.lo.len() == self.hi.len() {
            return (self.lo.peek().unwrap() + self.hi.peek().unwrap().0) as f64 / 2.0;
        }
        *self.lo.peek().unwrap() as f64
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

fn main() {
    // Create a new instance of MedianFinder
    let mut median_finder = MedianFinder::new();

    // Add numbers to the MedianFinder
    median_finder.add_num(2);
    median_finder.add_num(10);
    median_finder.add_num(4);
    median_finder.add_num(7);
    median_finder.add_num(9);
    median_finder.add_num(1);
    median_finder.add_num(3);
    median_finder.add_num(7);

    // Find and print the median
    let median = median_finder.find_median();
    println!("Median: {}", median);
}

```

