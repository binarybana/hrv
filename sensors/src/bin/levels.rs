#![no_main]
#![no_std]

use stm32f1xx_hal as _;
use stm_blinky_rtt as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("info");
    defmt::trace!("trace");
    defmt::warn!("warn");
    defmt::debug!("debug");
    defmt::error!("error");

    stm_blinky_rtt::exit()
}
