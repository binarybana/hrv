#![no_main]
#![no_std]

use dht_sensor::*;
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{delay, pac, prelude::*, timer::Timer};
use stm_blinky_rtt as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOC peripheral
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    /////////////

    // let mut p = pac::Peripherals::take().unwrap();
    // let cp = pac::CorePeripherals::take().unwrap();
    // let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);

    // This is used by `dht-sensor` to wait for signals
    let mut delay = delay::Delay::new(cp.SYST, clocks);

    // This could be any `gpio` port
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

    // The DHT11 datasheet suggests 1 second

    // An `Output<OpenDrain>` is both `InputPin` and `OutputPin`
    // let mut pa1 = cortex_m::interrupt::free(|_cs| gpioa.pa1.into_open_drain_output(&mut gpioa.crl));
    let mut pa1 = gpioa.pa1.into_open_drain_output(&mut gpioa.crl);
    let mut pa2 = gpioa
        .pa2
        .into_push_pull_output_with_state(&mut gpioa.crl, stm32f1xx_hal::gpio::State::Low);
    defmt::info!("Starting program (with a wait)...");
    led.set_low().unwrap();
    pa1.set_low().unwrap();
    pa2.set_low().unwrap();
    delay.delay_ms(100_u16);
    pa1.set_high().unwrap();
    delay.delay_ms(900u16);
    pa2.set_high().unwrap();

    // loop {
    //     pa1.set_low().unwrap();
    //     delay.delay_us(48u8);
    //     pa1.set_high().unwrap();
    //     delay.delay_us(48u8);
    // }

    loop {
        defmt::info!("Beginning sensor read!");
        match dht11::Reading::read(&mut delay, &mut pa1) {
            Ok(dht11::Reading {
                temperature,
                relative_humidity,
            }) => defmt::info!("{:i8}°, {:u8}% RH", temperature, relative_humidity),
            Err(_e) => defmt::error!("Error encountered"),
        }
        led.toggle().unwrap();
        pa1.set_high().unwrap();
        delay.delay_ms(5000u16);
    }
    // stm_blinky_rtt::exit()
}
