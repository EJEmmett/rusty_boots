use core::alloc::Layout;

use crate::util::kalloc::{get_page_range, Locked};
use x86_64::structures::paging::{
    mapper::MapToError, FrameAllocator, Mapper, PageTableFlags, Size4KiB,
};
use x86_64::VirtAddr;

mod buddy_alloc;
use buddy_alloc::BuddyAlloc;

type TOpt<T> = Option<&'static mut T>;

pub const HEAP_START: usize = 0x_7777_0000_0000;
pub const HEAP_SIZE: usize = 128 * 1024; // 128 KiB

#[global_allocator]
pub static ALLOCATOR: Locked<BuddyAlloc> = Locked::new(BuddyAlloc::new());

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = get_page_range(VirtAddr::new(HEAP_START as u64), HEAP_SIZE as u64);

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;

        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() }
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("Allocation Error: {:?}", layout)
}
