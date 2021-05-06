#![no_main]
#![no_std]

mod num;

use panic_rtt_target as _;

#[rtic::app(device = lpc8xx_hal::pac, peripherals = false)]
mod app {
    use heapless::{
        spsc::{self, Queue},
        Vec,
    };
    use lpc8xx_hal::{
        delay::Delay,
        gpio::{self, direction::Output, GpioPin},
        mrt, nb,
        pac::USART0,
        pins::{PIO0_16, PIO0_17, PIO0_18, PIO0_19, PIO0_20},
        prelude::*,
        usart::{
            self,
            state::{AsyncMode, Enabled},
        },
    };
    use protocol::Command;
    use rtt_target::{rprint, rprintln};
    use stepper::{
        drivers::stspin220::STSPIN220, motion_control::SoftwareMotionControl,
        ramp_maker, step_mode::StepMode256, Direction, Stepper,
    };

    use crate::num::{Num, SecondsToMrtTicks};

    #[resources]
    struct Resources {
        #[task_local]
        #[init(Queue::new())]
        usart_queue: Queue<u8, 128>,

        #[lock_free]
        delay: Delay,

        #[lock_free]
        usart: usart::Rx<USART0, Enabled<u8, AsyncMode>>,

        #[lock_free]
        usart_queue_prod: spsc::Producer<'static, u8, 128>,

        #[lock_free]
        usart_queue_cons: spsc::Consumer<'static, u8, 128>,

        #[lock_free]
        stepper: Stepper<
            SoftwareMotionControl<
                STSPIN220<
                    (),
                    GpioPin<PIO0_16, Output>,
                    GpioPin<PIO0_17, Output>,
                    GpioPin<PIO0_18, Output>,
                    GpioPin<PIO0_19, Output>,
                    GpioPin<PIO0_20, Output>,
                >,
                mrt::Channel<mrt::MRT0>,
                ramp_maker::Trapezoidal<Num>,
                SecondsToMrtTicks<mrt::MRT0>,
            >,
        >,
    }

    #[init(resources = [usart_queue])]
    fn init(cx: init::Context) -> (init::LateResources, init::Monotonics) {
        rtt_target::rtt_init_print!();
        rprint!("Initializing... ");

        let p = lpc8xx_hal::Peripherals::take().unwrap();

        let swm = p.SWM.split();
        let mut syscon = p.SYSCON.split();

        let mut swm_handle = swm.handle.enable(&mut syscon.handle);

        let delay = Delay::new(cx.core.SYST);

        let (u0_rxd, _) = swm
            .movable_functions
            .u0_rxd
            .assign(p.pins.pio0_24.into_swm_pin(), &mut swm_handle);
        let (u0_txd, _) = swm
            .movable_functions
            .u0_txd
            .assign(p.pins.pio0_25.into_swm_pin(), &mut swm_handle);

        let mut usart = p.USART0.enable_async(
            &usart::Clock::new_with_baudrate(115200),
            &mut syscon.handle,
            u0_rxd,
            u0_txd,
            usart::Settings::default(),
        );
        usart.enable_interrupts(usart::Interrupts {
            RXRDY: true,
            ..usart::Interrupts::default()
        });

        let (usart_queue_prod, usart_queue_cons) =
            cx.resources.usart_queue.split();

        let gpio = p.GPIO.enable(&mut syscon.handle);
        let mrt = p.MRT0.split(&mut syscon.handle);

        let standby_reset = p
            .pins
            .pio0_16
            .into_output_pin(gpio.tokens.pio0_16, gpio::Level::Low);
        let mode1 = p
            .pins
            .pio0_17
            .into_output_pin(gpio.tokens.pio0_17, gpio::Level::Low);
        let mode2 = p
            .pins
            .pio0_18
            .into_output_pin(gpio.tokens.pio0_18, gpio::Level::Low);
        let step_mode3 = p
            .pins
            .pio0_19
            .into_output_pin(gpio.tokens.pio0_19, gpio::Level::Low);
        let dir_mode4 = p
            .pins
            .pio0_20
            .into_output_pin(gpio.tokens.pio0_20, gpio::Level::Low);

        let mut timer = mrt.mrt0;

        let target_accel = Num::from_num(1000); // steps per second^2
        let profile = ramp_maker::Trapezoidal::new(target_accel);

        let stepper = Stepper::from_driver(STSPIN220::new())
            .enable_step_control(step_mode3)
            .enable_direction_control(dir_mode4, Direction::Forward, &mut timer)
            .unwrap()
            .enable_step_mode_control(
                (standby_reset, mode1, mode2),
                StepMode256::Full,
                &mut timer,
            )
            .unwrap()
            .enable_motion_control((timer, profile, SecondsToMrtTicks::new()));

        rprintln!("done.");

        (
            init::LateResources {
                delay,
                usart: usart.rx,
                usart_queue_prod,
                usart_queue_cons,
                stepper,
            },
            init::Monotonics(),
        )
    }

