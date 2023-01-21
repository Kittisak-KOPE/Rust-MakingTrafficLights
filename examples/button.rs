extern crate rust_gpiozero;
use rust_gpiozero::*;

fn main() {
    // Create a button which is attached to Pin 22
    let button = Button::new(22);

    // Wait for button press
    button.wait_for_press();

    // print message
    println!("button pressed");

}