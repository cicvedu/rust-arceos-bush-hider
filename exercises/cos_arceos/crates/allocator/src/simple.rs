//! Simple memory allocation.
//!
//! TODO: more efficient

use core::alloc::Layout;
use core::num::NonZeroUsize;

use crate::{AllocResult, BaseAllocator, ByteAllocator};
use super::AllocError;

pub struct SimpleByteAllocator{
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

// Bump Allocator : 跟踪分配的字节数和分配数, 线性分配内存, 只能一次释放所有内存 ;
impl SimpleByteAllocator {
    pub const fn new() -> Self {
        SimpleByteAllocator{
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    // Initialize the allocator with a free memory region.
    // This method is unsafe because the caller must ensure that the given
    // memory range is unused. Also, this method must be called only once.
    fn init(&mut self, _start: usize, _size: usize) {
        self.heap_start = _start;
        self.heap_end = _start + _size;
        self.next = _start;
    }

    // Add a free memory region to the allocator.
    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        if _start == self.heap_end {
            self.heap_end += _size;
            return Ok(());
        }
        else if _start + _size == self.heap_start {
            self.heap_start -= _size;
            if self.next == self.heap_start + _size {
                self.next = self.heap_start;
            }
            return Ok(());
        }
        Err(AllocError::InvalidParam)
    }
}

impl ByteAllocator for SimpleByteAllocator {
    // Allocate memory with the given size (in bytes) and alignment.
    fn alloc(&mut self, _layout: Layout) -> AllocResult<NonZeroUsize> {

        let alloc_start = self.next;
        let alloc_end = match alloc_start.checked_add(_layout.size()) {
            Some(end) => end,
            None => return Err(AllocError::InvalidParam),
        };

        if alloc_end > self.heap_end {
            return Err(AllocError::NoMemory);
        } 
        else {
            self.next = alloc_end;
            self.allocations += 1;
        }
        Ok(NonZeroUsize::new(alloc_start as *mut u8 as usize).unwrap())
    }

    // Deallocate memory at the given position, size, and alignment.
    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        self.allocations -= 1;
        if self.allocations == 0 {
            self.next = self.heap_start;
        }
    }

    // Returns total memory size in bytes.
    fn total_bytes(&self) -> usize {
        self.heap_end - self.heap_start
    }

    // Returns allocated memory size in bytes.
    fn used_bytes(&self) -> usize {
        self.next - self.heap_start
    }

    // Returns available memory size in bytes.
    fn available_bytes(&self) -> usize {
        self.heap_end - self.next
    }
}