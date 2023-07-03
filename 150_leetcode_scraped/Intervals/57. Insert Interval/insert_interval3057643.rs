// https://leetcode.com/problems/insert-interval/solutions/3057643/rust-buffering-interspersing-iterator-o-n-0ms/
use std::collections::VecDeque;
use std::iter::from_fn as custom_iter;

impl Solution {
    pub fn insert(intervals    : Vec<Vec<i32>>, 
                  new_interval : Vec<i32>) 
        -> Vec<Vec<i32>> 
    {
        let mut iter = intervals.into_iter();
        let mut buff = VecDeque::new();
        let mut ni   = Some(new_interval);

        custom_iter(move || {
            if !buff.is_empty() { 
                buff.pop_front() 
            } else if ni.is_none() { 
                iter.next() 
            } else if let Some(v) = iter.next() {
                let w = ni.as_mut().unwrap();
                if v[1] < w[0] {
                    Some(v)
                } else if v[0] > w[1] {
                    buff.push_back(v);
                    ni.take()
                } else {
                    w[0] = w[0].min(v[0]);
                    w[1] = w[1].max(v[1]);
                    while let Some(v) = iter.next() {
                        if v[0] <= w[1] {
                            w[1] = w[1].max(v[1]);
                        } else {
                            buff.push_back(v);
                            break;
                        }
                    }
                    ni.take()
                }
            } else { 
                ni.take()
            }
        }).collect()
    }
}