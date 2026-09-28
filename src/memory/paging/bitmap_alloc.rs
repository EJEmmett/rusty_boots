use core::cmp::min;

use bootloader::bootinfo::{MemoryMap, MemoryRegion, MemoryRegionType};
use x86_64::structures::paging::{FrameAllocator, FrameDeallocator, PageSize, PhysFrame, Size4KiB};
use x86_64::{PhysAddr, VirtAddr};


const FRAME_SIZE: u64 = Size4KiB::SIZE;   // 4096


fn usable_regions(memory_map: &'static MemoryMap) -> impl Iterator<Item = &'static MemoryRegion> {
    memory_map.iter().filter(|r| r.region_type == MemoryRegionType::Usable)
}

pub struct BitmapFrameAlloc {
    bitmap: &'static mut [u64],
    frame_count: usize,
    next_word: usize,
}

impl BitmapFrameAlloc {
    pub unsafe fn new(memory_map: &'static MemoryMap, phys_mem_offset: VirtAddr) -> Self {
        let highest_addr = usable_regions(memory_map).map(|r| r.range.end_addr()).max().expect("no usable memory");

        let frame_count: usize = (highest_addr / FRAME_SIZE) as usize;
        let words = (frame_count + 63) / 64;
        let bitmap_bytes = words * 8;

        let bitmap_region = usable_regions(memory_map).find(|r| r.range.end_addr() - r.range.start_addr() >= bitmap_bytes as u64).expect("no region big enough for the bitmap");

        let bitmap_phys = bitmap_region.range.start_addr();
        let ptr = (phys_mem_offset + bitmap_phys).as_mut_ptr::<u64>();
        let bitmap = core::slice::from_raw_parts_mut(ptr, words);

        let mut alloc = Self {
            bitmap,
            frame_count,
            next_word: 0,
        };

        alloc.bitmap.fill(u64::MAX);
        for region in usable_regions(memory_map) {
            for frame in region.range.start_addr() / FRAME_SIZE .. region.range.end_addr() / FRAME_SIZE {
                alloc.set_free(frame as usize);
            }
        }

        let bitmap_end = bitmap_phys + bitmap_bytes as u64;
        for frame in bitmap_phys / FRAME_SIZE .. (bitmap_end + FRAME_SIZE - 1) / FRAME_SIZE {
            alloc.set_used(frame as usize);
        }

        alloc
    }
    fn set_used(&mut self, frame: usize) {
        let word = frame / 64;
        let mask = 1u64 << (frame % 64);

        self.bitmap[word] |= mask
    }
    fn set_free(&mut self, frame: usize) {
        let word = frame / 64;
        let mask = 1u64 << (frame % 64);

        self.bitmap[word] &= !mask
    }
    fn is_used(&self, frame: usize) -> bool {
        let word = frame / 64;
        let mask = 1u64 << (frame % 64);

        self.bitmap[word] & mask != 0
    }
    pub fn free_count(&self) -> usize {
        self.bitmap.iter().map(|w| w.count_zeros() as usize).sum()
    }
}

unsafe impl FrameAllocator<Size4KiB> for BitmapFrameAlloc {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let words = (self.frame_count + 63) / 64;

        for n in 0..words {
            let idx = (self.next_word + n) % words;
            let word = self.bitmap[idx];
            if word == u64::MAX {
                continue
            }

            let lowest_bit = (!word).trailing_zeros();
            self.next_word = idx;

            let frame = idx * 64 + lowest_bit as usize;
            self.set_used(frame);

            return Some(PhysFrame::containing_address(PhysAddr::new(frame as u64 * FRAME_SIZE)));
        }

        None
    }
}
impl FrameDeallocator<Size4KiB> for BitmapFrameAlloc {
    unsafe fn deallocate_frame(&mut self, frame: PhysFrame) {
        let frame_num = (frame.start_address().as_u64() / FRAME_SIZE) as usize;
        assert!(self.is_used(frame_num));
        self.set_free(frame_num);
        self.next_word = min(self.next_word, frame_num / 64)
    }
}
