use std::{thread::sleep, time::Duration};

use gpio::{
    self,
    sysfs::{SysFsGpioInput, SysFsGpioOutput},
    GpioIn, GpioOut, GpioValue,
};

fn main() {
    // Attempt to open GPIO pin 11 for output
    let mut led = SysFsGpioOutput::open(17).unwrap();
    let mut button = SysFsGpioInput::open(18).unwrap();

    loop {
        match button.read_value() {
            Ok(v) => match v {
                GpioValue::Low => fiesta(&mut led),
                GpioValue::High => led.set_low().unwrap(),
            },
            Err(_) => {
                panic!();
            }
        }
    }
}
fn fiesta(led: &mut SysFsGpioOutput) {
    led.set_high();
    sleep(Duration::from_millis(50));
    led.set_low();
    sleep(Duration::from_millis(50));
}
