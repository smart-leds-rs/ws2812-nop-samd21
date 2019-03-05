# ws2812-nop-samd21
Nop-based bitbanger for 48MHz samd21 devices. This is a direct port of Adafruit's main neopixel_write library, so it should be the most compatible with their samd21 boards, but also most "hacky" ws2812 driver. It currently uses raw assembly "nop" instructions for delays, and in future we might move to other assembly instructions that provide 1 cycle of delay.

It requires a nightly compiler to use `#![feature(asm)]`, so this crate only works on nightly rust.

![cpx rainbow cycle](https://media.giphy.com/media/EPYzBKymtpVK2j6b4B/giphy.gif)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
