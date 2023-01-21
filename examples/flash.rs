extern crate rust_gpiozero;
use rust_gpiozero::*;

fn main() {

    // Tell the Pi which GPIO pin you are using
    let mut led = LED::new(17);

    // -------------------------------------------------
    // loop{
        // Make the led switch on
        // led.on();

        // Let the LED stay on for one second
        // sleep(Dration::from_secs(1));

        // Make the led switch off
        // led.off();

        // Let the LED stay off for one second
        // sleep(Duratio::from_secs(1));
    // }
    // -------------------------------------------------

    // let the LED blink indefinitely, staying on for 1 sec and off for 1 sec    
    led.blink(1,1);
}
