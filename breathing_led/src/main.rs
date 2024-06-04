use std::{thread::sleep, time::Duration};

use rppal::pwm::{Polarity, Pwm};

fn main() {
    let pwn = Pwm::with_frequency(
        rppal::pwm::Channel::Pwm0,
        1000.0,
        0.0,
        Polarity::Normal,
        false,
    )
    .expect("Failed to enable PWM port");

    loop {
        for i in 0..100 {
            let duty_cycle = i as f64 / 100.0;
            pwn.set_duty_cycle(duty_cycle).expect("Failed");
            sleep(Duration::from_millis(20));
        }

        sleep(Duration::from_secs(1));

        // Gradually decrease brightness
        for i in (0..=100).rev() {
            let duty_cycle = i as f64 / 100.0;
            pwn.set_duty_cycle(duty_cycle)
                .expect("Failed to set duty cycle");
            sleep(Duration::from_millis(20));
        }
    }
}
