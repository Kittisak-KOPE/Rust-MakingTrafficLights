extern crate rust_gpiozero;
use rust_gpiozero::*;
use std::thread::sleep;
use std::time::Duration;

fn main(){

    // Tell the Pi which GPIO pin you are using
    let mut buzzer = Buzzer::new(15);

    loop{
        // -------------------------------
        // Make the buzzer switch on
        buzzer.on();

        // Wait for one second
        sleep(Duration::from_secs(1));

        // Make the buzzer switch off
        buzzer.off();

        // Wait for one second
        sleep(Duration::from_secs(1));
        // -------------------------------

        // Let the buzzer beep
        // buzzer.beep(1, 1);
    }
}