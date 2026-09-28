use core::fmt;
use core::fmt::Formatter;
use core::hint::spin_loop;

use bit_field::BitField;
use x86_64::instructions::port::Port;

#[repr(u8)]
enum Register {
    Second = 0x00,
    Minute = 0x02,
    Hour = 0x04,
    Day = 0x07,
    Month = 0x08,
    Year = 0x09,
    A = 0x0A,
    B = 0x0B,
    C = 0x0C,
}

#[derive(Debug, PartialEq)]
pub struct RTC {
    second: u8,
    minute: u8,
    hour: u8,
    day: u8,
    month: u8,
    year: u16,
}

impl fmt::Display for RTC {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}:{:0>2} {:0>2}/{:0>2}/{:0>2}", self.hour, self.minute, self.second, self.month, self.day, self.year)
    }
}

pub struct CMOS {
    addr: Port<u8>,
    data: Port<u8>,
}

impl Default for CMOS {
    fn default() -> Self {
        Self::new()
    }
}

impl CMOS {
    pub fn new() -> Self {
        Self {
            addr: Port::new(0x70),
            data: Port::new(0x71),
        }
    }

    fn rtc_check(&mut self) -> RTC {
        RTC {
            second: self.read(Register::Second),
            minute: self.read(Register::Minute),
            hour: self.read(Register::Hour),
            day: self.read(Register::Day),
            month: self.read(Register::Month),
            year: self.read(Register::Year) as u16,
        }
    }

    pub fn rtc(&mut self) -> RTC {
        let mut rtc = self.update_check();

        let bcd = self.read(Register::B);

        if bcd & 0x04 == 0 {
            rtc.second = (rtc.second & 0x0F) + ((rtc.second / 16) * 10);
            rtc.minute = (rtc.minute & 0x0F) + ((rtc.minute / 16) * 10);
            rtc.hour = ((rtc.hour & 0x0F) + (((rtc.hour & 0x70) / 16) * 10)) | (rtc.hour & 0x80);
            rtc.day = (rtc.day & 0x0F) + ((rtc.day / 16) * 10);
            rtc.month = (rtc.month & 0x0F) + ((rtc.month / 16) * 10);
            rtc.year = (rtc.year & 0x0F) + ((rtc.year / 16) * 10);
        }

        if (bcd & 0x02 == 0) && (rtc.hour & 0x80 == 0) {
            rtc.hour = ((rtc.hour & 0x7f) + 12) % 24
        }

        rtc.year += 2000;

        rtc
    }

    fn update_check(&mut self) -> RTC {
        loop {
            let rtc = self.consistency_check();
            if rtc == self.consistency_check() {
                return rtc
            }
        }
    }

    fn consistency_check(&mut self) -> RTC {
        while self.is_updating() {
            spin_loop()
        }
        self.rtc_check()
    }

    fn is_updating(&mut self) -> bool {
        unsafe {
            self.addr.write(Register::A as u8);
            self.data.read().get_bit(7)
        }
    }

    fn read(&mut self, reg: Register) -> u8 {
        unsafe {
            self.addr.write(reg as u8);
            self.data.read()
        }
    }
}

pub fn date() -> RTC {
    CMOS::new().rtc()
}
