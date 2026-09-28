use core::marker::PhantomData;

use bit_field::BitField;
use x86_64::registers::segmentation::{Segment, SegmentSelector, CS};
use x86_64::PrivilegeLevel;

use crate::interrupts::interrupt_descriptor_table::types::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntryOpts(u16);

impl Default for EntryOpts {
    fn default() -> Self {
        Self::new()
    }
}

impl EntryOpts {
    /*
       0-2   IST Index                   (Swap to nth stack in IST)
       3-7   Reserved
       8     Toggle Interrupts
       9-11  Must be one
       12    Must be Zero
       13-14 Descriptor Privilege Level (Min Privilege Level)
       15    Present                     (Entry is present)
    */

    #[inline]
    pub fn new() -> Self {
        let mut opts = Self(0b1110_0000_0000);
        opts.set_present(true).disable_interrupts(true);
        opts
    }

    #[inline]
    pub fn missing() -> Self {
        Self(0b1110_0000_0000)
    }

    #[inline]
    pub fn set_stack_index(&mut self, index: u16) -> &mut Self {
        self.0.set_bits(0..3, index + 1);
        self
    }

    #[inline]
    pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        self.0.set_bit(8, !disable);
        self
    }

    #[inline]
    pub fn set_privilege_level(&mut self, dpl: PrivilegeLevel) -> &mut Self {
        self.0.set_bits(13..15, dpl as u16);
        self
    }

    #[inline]
    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.0.set_bit(15, present);
        self
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Entry<F = Handler> {
    fpointer_low: u16,
    gdt_selector: SegmentSelector,
    options: EntryOpts,
    fpointer_middle: u16,
    fpointer_high: u32,
    reserved: u32,
    phantom: PhantomData<F>,
}

impl<F> Entry<F> {
    #[inline]
    pub fn missing() -> Self {
        Self {
            gdt_selector: SegmentSelector::new(0, PrivilegeLevel::Ring0),
            fpointer_low: 0,
            fpointer_middle: 0,
            fpointer_high: 0,
            options: EntryOpts::missing(),
            reserved: 0,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn set_handler_addr(&mut self, addr: u64) -> &mut EntryOpts {
        self.fpointer_low = addr as u16;
        self.fpointer_middle = (addr >> 16) as u16;
        self.fpointer_high = (addr >> 32) as u32;

        self.gdt_selector = CS::get_reg();

        self.options.set_present(true).disable_interrupts(true)
    }
}

macro_rules! generate_set_handler {
    ($h:ty) => {
        impl Entry<$h> {
            #[inline]
            pub fn set_handler(&mut self, handler: $h) -> &mut EntryOpts {
                self.set_handler_addr(handler as u64)
            }
        }
    };
}

generate_set_handler!(Handler);
generate_set_handler!(HandlerErrCode);
generate_set_handler!(DivergingHandler);
generate_set_handler!(DivergingErrCode);
generate_set_handler!(PageFaultHandler);
