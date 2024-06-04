use rppal::gpio::Gpio;
use std::{thread::sleep, time::Duration};

fn main() {
    let gpio = Gpio::new().expect("Failed to initialize GPIO");
    let mut red = gpio.get(17).expect("Failed to get GPIO pin").into_output();
    let mut green = gpio.get(18).expect("Failed to get GPIO pin").into_output();
    let mut blue = gpio.get(27).expect("Failed to get GPIO pin").into_output();

    // Configure the desired PWM frequency and duty cycle
    let frequency = 1000.0; // 1 kHz
    let period = Duration::from_secs_f64(1.0 / frequency);
    let mut duty_cycle: f64;

    loop {
        // Ramp up the duty cycle from 0% to 100%
        for i in 0..=100 {
            duty_cycle = i as f64 / 100.0;
            let high_time = period.mul_f64(duty_cycle);
            let low_time = period - high_time;

            // Set the pin high for the high time duration
            red.set_high();
            sleep(high_time);
            green.set_high();
            sleep(high_time);
            blue.set_high();
            sleep(high_time);

            // Set the pin low for the low time duration
            red.set_low();
            sleep(low_time);
            green.set_low();
            sleep(low_time);
            blue.set_low();
            sleep(low_time);
        }

        // Ramp down the duty cycle from 100% to 0%
        for i in (0..=100).rev() {
            duty_cycle = i as f64 / 100.0;
            let high_time = period.mul_f64(duty_cycle);
            let low_time = period - high_time;

            // Set the pin high for the high time duration
            red.set_high();
            sleep(high_time);
            green.set_high();
            sleep(high_time);
            blue.set_high();
            sleep(high_time);

            // Set the pin low for the low time duration
            red.set_low();
            sleep(low_time);
            green.set_low();
            sleep(low_time);
            blue.set_low();
            sleep(low_time);
        }
    }
}
