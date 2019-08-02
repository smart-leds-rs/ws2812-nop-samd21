#![no_std]
#![feature(asm)]

use embedded_hal::digital::v2::OutputPin;
use smart_leds_trait::{RGB8, SmartLedsWrite};

pub struct Ws2812<P: OutputPin> {
    pin: P,
}

impl<P: OutputPin> Ws2812<P> {
    pub fn new(pin: P) -> Ws2812<P> {
        Ws2812 { pin: pin }
    }
    fn write_byte(&mut self, data: u8) {
        let mut bitmask: u8 = 0x80;
        while bitmask != 0 {
            self.pin.set_high().unwrap_or(());
            unsafe {
                asm!("nop; nop;");
            }
            if data & bitmask != 0 {
                unsafe { asm!("nop; nop; nop; nop; nop; nop; nop;") }
                self.pin.set_low().unwrap_or(());
            } else {
                self.pin.set_low().unwrap_or(());
                unsafe {
                    asm!("nop; nop;");
                }
            }
            bitmask >>= 1;
        }
        unsafe {
            asm!("nop; nop; nop; nop; nop;");
        }
    }
}

impl<P> SmartLedsWrite for Ws2812<P>
where
    P: OutputPin,
{
    type Color = RGB8;
    type Error = ();
    fn write<T, I>(&mut self, iterator: T) -> Result<(), ()>
    where
        T: Iterator<Item = I>,
        I: Into<Self::Color>
    {
        for item in iterator {
            let color: Self::Color = item.into();
            self.write_byte(color.g);
            self.write_byte(color.r);
            self.write_byte(color.b);
        }
        Ok(())
    }
}
