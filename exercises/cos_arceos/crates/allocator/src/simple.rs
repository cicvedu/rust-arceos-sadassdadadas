//! Simple memory allocation.
//!
//! TODO: more efficient
#![allow(dead_code)]

use core::alloc::Layout;
use core::num::NonZeroUsize;

use crate::{AllocResult, BaseAllocator, ByteAllocator};

pub struct SimpleByteAllocator {
    start: usize,
    size: usize,
    next: usize,
    allocations: usize,
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            start: 0,
            size: 0,
            next: 0,
            allocations: 0,
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, _start: usize, _size: usize) {
        //todo!();
        self.start = _start;
        self.size = _size;
        self.next = _start;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        todo!();
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, _layout: Layout) -> AllocResult<NonZeroUsize> {
        //todo!();
        let size = _layout.size();

        if size > (self.size - (self.next - self.start)) {
            Err(crate::AllocError::NoMemory)
        } else {
            let curr_ptr = self.next;
            self.next = self.next + size;
            self.allocations += 1;
            Ok(NonZeroUsize::new(curr_ptr).unwrap())
        }
    }

    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        //todo!();
        self.allocations -= 1;
    }

    fn total_bytes(&self) -> usize {
        //todo!();
        self.size
    }

    fn used_bytes(&self) -> usize {
        //todo!();
        self.next
    }

    fn available_bytes(&self) -> usize {
        //todo!();
        self.size - (self.next - self.start)
    }
}