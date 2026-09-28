use alloc::format;
use alloc::string::String;

use bootloader::BootInfo;
use x86_64::instructions::{hlt, interrupts};
use x86_64::structures::paging::mapper::MapToError;
use x86_64::structures::paging::{
    FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB, Translate,
};
use x86_64::VirtAddr;

use crate::multitasking::task::Task;
use crate::vga::color_code::ColorCode;
use crate::vga::writer::WRITER;

#[allow(unconditional_panic)]
pub fn vga_buffer() {
    println!("Macro Usage");

    interrupts::without_interrupts(|| {
        WRITER.lock().write_string("Direct Usage\n");
    });

    let color = color!("red", "green");
    cprint!(color, "Colored Text!");
    println!();
}

pub fn double_fault() {
    fn overflow() {
        overflow();
    }
    overflow()
}

pub fn page_fault() {
    unsafe {
        *(0xdeadbeef as *mut u64) = 0;
    }
}

pub fn virt_addr_translation(
    boot_info: &'static BootInfo,
    mapper: &mut (impl Mapper<Size4KiB> + Translate),
) {
    let addresses = [
        ("Boot Info Struct", (boot_info as *const _) as u64),
        ("VGA Buffer", 0xb8000),
        ("Virt Mem Map", boot_info.physical_memory_offset),
    ];

    for (rev, addr) in &addresses {
        let virt_addr = VirtAddr::new(*addr);
        let result = mapper.translate_addr(virt_addr);
        println!("{}: {:?} -> {:?}", rev, virt_addr, result.unwrap());
    }
}

pub fn page_creation(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let virt_addr = VirtAddr::new(0x0CAF_EBAB_E000);
    let page = Page::containing_address(virt_addr);
    let frame = frame_allocator
        .allocate_frame()
        .ok_or(MapToError::FrameAllocationFailed)?;
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() }

    println!("{:?} -> {:?}", page, frame);
    let ptr: *mut &str = page.start_address().as_mut_ptr();

    let s = format!("Help! Im trapped at {:p}", ptr as *const _);

    unsafe {
        ptr.write(s.as_str());
        println!("Wrote: {}", ptr.read());
    }

    let (frame, result) = mapper.unmap(page).expect("Failed to Unmap");
    result.flush();

    // let ptr: *mut &str = page.start_address().as_mut_ptr();
    // unsafe {
    //     println!("{}", ptr.read());
    // }

    let page = Page::containing_address(VirtAddr::new(0x0FEE_DBEE_F000));

    unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() }

    println!("{:?} -> {:?}", page, frame);

    let ptr: *mut &str = page.start_address().as_mut_ptr();
    unsafe {
        println!("Read: {}", ptr.read());
    }

    Ok(())
}

pub fn heap() {
    use crate::memory::kalloc::HEAP_START;
    use alloc::boxed::Box;
    use alloc::vec;
    let heap_box = Box::new(10);
    let heap_string = String::from("Im a string");
    let mut heap_vec = vec![1, 2, 3];

    println!("HEAP_START: 0x{:x}", HEAP_START);

    cprint!(ColorCode::YELLOW, "Box: {} at ", heap_box);
    cprintln!(
        ColorCode::RED,
        "{:p}",
        (Box::into_raw(heap_box) as *const _)
    );

    cprint!(ColorCode::YELLOW, "String: \"{}\" at ", heap_string);
    cprintln!(ColorCode::RED, "{:p}", (heap_string.as_ptr()));

    cprint!(ColorCode::YELLOW, "Vec: {:?} at ", heap_vec);
    cprintln!(ColorCode::RED, "{:p}", (heap_vec.as_ptr()));

    heap_vec.push(99);
    cprint!(ColorCode::YELLOW, "Updated Vec: {:?} at ", heap_vec);
    cprintln!(ColorCode::RED, "{:p}", (heap_vec.as_ptr()));
}

pub fn task() -> Task {
    use crate::TIME_SINCE_BOOT;

    async fn outer() {
        async fn inner() -> u32 {
            let t = unsafe { TIME_SINCE_BOOT };
            while unsafe { TIME_SINCE_BOOT } < t + 30 {
                hlt()
            }

            42
        }

        let number = inner();
        println!("Im waiting for a bit.");
        let num = number.await;
        print!("The number I waited for is ");
        cprint!(ColorCode::YELLOW, "{}", (num));
    }

    Task::new(outer())
}
