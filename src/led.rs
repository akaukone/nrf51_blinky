extern crate nrf51;
use nrf51::{GPIO, Gpio};

pub static LEDS: [Led; 4] = [Led { i: 21 }, Led { i: 22 }, Led { i: 23 }, Led { i: 24 }];

pub struct Led {
    i: u8,
}

impl Led {
    pub fn off(&self) {
        unsafe {
            (*GPIO.get()).outset.modify(|_, w| w.bits(1 << self.i));
        }
    }

    pub fn on(&self) {
        unsafe {
            (*GPIO.get()).outclr.modify(|_, w| w.bits(1 << self.i));
        }
    }
}

pub fn init(gpio: &Gpio) {
    gpio.dir
        .modify(|_, w| {
            w.pin21()
                .output()
                .pin22()
                .output()
                .pin23()
                .output()
                .pin24()
                .output()
        });

    for led in &LEDS {
        led.off();
    }
}
