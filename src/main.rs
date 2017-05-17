#![feature(const_fn)]
#![feature(used)]
#![no_std]

extern crate cortex_m_rt;

#[macro_use]
extern crate cortex_m_rtfm as rtfm;
extern crate nrf51;

pub mod led;
pub mod timer;

use rtfm::{Local, P0, P2, T0, T2, TMax};
use led::LEDS;
use nrf51::interrupt::Timer0;

const FREQUENCY: u32 = 31250; // Hz

peripherals!(nrf51, {
    CLOCK: Peripheral {
        register_block: Clock,
        ceiling: C0,
    },
    GPIO: Peripheral {
        register_block: Gpio,
        ceiling: C1,
    },
    TIMER0: Peripheral {
        register_block: Timer0,
        ceiling: C2,
    },
});

fn init(ref priority: P0, threshold: &TMax) {
    let clock = CLOCK.access(priority, threshold);
    clock.tasks_hfclkstart.write(|w| unsafe { w.bits(1) });

    let gpio = GPIO.access(priority, threshold);
    let timer0 = TIMER0.access(priority, threshold);
    let timer = timer::Timer(&timer0);

    led::init(&gpio);

    timer.init(FREQUENCY);
    timer.resume();
}

fn idle(_priority: P0, _threshold: T0) -> ! {
    loop {
        rtfm::wfi();
    }
}

tasks!(nrf51, {
    periodic: Task {
        interrupt: Timer0,
        priority: P2,
        enabled: true,
    },
});

fn periodic(mut task: Timer0, ref priority: P2, ref threshold: T2) {
    static STATE: Local<bool, Timer0> = Local::new(false);

    let timer0 = TIMER0.access(priority, threshold);
    let timer = timer::Timer(&timer0);
    timer.clear();
    timer.clear_event();

    let state = STATE.borrow_mut(&mut task);

    *state = !*state;
    if *state {
        LEDS[0].on();
        LEDS[2].on();
        LEDS[1].off();
        LEDS[3].off();
    } else {
        LEDS[0].off();
        LEDS[2].off();
        LEDS[1].on();
        LEDS[3].on();
    }
}
