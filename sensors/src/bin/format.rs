#![no_main]
#![no_std]

use defmt::Format; // <- derive attribute
use stm32f1xx_hal as _;
use stm_blinky_rtt as _; // global logger + panicking-behavior + memory layout

#[derive(Format)]
struct S1<T> {
    x: u8,
    y: T,
}

#[derive(Format)]
struct S2 {
    z: u8,
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let s = S1 {
        x: 42,
        y: S2 { z: 43 },
    };
    defmt::info!("s={:?}", s);
    let x = 42;
    defmt::info!("x={:u8}", x);

    stm_blinky_rtt::exit()
}
