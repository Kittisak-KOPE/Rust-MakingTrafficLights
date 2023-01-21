extern crate rust_gpiozero;
use rust_gpiozero::*;
use std::thread::sleep;
use std::time::Duration;

fn main(){

    // Tell the Pi whch GPIO pin you are using
    let mut led = LED::new(17);

    // Create a button which is attached to Pin 22
    let button = Button::new(22);

    button.wait_for_press();
    led.on();
    sleep(Duration::from_secs(3));
    led.off();
}