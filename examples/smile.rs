#![feature(asm)]
#![no_std]
#![no_main]

extern crate circuit_playground_express;

#[macro_use(entry)]
extern crate cortex_m_rt;
extern crate cortex_m;

extern crate panic_abort;
extern crate ws2812_nop_samd21;

use circuit_playground_express::clock::GenericClockController;
use circuit_playground_express::{Peripherals};

use ws2812_nop_samd21::Ws2812;
use smart_leds_trait::SmartLedsWrite;
use smart_leds_trait::Color;

entry!(main);

fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
     
    let _clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut pins = circuit_playground_express::Pins::new(peripherals.PORT);
    let neopixel_pin = pins.neopixel.into_push_pull_output(&mut pins.port);
    let mut neopixel = Ws2812::new(neopixel_pin);

    let yellow = Color {r: 0x01, g: 0x01, b: 0x00};
    let off = Color {r: 0x00, g: 0x00, b: 0x00};
    let smile = [yellow, off, yellow, yellow, yellow, yellow, yellow, yellow, off, yellow];  
    neopixel.write(smile.iter().cloned()).unwrap();
    loop {}
}
