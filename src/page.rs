use std::ops;

use num::BigUint;

pub const PAGE_SIZE: usize = 0x10000;

pub struct Page(Box<[BigUint; PAGE_SIZE]>);

impl Page {
    pub fn new() -> Self {
        let mut page = Vec::with_capacity(PAGE_SIZE);

        unsafe {
            page.set_len(PAGE_SIZE);
        }

        for i in 0..PAGE_SIZE {
            page[i] = BigUint::from(0u8);
        }

        unsafe {
            Page(Box::from_raw(Box::into_raw(page.into_boxed_slice())
                as *mut [_; PAGE_SIZE]))
        }
    }
}

impl ops::Index<usize> for Page {
    type Output = BigUint;

    fn index(&self, idx: usize) -> &BigUint {
        &self.0[idx]
    }
}

impl ops::IndexMut<usize> for Page {
    fn index_mut(&mut self, idx: usize) -> &mut BigUint {
        &mut self.0[idx]
    }
}
