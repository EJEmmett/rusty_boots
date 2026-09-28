//! https://pdos.csail.mit.edu/6.828/2008/readings/i386/c09.htm
//! https://wiki.osdev.org/Exceptions
use x86_64::VirtAddr;

use entry::Entry;
use types::*;

use crate::interrupts::PIC_1_OFFSET;

mod entry;
mod types;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum UserInterruptIndex {
    Timer,
    Keyboard,
}

impl UserInterruptIndex {
    #[inline]
    pub fn as_offset(self) -> u8 {
        self as u8 + PIC_1_OFFSET
    }
}

#[repr(C)]
#[repr(align(16))]
pub struct InterruptDescriptorTable {
    pub divide_error: Entry,
    pub debug: Entry,
    pub non_maskable_interrupt: Entry,
    pub breakpoint: Entry,
    pub overflow: Entry,
    pub bound_range_exceeded: Entry,
    pub invalid_opcode: Entry,
    pub device_not_available: Entry,
    pub double_fault: Entry<DivergingErrCode>,
    coprocessor_segment_overrun: Entry,
    pub invalid_tss: Entry<HandlerErrCode>,
    pub segment_not_present: Entry<HandlerErrCode>,
    pub stack_segment_fault: Entry<HandlerErrCode>,
    pub general_protection_fault: Entry<HandlerErrCode>,
    pub page_fault: Entry<PageFaultHandler>,
    reserved_1: Entry,
    pub x87_floating_point: Entry,
    pub alignment_check: Entry<HandlerErrCode>,
    pub machine_check: Entry<DivergingHandler>,
    pub simd_floating_point: Entry,
    pub virtualization: Entry,
    reserved_2: [Entry; 9],
    pub security_exception: Entry<HandlerErrCode>,
    reserved_3: Entry,
    pub interrupts: [Entry; 256 - 32], // idx 32
}

impl Default for InterruptDescriptorTable {
    fn default() -> Self {
        Self::new()
    }
}

impl InterruptDescriptorTable {
    #[inline]
    pub fn new() -> Self {
        Self {
            divide_error: Entry::missing(),
            debug: Entry::missing(),
            non_maskable_interrupt: Entry::missing(),
            breakpoint: Entry::missing(),
            overflow: Entry::missing(),
            bound_range_exceeded: Entry::missing(),
            invalid_opcode: Entry::missing(),
            device_not_available: Entry::missing(),
            double_fault: Entry::missing(),
            coprocessor_segment_overrun: Entry::missing(),
            invalid_tss: Entry::missing(),
            segment_not_present: Entry::missing(),
            stack_segment_fault: Entry::missing(),
            general_protection_fault: Entry::missing(),
            page_fault: Entry::missing(),
            reserved_1: Entry::missing(),
            x87_floating_point: Entry::missing(),
            alignment_check: Entry::missing(),
            machine_check: Entry::missing(),
            simd_floating_point: Entry::missing(),
            virtualization: Entry::missing(),
            reserved_2: [Entry::missing(); 9],
            security_exception: Entry::missing(),
            reserved_3: Entry::missing(),
            interrupts: [Entry::missing(); 256 - 32],
        }
    }

    #[inline]
    pub fn load(&self) {
        use core::mem::size_of;
        use x86_64::instructions::tables::lidt;
        use x86_64::structures::DescriptorTablePointer;

        let ptr = DescriptorTablePointer {
            base: VirtAddr::from_ptr(self as *const _),
            limit: (size_of::<Self>() - 1) as u16,
        };

        unsafe { lidt(&ptr) };
    }
}
