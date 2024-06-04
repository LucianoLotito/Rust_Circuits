use rppal::gpio::{Gpio, Level};

fn main() {
    let gpio = Gpio::new().expect("Failed to initialize GPIO");

    let mut buzzer = gpio.get(17).expect("Failed to get pin").into_output();
    let button = gpio.get(18).expect("Failed to get pin").into_input();
    let _ = buzzer.set_pwm_frequency(2000.0, 1.0);

    loop {
        match button.read() {
            Level::High => buzzer.set_low(),
            Level::Low => buzzer.set_high(),
        };
    }
}
