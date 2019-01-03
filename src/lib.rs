#![no_std]
#![feature(asm)]

use embedded_hal::digital::OutputPin;
use smart_leds_trait::{Color, SmartLedsWrite};

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
            self.pin.set_high();
            unsafe {
                asm!("nop; nop;");
            }
            if data & bitmask != 0 {
                unsafe { asm!("nop; nop; nop; nop; nop; nop; nop;") }
                self.pin.set_low();
            } else {
                self.pin.set_low();
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
    type Error = ();
    fn write<T>(&mut self, iterator: T) -> Result<(), ()>
    where
        T: Iterator<Item = Color>,
    {
        for item in iterator {
            self.write_byte(item.g);
            self.write_byte(item.r);
            self.write_byte(item.b);
        }
        Ok(())
    }
}
