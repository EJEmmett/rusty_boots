use core::alloc::{GlobalAlloc, Layout};
use core::cmp::{max, min};
use core::ptr::null_mut;

use crate::memory::kalloc::TOpt;
use crate::util::kalloc::Locked;

const MIN_BLOCK: usize = 16;
const ORDERS: usize = calculate_order_count(super::HEAP_SIZE);

const fn calculate_order_count (heap_size: usize) -> usize {
    assert!(heap_size.is_power_of_two());
    (heap_size.trailing_zeros() - MIN_BLOCK.trailing_zeros()) as usize + 1
}


struct FreeBlock {
    next: TOpt<FreeBlock>
}

impl FreeBlock {
    fn get_addr(&self) -> usize {
        self as *const Self as usize
    }
}

pub struct BuddyAlloc {
    heap_start: usize,
    free_lists: [TOpt<FreeBlock>; ORDERS],
}

impl BuddyAlloc {
    const EMPTY: TOpt<FreeBlock> = None;

    pub const fn new() -> Self {
        Self { heap_start: 0, free_lists: [Self::EMPTY; ORDERS] }

    }
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        assert!(heap_size == MIN_BLOCK << (ORDERS - 1));
        assert!(heap_start % heap_size == 0);
        self.heap_start = heap_start;
        self.push(ORDERS - 1, heap_start)
    }
    fn order_for(layout: Layout) -> Option<usize> {
        let size = max(max(layout.size(), layout.align()), MIN_BLOCK).next_power_of_two();
        let order = (size.trailing_zeros() - MIN_BLOCK.trailing_zeros()) as usize;
        if order < ORDERS {
            Some(order)
        } else {
            None
        }
    }
    unsafe fn push(&mut self, order: usize, addr: usize) {
        assert!(order < ORDERS);
        assert!(addr >= self.heap_start);

        let free_block = FreeBlock { next: self.free_lists[order].take() };

        let free_block_ptr = addr as *mut FreeBlock;
        free_block_ptr.write(free_block);

        self.free_lists[order] = Some(&mut *free_block_ptr)
    }
    fn pop(&mut self, order: usize) -> Option<usize> {
        assert!(order < ORDERS);

        let free_block = self.free_lists[order].take()?;
        self.free_lists[order] = free_block.next.take();

        Some(free_block as *mut FreeBlock as usize)
    }
    fn remove(&mut self, order: usize, addr: usize) -> bool {
        assert!(order < ORDERS);
        assert!(addr >= self.heap_start);

        let mut current = &mut self.free_lists[order];

        while current.is_some() {
            if current.as_ref().unwrap().get_addr() == addr {
                let found = current.take().unwrap();
                *current = found.next.take();
                return true;
            }
            current = &mut current.as_mut().unwrap().next
        }

        false
    }
    fn buddy_of(&self, order: usize, addr: usize) -> usize {
        let block_size = MIN_BLOCK << order;
        let offset = addr - self.heap_start;

        assert!(order < ORDERS - 1);
        assert!(offset % block_size == 0);

        let buddy_offset = offset ^ block_size;
        self.heap_start + buddy_offset
    }
    unsafe fn alloc_order(&mut self, order: usize) -> Option<usize> {
        assert!(order < ORDERS);

        let mut j = (order..ORDERS).find(|&j| self.free_lists[j].is_some())?;

        let addr = self.pop(j)?;
        while j > order {
            j -= 1;
            self.push(j, addr + (MIN_BLOCK << j));
        }

        Some(addr)
    }
    unsafe fn free_order(&mut self, order: usize, addr: usize) {
        assert!(order < ORDERS);
        assert!(addr >= self.heap_start);

        let mut order = order;
        let mut addr = addr;

        while order < ORDERS - 1 {
            let buddy = self.buddy_of(order, addr);
            if !self.remove(order, buddy) {
                break;
            }
            addr = min(addr, buddy);
            order += 1;
        }

        self.push(order, addr)
    }
 }

unsafe impl GlobalAlloc for Locked<BuddyAlloc> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let order = match BuddyAlloc::order_for(layout) {
            Some(order) => order,
            None => return null_mut(),
        };

        match self.lock().alloc_order(order) {
            Some(addr) => addr as *mut u8,
            None => null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let order = BuddyAlloc::order_for(layout).unwrap();
        self.lock().free_order(order, ptr as usize)
    }
}