    #[idle(resources = [delay, usart_queue_cons, stepper])]
    fn idle(cx: idle::Context) -> ! {
        let delay = cx.resources.delay;
        let usart = cx.resources.usart_queue_cons;
        let stepper = cx.resources.stepper;

        let mut buf: Vec<_, 128> = Vec::new();

        loop {
            while let Some(word) = usart.dequeue() {
                buf.push(word).unwrap();

                if word == 0 {
                    let command: Command =
                        postcard::from_bytes_cobs(&mut buf).unwrap();
                    buf.clear();

                    rprintln!("Executing command: {:?}", command);

                    match command {
                        Command::Step(step) => {
                            // There are two sources for panics in the
                            // following code:
                            // - Ongoing motion: The driver only provides
                            //   support for `set_direction` and `step`, if no
                            //   motion is currently ongoing.
                            // - Errors from the `OutputPin` or
                            //   `timer::CountDown` implementations.
                            //
                            // Since we currently don't use the motion
                            // control API, and our implementations of those
                            // traits are infallible, neither of those can
                            // actually happen.

                            match step.direction {
                                protocol::Direction::Forward => {
                                    stepper
                                        .driver_mut()
                                        .set_direction(Direction::Forward)
                                        .expect("Ongoing motion")
                                        .wait()
                                        .expect("Pin or timer error");
                                }
                                protocol::Direction::Backward => {
                                    stepper
                                        .driver_mut()
                                        .set_direction(Direction::Backward)
                                        .expect("Ongoing motion")
                                        .wait()
                                        .expect("Pin or timer error");
                                }
                            }

                            for _ in 0..step.steps {
                                stepper
                                    .driver_mut()
                                    .step()
                                    .expect("Ongoing motion")
                                    .wait()
                                    .expect("Pin or timer error");

                                delay.delay_ms(step.delay);
                            }
                        }
                        Command::MoveTo(move_to) => {
                            // There are two sources of panics in the following
                            // code:
                            // 1. Errors from the `OutputPin` or `CountDown`
                            //    implementations.
                            // 2. Errors while converting from nanoseconds to
                            //    timer ticks, or seconds to timer ticks.
                            //
                            // All our trait implementations are infallible, so
                            // 1. shouldn't happen. 2. also shouldn't happen, as
                            // we're using reasonable values that should be well
                            // within the bounds of the data types. Detailed
                            // analysis hasn't happened though, so panics for
                            // that reason can't be rules out.
                            stepper
                                .move_to_position(
                                    Num::from_num(move_to.max_speed),
                                    move_to.target_step,
                                )
                                .wait()
                                .unwrap();
                        }
                    }
                }
            }
        }
    }

    #[task(binds = USART0, resources = [usart, usart_queue_prod])]
    fn usart0(cx: usart0::Context) {
        let usart = cx.resources.usart;
        let queue = cx.resources.usart_queue_prod;

        loop {
            match usart.read() {
                Ok(word) => queue.enqueue(word).unwrap(),
                Err(nb::Error::WouldBlock) => break,
                Err(nb::Error::Other(err)) => panic!("{:?}", err),
            }
        }
    }
}
