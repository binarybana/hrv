use crate::Config;

use anyhow::Result;
use rppal::gpio::Gpio;

use std::sync::{Arc, Mutex};

pub fn setup_hardware(config: Arc<Mutex<Config>>) -> Result<()> {
    let my_info = rppal::system::DeviceInfo::new()?;
    dbg!(my_info);

    std::thread::spawn(move || {
        let gpio = Gpio::new().expect("Couldn't get GPIO handle");
        let mut pin1 = gpio
            .get(14)
            .expect("Couldn't get GPIO pin 12")
            .into_output();

        let mut pin2 = gpio
            .get(15)
            .expect("Couldn't get GPIO pin 12")
            .into_output();

        let mut fan1_desired_pwm;
        let mut fan2_desired_pwm;
        loop {
            std::thread::sleep(std::time::Duration::from_millis(1000));

            {
                let config = config
                    .lock()
                    .expect("Couldn't grab config lock in hw thread");
                fan1_desired_pwm = config.fan1_speed;
                fan2_desired_pwm = config.fan2_speed;
            }
            if fan1_desired_pwm > 0.0 {
                pin2.set_pwm_frequency(25_000.0, fan1_desired_pwm)
                    .expect("Couldn't initialize fan1 PWM");
                println!("Setting duty cycle to {}", fan1_desired_pwm);
            } else if fan1_desired_pwm > 0.95 {
                pin1.clear_pwm().expect("Couldn't clear pwm on pin1");
                pin1.set_high();
            } else {
                pin1.clear_pwm().expect("Couldn't clear pwm on pin1");
                pin1.set_low();
            }

            if fan2_desired_pwm > 0.0 {
                pin2.set_pwm_frequency(25_000.0, fan2_desired_pwm)
                    .expect("Couldn't initialize fan2 PWM");
                println!("Setting duty cycle to {}", fan2_desired_pwm);
            } else if fan2_desired_pwm > 0.95 {
                pin2.clear_pwm().expect("Couldn't clear pwm on pin2");
                pin2.set_high();
            } else {
                pin2.clear_pwm().expect("Couldn't clear pwm on pin2");
                pin2.set_low();
            }
        }
    });
    Ok(())
}
