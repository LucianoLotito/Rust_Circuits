use std::{thread::sleep, time::Duration};

use i2cdev::{core::I2CDevice, linux::LinuxI2CDevice};

const ROTARY_POTENTIOMETER: u16 = 0x4b;
fn main() {
    let mut dev =
        LinuxI2CDevice::new("/dev/i2c-1", ROTARY_POTENTIOMETER).expect("Failed to enable i2c");

    loop {
        println!(
            "{:?}",
            dev.smbus_read_i2c_block_data(11, 1).expect("Failed")
        );
        sleep(Duration::from_millis(500));
    }
}
