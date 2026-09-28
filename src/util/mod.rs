
use x86_64::instructions::tables::{sgdt, sidt};
use x86_64::structures::paging::{Mapper, Size4KiB, Translate};

pub mod exit;
pub mod kalloc;

#[inline]
pub fn hlt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[inline]
pub fn print_mem_addrs(offset_table: &mut (impl Mapper<Size4KiB> + Translate)) {
    let gdt_virt = sgdt().base;
    let gdt_phys = offset_table.translate_addr(gdt_virt).unwrap();
    let idt_virt = sidt().base;
    let idt_phys = offset_table.translate_addr(idt_virt).unwrap();
    println!("GDT: Virt({:p}) -> Phys({:p})", gdt_virt, gdt_phys);
    println!("IDT: Virt({:p}) -> Phys({:p})", idt_virt, idt_phys);
}

/// Reads a byte from an I/O port.
///
/// # Safety
/// Reading a port can have side effects on the device behind it; the caller must
/// ensure `port` is valid to read and that the read doesn't break other users of it.
pub unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack));
    value
}

/// Writes a byte to an I/O port.
///
/// # Safety
/// Writing a port can reconfigure hardware; the caller must ensure `port` and
/// `value` are valid for the device and won't violate memory safety.
pub unsafe fn outb(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack))
}