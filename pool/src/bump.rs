pub struct BumpUp {
    start: *mut u8,
    end: *mut u8,
    ptr: *mut u8,
}

macro_rules! try_null {
    ($e: expr) => {
        match $e {
            None => return std::ptr::null_mut(),
            Some(e) => e,
        }
    };
}

impl BumpUp {
    pub unsafe fn new(start: *mut u8, end: *mut u8) -> BumpUp {
        assert!(start as usize <= end as usize);
        let ptr = start;
        BumpUp { start, end, ptr }
    }

    pub unsafe fn alloc(&mut self, size: usize, align: usize) -> *mut u8 {
        debug_assert!(align > 0);
        debug_assert!(align.is_power_of_two());

        let ptr = self.ptr as usize;
        let aligned = try_null!(ptr.checked_add(align - 1)) & !(align - 1);
        let new_ptr = try_null!(aligned.checked_add(size));
        let end = self.end as usize;
        if new_ptr > end {
            return std::ptr::null_mut();
        }
        self.ptr = new_ptr as *mut u8;
        aligned as *mut u8
    }
}

#[repr(C)]
pub struct BumpDown {
    start: *mut u8,
    end: *mut u8,
    ptr: *mut u8,
}

impl BumpDown {
    pub unsafe fn new(start: *mut u8, end: *mut u8) -> BumpDown {
        assert!(start as usize <= end as usize);
        let ptr = end;
        BumpDown { start, end, ptr }
    }

    pub unsafe fn alloc(&mut self, size: usize, align: usize) -> *mut u8 {
        debug_assert!(align > 0);
        debug_assert!(align.is_power_of_two());
        let ptr = self.ptr as usize;
        let new_ptr = try_null!(ptr.checked_add(size));

        let new_ptr = new_ptr & !(align - 1);
        let start = self.start as usize;
        if new_ptr < start {
            return std::ptr::null_mut();
        }
        self.ptr = new_ptr as *mut u8;
        self.ptr
    }
}
