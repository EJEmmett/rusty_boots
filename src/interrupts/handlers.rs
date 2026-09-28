use paste::paste;
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

use crate::{interrupts::{UserInterruptIndex, PICS}, TIME_SINCE_BOOT};

macro_rules! generate_interrupt_handler {
    ($name: tt) => {
        paste! {
            pub extern "x86-interrupt" fn [<$name _handler>](stack_frame: InterruptStackFrame) {
                panic!("{}\n{:#?}", ($name).to_uppercase(), stack_frame);
            }
        }
    };
}

macro_rules! generate_interrupt_error_code_handler {
    ($name: tt) => {
        paste!{
            pub extern "x86-interrupt" fn [<$name _handler>](stack_frame: InterruptStackFrame, error_code: u64) {
                panic!("{}Error Code: {}\n{:#?}", ($name).to_uppercase(), error_code, stack_frame);
            }
        }
    };
}

generate_interrupt_handler!("divide_error");
generate_interrupt_handler!("debug");
generate_interrupt_handler!("non_maskable_interrupt");
generate_interrupt_handler!("breakpoint");
generate_interrupt_handler!("overflow");
generate_interrupt_handler!("bound_range_exceeded");
generate_interrupt_handler!("invalid_opcode");
generate_interrupt_handler!("device_not_available");

pub extern "x86-interrupt" fn double_fault_handler(
    interrupt_stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("DOUBLE FAULT\n{:#?}", interrupt_stack_frame);
}

generate_interrupt_error_code_handler!("invalid_tss");
generate_interrupt_error_code_handler!("segment_not_present");
generate_interrupt_error_code_handler!("stack_segment_fault");
generate_interrupt_error_code_handler!("general_protection_fault");

pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    panic!(
        "PAGE FAULT {:?} at {:?}\n{:#?}",
        error_code,
        Cr2::read(),
        stack_frame
    );
}

generate_interrupt_handler!("x87_floating_point");

pub extern "x86-interrupt" fn machine_check_handler(
    interrupt_stack_frame: InterruptStackFrame,
) -> ! {
    panic!("MACHINE CHECK\n{:#?}", interrupt_stack_frame);
}

generate_interrupt_handler!("simd_floating_point");
generate_interrupt_handler!("virtualization");

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        TIME_SINCE_BOOT += 1;
        PICS.lock()
            .notify_end_of_interrupt(UserInterruptIndex::Timer.as_offset());
    }
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use crate::multitasking::task::keyboard;
    use x86_64::instructions::port::Port;
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    keyboard::add_scancode(scancode);

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(UserInterruptIndex::Keyboard.as_offset());
    }
}
