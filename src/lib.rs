#![no_std]

use embedded_hal::digital::OutputPin;
use smart_leds_trait::{Color, SmartLedsWrite};
use cortex_m::asm::delay;

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
            delay(2);
            if data & bitmask != 0 {
                delay(7);
                self.pin.set_low();
            } else {
                self.pin.set_low();
                delay(2); 
            }
            bitmask >>= 1;
        }
        delay(5);
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
