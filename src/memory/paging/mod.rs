use bootloader::bootinfo::MemoryMap;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::structures::paging::OffsetPageTable;
use x86_64::VirtAddr;

mod bitmap_alloc;
use bitmap_alloc::BitmapFrameAlloc;

pub fn init(phys_mem_offset: VirtAddr) -> OffsetPageTable<'static> {
    use x86_64::registers::control::Cr3;

    let l4_table_frame = Cr3::read().0;

    let l4_table_phys_addr = l4_table_frame.start_address();
    let l4_table = (phys_mem_offset + l4_table_phys_addr.as_u64()).as_mut_ptr();

    unsafe {
        // Table must be in valid format
        // Phys offset must be correct
        OffsetPageTable::new(&mut *l4_table, phys_mem_offset)
    }
}

pub fn init_frame_alloc(memory_map: &'static MemoryMap, phys_mem_offset: VirtAddr) -> BitmapFrameAlloc {
    unsafe {
        // Usable regions in the memory map must be unused
        // Phys offset must be correct
        BitmapFrameAlloc::new(memory_map, phys_mem_offset)
    }
}

lazy_static! {
    pub static ref MAPPER: Mutex<Option<OffsetPageTable<'static>>> = Mutex::new(None);
    pub static ref FRAME_ALLOCATOR: Mutex<Option<BitmapFrameAlloc>> =
        Mutex::new(None);
}
