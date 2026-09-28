//! https://www.amd.com/system/files/TechDocs/24593.pdf
#![no_std]
#![feature(
    abi_x86_interrupt,
    alloc_error_handler,
    char_internals,
    const_mut_refs,
    const_panic,
    stmt_expr_attributes,
    unicode_internals
)]
#![feature(asm)]
#![allow(dead_code, unconditional_recursion)]
extern crate alloc;

use bootloader::BootInfo;
use x86_64::instructions::tables::{sgdt, sidt};
use x86_64::VirtAddr;

#[macro_use]
pub mod vga;
#[macro_use]
pub mod serial;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod multitasking;
pub mod showcases;
pub mod util;
pub mod logger;
pub mod time;

pub static mut TIME_SINCE_BOOT: u32 = 0;
pub static mut BOOT_ENTRY_TICK: f64 = 0.0;

#[inline]
pub fn init(boot_info: &'static BootInfo) {
    println!("Boot Time: {}", time::cmos::date());

    gdt::init();
    let gdt_ptr = sgdt();
    let (gdt_base, gdt_limit) = (gdt_ptr.base, gdt_ptr.limit);
    println!("GDT: {:p} -> {:p}", gdt_base, gdt_base + gdt_limit as u64);
    serial_println!("Loaded GDT");
    interrupts::init();
    let idt_ptr = sidt();
    let (idt_base, idt_limit) = (idt_ptr.base, idt_ptr.limit);
    println!("IDT: {:p} -> {:p}", idt_base, idt_base + idt_limit as u64);
    serial_println!("Loaded IDT");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut offset_table = memory::paging::init(phys_mem_offset);
    serial_println!("Created Page Table");

    let mut frame_allocator = memory::paging::init_frame_alloc(&boot_info.memory_map, phys_mem_offset);
    serial_println!("Created Frame Allocator");

    memory::kalloc::init_heap(&mut offset_table, &mut frame_allocator)
        .expect("Heap Initialization Failed!");
    serial_println!("Initialized Heap");

    *memory::paging::MAPPER.lock() = Some(offset_table);
    *memory::paging::FRAME_ALLOCATOR.lock() = Some(frame_allocator);
}
