#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate cortex_m_rtfm as rtfm;
extern crate stm32f103xx;

use rtfm::app;

app! {
    //~^ error no associated item named `EXTI33` found for type
    //~| error no associated item named `EXTI33` found for type
    device: stm32f103xx,

    tasks: {
        // ERROR this interrupt doesn't exist
        EXTI33: {},
    },
}

fn init(_p: init::Peripherals) {}

fn idle() -> ! {
    loop {}
}
