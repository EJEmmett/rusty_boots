#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(stmt_expr_attributes)]
#![feature(array_chunks)]
#![feature(asm)]
#![allow(dead_code)]

extern crate alloc;

use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};

use rusty_boots::util;
use rusty_boots::{cprint, println};
use rusty_boots::memory::paging::{FRAME_ALLOCATOR, MAPPER};
use rusty_boots::multitasking::task::{executor::Executor, keyboard, Task};
use rusty_boots::vga::color_code::ColorCode;

entry_point!(k_main);

fn k_main(boot_info: &'static BootInfo) -> ! {
    rusty_boots::init(boot_info);
    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));

    // {
    //     let mut offset_table = MAPPER.lock();
    //     util::print_mem_addrs(offset_table.as_mut().unwrap());
    //     println!();
    // }

    // use rusty_boots::showcases;
    // // Execution ends after these fns
    // // showcases::double_fault();
    // // showcases::page_fault();
    // {
    //     showcases::vga_buffer();
    //     println!();
    // }
    // {
    //     let mut offset_table = MAPPER.lock();
    //     let mut frame_allocator = FRAME_ALLOCATOR.lock();
    //     showcases::virt_addr_translation(boot_info, offset_table.as_mut().unwrap());
    //     println!();
    //     showcases::page_creation(
    //         offset_table.as_mut().unwrap(),
    //         frame_allocator.as_mut().unwrap(),
    //     )
    //     .expect("Page Creation Failed!");
    //     println!();
    // }
    // {
    //     showcases::heap();
    //     println!();
    // }
    // {
    //     let test_task = showcases::task();
    //     executor.spawn(test_task)
    // }
    executor.run()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cprint!(ColorCode::YELLOW, "{}", info);
    util::hlt()
}
