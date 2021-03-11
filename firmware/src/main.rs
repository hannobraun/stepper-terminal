#![no_main]
#![no_std]

use panic_rtt_target as _;

#[rtic::app(device = lpc8xx_hal::pac, peripherals = false)]
mod app {
    use rtt_target::{rprint, rprintln};

    #[init]
    fn init(_: init::Context) -> (init::LateResources, init::Monotonics) {
        rtt_target::rtt_init_print!();
        rprint!("Initializing... ");
        rprintln!("done.");

        (init::LateResources {}, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            rprintln!("Hello, world!");
        }
    }
}
