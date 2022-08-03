#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use panic_semihosting as _;

// NOTES:
//
// - Async tasks cannot have `#[lock_free]` resources, as they can interleve and each async
//   task can have a mutable reference stored.
// - Spawning an async task equates to it being polled once.

#[rtic::app(device = lm3s6965, dispatchers = [SSI0, UART0], peripherals = true)]
mod app {
    use cortex_m_semihosting::{debug, hprintln};
    use systick_monotonic::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = Systick<100>;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        hprintln!("init").unwrap();

        (
            Shared {},
            Local {},
            init::Monotonics(Systick::new(cx.core.SYST, 12_000_000)),
        )
    }

    #[idle]
    async fn idle(_: idle::Context) -> ! {
        hprintln!("idle");

        for i in 0..2 {
            monotonics::delay(100.millis()).await;
            hprintln!("async delay {}").ok();
        }

        debug::exit(debug::EXIT_SUCCESS);

        loop {}
    }
}
