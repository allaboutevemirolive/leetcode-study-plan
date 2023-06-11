// https://leetcode.com/problems/min-stack/solutions/1686145/unsafe-rust-solution-using-raw-pointers/
use std::{
    ptr::{ self, NonNull },
    marker::PhantomData,
    mem,
    alloc::{ self, Layout },
};


struct MinStack {
    ptr: NonNull<i32>,
    cap: usize,
    len: usize,
    _marker: PhantomData<i32>,
}


impl MinStack {
    fn new() -> Self {
        MinStack {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
            _marker: PhantomData,
        }
    }
    
    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<i32>(1).unwrap())
        } else {
            let new_cap = 2 * self.cap;
            (new_cap, Layout::array::<i32>(new_cap).unwrap())
        };
        assert!(new_layout.size() <= isize::MAX as usize, "Too large");
        
        let new_ptr = if self.cap == 0 {
            unsafe {
				alloc::alloc(new_layout)
			}
        } else {
            let old_layout = Layout::array::<i32>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe {
				alloc::realloc(old_ptr, old_layout, new_layout.size())
			}
        };
        
        self.ptr = match NonNull::new(new_ptr as *mut i32) {
            Some(p) => p,
            None    => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
    
    
    fn push(&mut self, val: i32) {
        if self.len == self.cap { self.grow(); }
        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len), val);
        }
        self.len += 1;
    }
    
    
    fn pop(&mut self) {
        if self.len != 0 {
            self.len -= 1;
        }
    }
    
    
    fn top(&self) -> i32 {
        if self.len != 0 {
            unsafe {
                ptr::read(self.ptr.as_ptr().add(self.len - 1))
            }
        } else { 0 }
    }
    
    
    fn get_min(&self) -> i32 {
        let mut min = i32::MAX;
        
        if self.len != 0 {
            for i in 0..self.len {
                unsafe {
                    let val = ptr::read(self.ptr.as_ptr().add(i));
                    if min > val { min = val; }
                }
            }
        }
        min
    }
}


unsafe impl Send for MinStack {}
unsafe impl Sync for MinStack {}


impl Drop for MinStack {
    fn drop(&mut self) {
        if self.cap != 0 {
            let layout = Layout::array::<i32>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}