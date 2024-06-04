use std::time::{Duration, Instant};

use gpio::{
    self,
    sysfs::{SysFsGpioInput, SysFsGpioOutput},
    GpioIn, GpioOut, GpioValue,
};

const CAPTURE_TIME: u8 = 50;
fn main() {
    // Attempt to open GPIO pin 11 for output
    let mut led = SysFsGpioOutput::open(17).unwrap();
    let mut button = SysFsGpioInput::open(18).unwrap();
    let mut last_change = Instant::now();
    let mut value = false;
    let mut reading = match button.read_value() {
        Ok(v) => v,
        Err(_) => panic!(),
    };
    let mut last_button_state = reading.clone();
    let mut button_state = reading.clone();

    loop {
        reading = match button.read_value() {
            Ok(v) => v,
            Err(_) => panic!(),
        };

        if reading != last_button_state {
            last_change = Instant::now();
        }
        if last_change.elapsed() > Duration::from_millis(CAPTURE_TIME as u64) {
            if reading != button_state {
                button_state = reading;

                if button_state == GpioValue::Low {
                    value = !value;
                }
            }
        }

        let _ = led.set_value(value.clone());

        last_button_state = reading.clone();
    }
}
