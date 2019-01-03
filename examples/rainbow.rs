#![feature(asm)]
#![no_std]
#![no_main]

#[macro_use(entry)]
extern crate cortex_m_rt;
extern crate cortex_m;

extern crate panic_abort;
extern crate ws2812_nop_samd21;

use circuit_playground_express::clock::GenericClockController;
use circuit_playground_express::{Peripherals, CorePeripherals};
use circuit_playground_express::delay::Delay;
use embedded_hal::blocking::delay::DelayMs;

use ws2812_nop_samd21::Ws2812;
use smart_leds_trait::SmartLedsWrite;
use smart_leds_trait::Color;
use smart_leds::brightness;

entry!(main);

fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap(); 
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut pins = circuit_playground_express::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let neopixel_pin = pins.neopixel.into_push_pull_output(&mut pins.port);
    let mut neopixel = Ws2812::new(neopixel_pin);

    const NUM_LEDS: usize = 10;
    let mut data = [Color::default(); NUM_LEDS];

    loop {
        for j in 0..(256*5) {
            for i in 0..NUM_LEDS {
                data[i] = wheel((((i * 256) as u16 / NUM_LEDS as u16 + j as u16) & 255) as u8);
            }
            neopixel.write(brightness(data.iter().cloned(), 32)).unwrap();
            delay.delay_ms(5u8);
        }
    }
}

/// Input a value 0 to 255 to get a color value
/// The colours are a transition r - g - b - back to r.
fn wheel(mut wheel_pos: u8) -> Color {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3).into()
    }
    if wheel_pos < 170 {
        wheel_pos -=85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3).into()
    }
    wheel_pos -= 170;
    (wheel_pos*3, 255 - wheel_pos * 3, 0).into()
}
