use embedded_time::{clock, fraction::Fraction, Clock, Instant};

pub struct SysClock;

impl Clock for SysClock {
    type T = u32;
    const SCALING_FACTOR: Fraction = Fraction::new(1, 1_000_000);

    fn try_now(&self) -> Result<Instant<Self>, clock::Error> {
        static mut TICKS: u32 = 0;
        unsafe {
            TICKS += 1;
        }
        Ok(Instant::new(unsafe { TICKS as Self::T }))
    }
}
