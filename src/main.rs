#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);
    // let delay = arduino_hal::Delay::new();

    let mut led = pins.d13.into_output();

    loop {
        arduino_hal::delay_ms(1000);
        led.toggle();
    }
}
