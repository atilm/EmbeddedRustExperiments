#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

// use cortex_m::asm;
extern crate cortex_m;
use cortex_m_rt::entry;
use rtt_target::{debug_rtt_init_print, debug_rprintln};

#[entry]
fn main() -> ! {
    //asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
    debug_rtt_init_print!(); // nop in --release
    let x = 1;
    let _y = x + x;
    debug_rprintln!("Hello, world!");

    loop {     
        // your code goes here 
    }
}
