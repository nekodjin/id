use crate::{
    instr::Instr,
    page::{Page, PAGE_SIZE},
};

use fxhash::{FxBuildHasher, FxHashMap as HashMap};

#[allow(non_camel_case_types)]
use num::BigUint as ubig;

pub struct Machine {
    ip: ubig,
    pages: HashMap<ubig, Page>,
}

impl Machine {
    pub fn new() -> Self {
        let mut machine = Machine {
            ip: ubig::from(0u8),
            pages: HashMap::with_hasher(FxBuildHasher::default()),
        };

        machine.pages.insert(ubig::from(0u8), Page::new());

        machine
    }

    pub fn run(&mut self, instrs: Vec<Instr>) {
        loop {
            for instr in &instrs {
                match instr {
                    Instr::I => self.i(),
                    Instr::D => self.d(),
                }
            }
        }
    }

    fn i(&mut self) {
        #[cfg(debug_assertions)]
        eprintln!("i: ip({})", self.ip);

        let page_offset: usize = unsafe {
            (&self.ip % PAGE_SIZE).try_into().unwrap_unchecked()
        };
        let page_boundary = &self.ip - page_offset;

        unsafe {
            self.pages.get_mut(&page_boundary).unwrap_unchecked()
                [page_offset] += 1u8;
        }
    }

    fn d(&mut self) {
        #[cfg(debug_assertions)]
        eprintln!("d: ip({})", self.ip);

        let page_offset: usize = unsafe {
            (&self.ip % PAGE_SIZE).try_into().unwrap_unchecked()
        };
        let page_boundary = &self.ip - page_offset;

        self.ip = self.pages[&page_boundary][page_offset].clone();

        let page_offset: usize = unsafe {
            (&self.ip % PAGE_SIZE).try_into().unwrap_unchecked()
        };
        let page_boundary = &self.ip - page_offset;

        if !self.pages.contains_key(&page_boundary) {
            #[cfg(debug_assertions)]
            eprintln!("allocating page {}", &page_boundary);

            self.pages.insert(page_boundary, Page::new());
        }
    }
}
