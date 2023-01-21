extern crate rust_gpiozero;
use rust_gpiozero::*;

fn main() {

    // Tell the Pi which GPIO pin you are using
    let mut led = LED::new(17);

    // Make the led switch on
    // led.on();
    led.off();
}
