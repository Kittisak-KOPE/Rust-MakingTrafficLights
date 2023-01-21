# Raspberry Pi with Rust

## Steps

From your project directory, build your project by entering the following commands:

```
cargo check
```

Run

```
cargo run --example <fileName>
```

## Making traffic lights

### Dive into Rust

- Open the Python application IDLE and get started by testing out the button.

  ```
  extern crate rust_gpiozero;
  use rust_gpiozero::*;

  fn main() {
      // Create a button which is attached to Pin 22
      let button = Button::new(22);

      loop{
          println!("{}",button.is_active());
      }
  }
  ```

- Run the program using the command:

  ```
  cargo run --example trafficlights
  ```

- This will constantly print ==false==. When you press the button this will switch to ==true==, and when you let go it will return to ==false==. <br>
  ==button.is_active()== is a method of the ==button== instance, which provides the state of the button (pressed or not) at any given time.

- Now edit the code again and modify your ==loop== to the following:

  ```
  loop{
      if(button.is_active()){
          println!("Hello");
      }else{
          println!("Goodbye");
      }
  }
  ```

- Run the code again and you'll see "Hello" printed when the button is pressed, and "Goodbye" when the button is not pressed.

- Modify the loop again:

  ```
  loop{
      button.wait_for_press();
      println!("Pressed");
      button.wait_for_release();
      println!("Released");
  }
  ```

- When you run the code this time, nothing will happen until you press the button, when you'll see "Pressed", then when you let go you'll see "Released". This will occur each time the button is pressed, but rather than continuously printing one or the other, it only does it once per press.

### Add an LED

Now you'll add an LED into the code and use GPIO Zero to allow the button to determine when the LED is lit.

- Add a line below ==let button = Button::new(22);== to create an instance of an ==LED==:
  ```
  let mut led = LED::new(25);
  ```
- Now modify your ==loop== to turn the LED on when the button is pressed:
  ```
  loop{
      button.wait_for_press();
      led.on();
      button.wait_for_release();
      led.off();
  }
  ```
- Run your code and the LED will come on when you press the button. Hold the button down to keep the LED lit.
- Now swap the ==on== and ==off== lines to reverse the logic:
  ```
  loop{
      led.on();
      button.wait_for_press();
      led.off();
      button.wait_for_release();
  }
  ```
- Run the code and you'll see the LED stays on until the button is pressed.

### Traffic lights

You have three LEDs: red, amber, and green. Perfect for traffic lights!

- Replace your ==let mut led = LED::new(25);== line with the following:
  ```
  let mut red = LED::new(25);
  let mut amber = LED::new(8);
  let mut green = LED::new(7);
  ```
  The are three GPIO pin numbers: red, amber, and green (in that order).
- Now amend your ==loop== to control the ==LEDs==:
  ```
  loop{
      button.wait_for_press();
      // Switch LEDs on
      red.on();
      amber.on();
      green.on();
      button.wait_for_release();
      // Switch LEDs off
      red.off();
      amber.off();
      green.off();
  }
  ```

### Add a buzzer

Now you'll add your buzzer to make some noise.

- Add a line below your creation of ==button== and ==red==, ==amber== and green to add a ==Buzzer==:
  ```
  let mut buzzer = Buzzer::new(15);
  ```
- ==Buzzer== works exactly like ==LED==, so try adding a ==buzzer.on();== and ==buzzer.off();== into your loop:
  ```
  loop{
      button.wait_for_press();
      // Switch LEDs on
      red.on();
      amber.on();
      green.on();
      // Switch Buzzer on
      buzzer.on();
      button.wait_for_release();
      // Switch LEDs off
      red.off();
      amber.off();
      green.off();
      // Switch Buzzer off
      buzzer.off();
  }
  ```

### Traffic lights sequence

With traffic light LEDs, a button and a buzzer, you can create your own traffic lights sequence, complete with pedestrian crossing!

- At the top, add the following:
  ```
  use std::thread::sleep;
  use std::time::Duration;
  ```
- Modify your loop to perform an automated sequence of LEDs being lit:
  ```
  loop{
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
  ```
- Add a ==wait_for_press== so that pressing the button initiates the sequence:
  ```
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
  ```
  Try some more sequences of your own.
- Now try creating the full traffic lights sequence:
  - Green on
  - Amber on
  - Red on
  - Red and amber on
  - Green on
    Be sure to turn the correct lights on and off at the right time, and make sure you use sleep to time the sequence perfectly.
- Try adding the button for a pedestrian crossing. The button should move the lights to red (not immediately), and give the pedestrians time to cross before moving back to green until the button is pressed again.
- Now try adding a buzzer to beep quickly to indicate that it is safe to cross, for the benefit of visually impaired pedestrians.
