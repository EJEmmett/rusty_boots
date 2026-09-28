mod handlers;

use lazy_static::lazy_static;
use paste::paste;
use pic8259::ChainedPics;

use interrupt_descriptor_table::{InterruptDescriptorTable, UserInterruptIndex};

use crate::gdt::{DOUBLE_FAULT_IST_INDEX, OVERFLOW_IST_INDEX};
use handlers::*;

pub mod interrupt_descriptor_table;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

macro_rules! set_irq_handler {
    ($idt: ident, $name: tt) => {
        paste! {
            ($idt).[<$name>].set_handler([<$name _handler>])
        }
    };
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        set_irq_handler!(idt, "divide_error");
        set_irq_handler!(idt, "debug");
        set_irq_handler!(idt, "non_maskable_interrupt");
        set_irq_handler!(idt, "breakpoint");
        set_irq_handler!(idt, "overflow").set_stack_index(OVERFLOW_IST_INDEX);
        set_irq_handler!(idt, "bound_range_exceeded");
        set_irq_handler!(idt, "invalid_opcode");
        set_irq_handler!(idt, "device_not_available");
        set_irq_handler!(idt, "double_fault").set_stack_index(DOUBLE_FAULT_IST_INDEX);
        set_irq_handler!(idt, "invalid_tss");
        set_irq_handler!(idt, "segment_not_present");
        set_irq_handler!(idt, "stack_segment_fault");
        set_irq_handler!(idt, "general_protection_fault");
        set_irq_handler!(idt, "page_fault");
        set_irq_handler!(idt, "x87_floating_point");
        set_irq_handler!(idt, "machine_check");
        set_irq_handler!(idt, "simd_floating_point");
        set_irq_handler!(idt, "virtualization");

        idt.interrupts[UserInterruptIndex::Timer as usize].set_handler(timer_interrupt_handler);
        idt.interrupts[UserInterruptIndex::Keyboard as usize]
            .set_handler(keyboard_interrupt_handler);

        idt
    };
}

#[inline]
pub fn init() {
    IDT.load();
    unsafe {
        PICS.lock().initialize();
    }
    x86_64::instructions::interrupts::enable();
}
