use gpio::GpioOut;
use std::{thread::sleep, time::Duration};

fn main() {
    // Attempt to open GPIO pin 11 for output
    let mut led_one = gpio::sysfs::SysFsGpioOutput::open(17).unwrap();
    let mut led_two = gpio::sysfs::SysFsGpioOutput::open(18).unwrap();

    led_one.set_high();
    led_two.set_high();
    sleep(Duration::from_millis(10));
    led_one.set_low();
    led_two.set_low();
}
