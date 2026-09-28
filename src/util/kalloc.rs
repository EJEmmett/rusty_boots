use x86_64::structures::paging::{Page, page::PageRangeInclusive};
use x86_64::VirtAddr;
use spin::mutex::Mutex;
use spin::MutexGuard;

#[inline]
pub fn get_page_range(start: VirtAddr, limit: u64) -> PageRangeInclusive {
    let end = start + limit - 1u64;
    let start_page = Page::containing_address(start);
    let end_page = Page::containing_address(end);
    Page::range_inclusive(start_page, end_page)
}

#[inline]
pub fn get_page_range_usize(start: usize, end: usize) -> PageRangeInclusive {
    let start = VirtAddr::new(start as u64);
    let end = VirtAddr::new(end as u64);

    let start_page = Page::containing_address(start);
    let end_page = Page::containing_address(end);
    Page::range_inclusive(start_page, end_page)
}

pub struct Locked<T> {
    inner: Mutex<T>,
}

impl<T> Locked<T> {
    pub const fn new(inner: T) -> Self {
        Self {
            inner: Mutex::new(inner),
        }
    }

    #[inline]
    pub fn lock(&self) -> MutexGuard<T> {
        self.inner.lock()
    }
}
