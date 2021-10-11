use core::{
    convert::{Infallible, TryFrom as _},
    marker::PhantomData,
};

use lpc8xx_hal::mrt;
use num_traits::Zero as _;
use stepper::{
    embedded_hal::timer::nb::CountDown,
    embedded_time::duration::{
        Microseconds, Milliseconds, Nanoseconds, Seconds,
    },
    motion_control::DelayToTicks,
};

pub type Num = fixed::FixedI64<typenum::U32>;

pub struct SecondsToMrtTicks<I>(PhantomData<I>);

impl<I> SecondsToMrtTicks<I> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<I> DelayToTicks<Num> for SecondsToMrtTicks<I>
where
    I: mrt::Trait,
{
    type Ticks = <mrt::Channel<I> as CountDown>::Time;

    type Error = Infallible;

    fn delay_to_ticks(&self, delay: Num) -> Result<Self::Ticks, Self::Error> {
        let mut ticks = mrt::Ticks::zero();

        let delay_s = delay;
        ticks += mrt::Ticks::try_from(Seconds(delay_s.int().to_num())).unwrap();

        let delay_ms = delay_s.frac() * 1000;
        ticks += mrt::Ticks::try_from(Milliseconds(delay_ms.int().to_num()))
            .unwrap();

        let delay_us = delay_ms.frac() * 1000;
        ticks += mrt::Ticks::try_from(Microseconds(delay_us.int().to_num()))
            .unwrap();

        let delay_ns = delay_us.frac() * 1000;
        ticks += mrt::Ticks::from(Nanoseconds(delay_ns.int().to_num()));

        Ok(ticks)
    }
}
