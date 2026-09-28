use core::ops::{Deref, DerefMut};
use crate::vga::color_code::ColorCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct DisplayChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}

impl DisplayChar {
    #[inline]
    pub fn new(ascii_character: u8, color_code: ColorCode) -> Self {
        Self {
            ascii_character,
            color_code,
        }
    }
}

// Deref/Mut required for Volatile Writes
impl Deref for DisplayChar {
    type Target = Self;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self
    }
}

impl DerefMut for DisplayChar {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}