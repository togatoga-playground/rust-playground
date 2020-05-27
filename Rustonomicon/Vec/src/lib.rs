use std::{
    alloc::{alloc, realloc, Layout},
    mem,
    ptr::NonNull,
};
#[derive(Debug)]
pub struct MyVec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");
        MyVec {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }
    fn grow(&mut self) {
        //eprintln!("{:?} {} {}", self.ptr, self.cap, self.len);
        unsafe {
            let align = mem::align_of::<T>();
            let elem_size = mem::size_of::<T>();
            let (new_cap, ptr) = if self.cap == 0 {
                let layout = Layout::new::<T>();
                let ptr = alloc(layout);
                (1, ptr)
            } else {
                let new_cap = 2 * self.cap;
                let layout = Layout::from_size_align_unchecked(elem_size * self.cap, align);
                let ptr = realloc(self.ptr.as_ptr() as *mut _, layout, new_cap * elem_size);
                (new_cap, ptr)
            };
            if ptr.is_null() {
                panic!("OOM");
            }
            self.ptr = NonNull::new_unchecked(ptr as *mut _);
            self.cap = new_cap;
        }
    }
    pub fn push(&mut self, elem: T) {
        if self.len == self.cap {
            self.grow();
        }
        unsafe {
            let ptr_last = self.ptr.as_ptr().offset(self.len as isize);
            std::ptr::write(ptr_last, elem);
        }
        self.len += 1;
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                let ptr_last = self.ptr.as_ptr().offset(self.len as isize);

                Some(std::ptr::read(ptr_last))
            }
        }
    }
}
