use std::{thread::sleep, time::Duration};

use gpio::{sysfs::SysFsGpioOutput, GpioOut};

const PINS: [u8; 10] = [17, 18, 27, 22, 23, 24, 25, 2, 3, 8];
fn main() {
    let mut active_pins: Vec<SysFsGpioOutput> = vec![];
    let mut activating_pin: SysFsGpioOutput;
    for pin in PINS {
        activating_pin = match SysFsGpioOutput::open(pin as u16) {
            Ok(p) => p,
            Err(_) => panic!(),
        };
        activating_pin.set_low();
        activating_pin.set_high();
        active_pins.push(activating_pin);
    }
    loop {
        for pin in 0..PINS.len() / 2 {
            match active_pins[pin].set_low() {
                Ok(_) => (),
                Err(_) => panic!(),
            }
            match active_pins[PINS.len() - 1 - pin].set_low() {
                Ok(_) => (),
                Err(_) => panic!(),
            }

            sleep(Duration::from_millis(50));
        }

        sleep(Duration::from_millis(100));

        //     for pin in 0..PINS.len() / 2 {
        //         match active_pins[pin].set_high() {
        //             Ok(_) => (),
        //             Err(_) => panic!(),
        //         }
        //         match active_pins[PINS.len() - 1 - pin].set_high() {
        //             Ok(_) => (),
        //             Err(_) => panic!(),
        //         }

        //         sleep(Duration::from_millis(50));
        //     }
        //     sleep(Duration::from_millis(100));
    }
}
