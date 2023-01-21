extern crate rust_gpiozero;
use rust_gpiozero::*;
use std::thread::sleep;
use std::time::Duration;

fn main(){
    // Create a button which is atached to Pin 22 (1)
    let button = Button::new(22);

    let mut red = LED::new(25);
    let mut amber = LED::new(8);
    let mut green = LED::new(7);

    let mut _buzzer = Buzzer::new(15);

    loop{

        button.wait_for_press();
        green.on();
        sleep(Duration::from_secs(1));
        amber.on();
        sleep(Duration::from_secs(1));
        red.on();
        sleep(Duration::from_secs(1));
        green.off();
        amber.off();
        red.off();

    }
}