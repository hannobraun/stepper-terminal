#![no_main]
#![no_std]

use panic_rtt_target as _;

#[rtic::app(device = lpc8xx_hal::pac, peripherals = false)]
mod app {
    use heapless::{
        consts::U128,
        spsc::{self, Queue},
        Vec,
    };
    use lpc8xx_hal::{
        nb,
        pac::USART0,
        prelude::*,
        usart::{
            self,
            state::{AsyncMode, Enabled},
        },
    };
    use protocol::Command;
    use rtt_target::{rprint, rprintln};

    #[resources]
    struct Resources {
        #[task_local]
        #[init(Queue(heapless::i::Queue::new()))]
        usart_queue: Queue<u8, U128>,

        #[lock_free]
        usart: usart::Rx<USART0, Enabled<u8, AsyncMode>>,

        #[lock_free]
        usart_queue_prod: spsc::Producer<'static, u8, U128>,

        #[lock_free]
        usart_queue_cons: spsc::Consumer<'static, u8, U128>,
    }

    #[init(resources = [usart_queue])]
    fn init(cx: init::Context) -> (init::LateResources, init::Monotonics) {
        rtt_target::rtt_init_print!();
        rprint!("Initializing... ");

        let p = lpc8xx_hal::Peripherals::take().unwrap();

        let swm = p.SWM.split();
        let mut syscon = p.SYSCON.split();

        let mut swm_handle = swm.handle.enable(&mut syscon.handle);

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

        rprintln!("done.");

        (
            init::LateResources {
                usart: usart.rx,
                usart_queue_prod,
                usart_queue_cons,
            },
            init::Monotonics(),
        )
    }

    #[idle(resources = [usart_queue_cons])]
    fn idle(cx: idle::Context) -> ! {
        let usart = cx.resources.usart_queue_cons;

        let mut buf: Vec<_, U128> = Vec::new();

        loop {
            while let Some(word) = usart.dequeue() {
                buf.push(word).unwrap();

                if word == 0 {
                    let command: Command =
                        postcard::from_bytes_cobs(&mut buf).unwrap();
                    buf.clear();

                    rprintln!("{:?}", command);
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
