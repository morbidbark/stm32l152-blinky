#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m::asm;
use cortex_m_rt::entry;
use stm32l1::stm32l151 as pac;

use blinky::led::LED;
use blinky::button::Button;

#[entry]
fn main() -> ! {
    let periphs = pac::Peripherals::take().unwrap();
    let rcc = &periphs.RCC;
    rcc.ahbenr.write(|w| {
        w.gpiopaen().set_bit().gpiopben().set_bit()
    });

    let mut led = LED::init(&periphs);
    let button = Button::init(&periphs);

    loop {
        if button.is_pressed() {
            led.set_high();
        } else {
            led.set_low();
        }
        asm::delay(1_000);
    }
}
