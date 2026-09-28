use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

pub type Handler = extern "x86-interrupt" fn(InterruptStackFrame);
pub type HandlerErrCode = extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64);
pub type PageFaultHandler =
    extern "x86-interrupt" fn(InterruptStackFrame, error_code: PageFaultErrorCode);
pub type DivergingHandler = extern "x86-interrupt" fn(InterruptStackFrame) -> !;
pub type DivergingErrCode = extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64) -> !;
