extern crate nrf51;
use nrf51::Timer0;

#[derive(Clone, Copy)]
pub struct Timer<'a>(pub &'a Timer0);


impl<'a> Timer<'a> {
    /// Initializes the timer with a periodic timeout of `frequency` Hz
    //
    // NOTE: After initialization, the timer will be in paused state.
    pub fn init(&self, frequency: u32) {
        let timer0 = self.0;
        let prescaler = match frequency {
            16000000 => 0,
            8000000 => 1,
            4000000 => 2,
            2000000 => 3,
            1000000 => 4,
            500000 => 5,
            250000 => 6,
            125000 => 7,
            62500 => 8,
            31250 => 9,
            _ => panic!("Invalid frequency: {}", frequency),
        };

        timer0
            .prescaler
            .write(|w| unsafe { w.prescaler().bits(prescaler) });

        // Configure 32-bit timer mode
        timer0.mode.write(|w| w.mode().timer());
        timer0.bitmode.write(|w| w.bitmode()._32bit());
        timer0.intenset.write(|w| w.compare0().set());
        timer0.cc0.write(|w| unsafe { w.bits(20000) });

        self.clear()
    }

    pub fn resume(&self) {
        self.0.tasks_start.write(|w| unsafe { w.bits(1) });
    }

    pub fn pause(&self) {
        self.0.tasks_stop.write(|w| unsafe { w.bits(1) });
    }

    pub fn clear(&self) {
        self.0.tasks_clear.write(|w| unsafe { w.bits(1) });
    }

    pub fn clear_event(&self) {
        self.0.events_compare0.write(|w| unsafe { w.bits(0) });
    }
}
