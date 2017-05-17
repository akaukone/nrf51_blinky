#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
#[macro_use]
extern crate cortex_m_rtfm as rtfm;
extern crate nrf51;

use rtfm::{P0, T0, TMax};

tasks!(nrf51, {});

fn init(_priority: P0, _threshold: &TMax) {}

fn idle(_priority: P0, _threshold: T0) -> ! {
    loop {
        rtfm::wfi();
    }
}
